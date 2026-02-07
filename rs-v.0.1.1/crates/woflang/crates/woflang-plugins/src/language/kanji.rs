//! Kanji learning operations for Woflang.
//!
//! Provides kanji lookup, search, random selection, and quiz functionality.
//! Uses the COMPLETE embedded database of 374 JLPT kanji (N5-N2).

use std::collections::HashMap;
use std::sync::OnceLock;
use serde::Deserialize;
use woflang_core::{WofError, WofValue, InterpreterContext};
use woflang_runtime::Interpreter;

/// Embed the FULL kanji database at compile time.
const KANJI_JSON: &str = include_str!("../data/kanji_database.json");

/// JSON structure for deserialization.
#[derive(Debug, Deserialize)]
struct KanjiDatabase {
    metadata: KanjiMetadata,
    kanji_by_level: HashMap<String, Vec<KanjiJson>>,
}

#[derive(Debug, Deserialize)]
struct KanjiMetadata {
    total_kanji: u32,
    levels: HashMap<String, u32>,
    description: String,
}

#[derive(Debug, Deserialize, Clone)]
struct KanjiJson {
    kanji: String,
    onyomi: String,
    romaji: String,
    meaning: String,
    example: String,
}

/// Runtime kanji entry with level attached.
#[derive(Debug, Clone)]
pub struct KanjiEntry {
    pub kanji: String,
    pub onyomi: String,
    pub romaji: String,
    pub meaning: String,
    pub example: String,
    pub level: String,
}

impl KanjiEntry {
    /// Format as pipe-delimited string (matches C++ format).
    fn to_pipe_string(&self) -> String {
        format!("{}|{}|{}|{}|{}|{}", 
            self.kanji, self.onyomi, self.romaji, self.meaning, self.example, self.level)
    }
}

/// Loaded kanji database.
struct KanjiDb {
    entries: Vec<KanjiEntry>,
    by_kanji: HashMap<String, usize>,
    metadata: KanjiMetadata,
}

impl KanjiDb {
    fn load() -> Result<Self, String> {
        let db: KanjiDatabase = serde_json::from_str(KANJI_JSON)
            .map_err(|e| format!("Failed to parse kanji JSON: {}", e))?;
        
        let mut entries = Vec::new();
        let mut by_kanji = HashMap::new();
        
        for (level_name, kanji_list) in &db.kanji_by_level {
            // Extract base level (N5, N4, etc.) from level name like "N5_BASIC"
            let level = level_name.split('_').next().unwrap_or(level_name);
            
            for k in kanji_list {
                let idx = entries.len();
                let entry = KanjiEntry {
                    kanji: k.kanji.clone(),
                    onyomi: k.onyomi.clone(),
                    romaji: k.romaji.clone(),
                    meaning: k.meaning.clone(),
                    example: k.example.clone(),
                    level: level.to_string(),
                };
                by_kanji.insert(k.kanji.clone(), idx);
                entries.push(entry);
            }
        }
        
        Ok(Self {
            entries,
            by_kanji,
            metadata: db.metadata,
        })
    }
}

/// Get the kanji database (lazy initialization).
fn get_db() -> &'static KanjiDb {
    static DB: OnceLock<KanjiDb> = OnceLock::new();
    DB.get_or_init(|| {
        KanjiDb::load().expect("Failed to load embedded kanji database")
    })
}

/// Simple pseudo-random index based on time.
fn random_index(max: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    (nanos as usize) % max
}

/// Register kanji operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // KANJI LOOKUP
    // ═══════════════════════════════════════════════════════════════
    
    // Look up kanji by character
    // Stack: kanji_char → info_string
    interp.register("kanji_info", |interp| {
        let key = interp.stack_mut().pop()?.as_string()?;
        let db = get_db();
        
        match db.by_kanji.get(&key) {
            Some(&idx) => {
                let entry = &db.entries[idx];
                interp.stack_mut().push(WofValue::string(entry.to_pipe_string()));
            }
            None => {
                interp.stack_mut().push(WofValue::string(format!("{}|!NOT_FOUND||||", key)));
            }
        }
        Ok(())
    });

    // Search by meaning (case-insensitive substring)
    // Stack: query → results... (multiple strings pushed)
    interp.register("kanji_search_meaning", |interp| {
        let query = interp.stack_mut().pop()?.as_string()?.to_lowercase();
        let db = get_db();
        
        if query.is_empty() {
            interp.stack_mut().push(WofValue::string("!NO_RESULTS|||||".to_string()));
            return Ok(());
        }
        
        let mut found = false;
        for entry in &db.entries {
            if entry.meaning.to_lowercase().contains(&query) {
                interp.stack_mut().push(WofValue::string(entry.to_pipe_string()));
                found = true;
            }
        }
        
        if !found {
            interp.stack_mut().push(WofValue::string("!NO_RESULTS|||||".to_string()));
        }
        Ok(())
    });

    // Search by reading (onyomi/romaji, case-insensitive)
    // Stack: query → results...
    interp.register("kanji_search_reading", |interp| {
        let query = interp.stack_mut().pop()?.as_string()?.to_lowercase();
        let db = get_db();
        
        if query.is_empty() {
            interp.stack_mut().push(WofValue::string("!NO_RESULTS|||||".to_string()));
            return Ok(());
        }
        
        let mut found = false;
        for entry in &db.entries {
            if entry.onyomi.to_lowercase().contains(&query) 
               || entry.romaji.to_lowercase().contains(&query) {
                interp.stack_mut().push(WofValue::string(entry.to_pipe_string()));
                found = true;
            }
        }
        
        if !found {
            interp.stack_mut().push(WofValue::string("!NO_RESULTS|||||".to_string()));
        }
        Ok(())
    });

    // Random kanji (optionally filtered by level)
    // Stack: [level_filter] → info_string
    interp.register("kanji_random", |interp| {
        let filter = if !interp.stack().is_empty() {
            match interp.stack().peek() {
                Ok(v) if v.is_string() => {
                    let f = interp.stack_mut().pop()?.as_string()?;
                    Some(f)
                }
                _ => None
            }
        } else {
            None
        };
        
        let db = get_db();
        
        let candidates: Vec<&KanjiEntry> = if let Some(f) = &filter {
            db.entries.iter().filter(|e| e.level.starts_with(f.as_str())).collect()
        } else {
            db.entries.iter().collect()
        };
        
        if candidates.is_empty() {
            interp.stack_mut().push(WofValue::string(
                format!("!NO_MATCH|Filter: {}||||", filter.unwrap_or_default())
            ));
            return Ok(());
        }
        
        let idx = random_index(candidates.len());
        interp.stack_mut().push(WofValue::string(candidates[idx].to_pipe_string()));
        Ok(())
    });

    // Get summary of available levels
    // Stack: → summary_string
    interp.register("kanji_levels", |interp| {
        let db = get_db();
        
        let mut level_counts: Vec<_> = db.metadata.levels.iter().collect();
        level_counts.sort_by_key(|(k, _)| *k);
        
        let levels_str = level_counts.iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join(", ");
        
        let summary = format!(
            "Kanji DB summary: Total Kanji: {}. {}. Levels: {}",
            db.metadata.total_kanji,
            db.metadata.description,
            levels_str
        );
        interp.stack_mut().push(WofValue::string(summary));
        Ok(())
    });

    // Get count of kanji in database
    // Stack: → count
    interp.register("kanji_count", |interp| {
        let db = get_db();
        interp.stack_mut().push(WofValue::integer(db.entries.len() as i64));
        Ok(())
    });

    // Get all kanji for a specific level
    // Stack: level → count (pushes all matching kanji)
    interp.register("kanji_by_level", |interp| {
        let level = interp.stack_mut().pop()?.as_string()?;
        let db = get_db();
        
        let mut count = 0;
        for entry in &db.entries {
            if entry.level == level {
                interp.stack_mut().push(WofValue::string(entry.to_pipe_string()));
                count += 1;
            }
        }
        
        interp.stack_mut().push(WofValue::integer(count));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // QUIZ MODE
    // ═══════════════════════════════════════════════════════════════
    
    // Quiz: push question and expected answer (meaning quiz)
    // Stack: → question answer
    interp.register("kanji_quiz", |interp| {
        let db = get_db();
        
        if db.entries.is_empty() {
            interp.stack_mut().push(WofValue::string("No kanji loaded".to_string()));
            interp.stack_mut().push(WofValue::string(String::new()));
            return Ok(());
        }
        
        let idx = random_index(db.entries.len());
        let entry = &db.entries[idx];
        
        let question = format!(
            "What is the meaning of '{}' ({})?",
            entry.kanji, entry.onyomi
        );
        
        interp.stack_mut().push(WofValue::string(question));
        interp.stack_mut().push(WofValue::string(entry.meaning.clone()));
        Ok(())
    });

    // Reverse quiz: given meaning, what's the kanji?
    // Stack: → question answer
    interp.register("kanji_quiz_reverse", |interp| {
        let db = get_db();
        
        if db.entries.is_empty() {
            interp.stack_mut().push(WofValue::string("No kanji loaded".to_string()));
            interp.stack_mut().push(WofValue::string(String::new()));
            return Ok(());
        }
        
        let idx = random_index(db.entries.len());
        let entry = &db.entries[idx];
        
        let question = format!("What kanji means '{}'?", entry.meaning);
        
        interp.stack_mut().push(WofValue::string(question));
        interp.stack_mut().push(WofValue::string(entry.kanji.clone()));
        Ok(())
    });

    // Reading quiz: given kanji, what's the reading?
    // Stack: → question answer
    interp.register("kanji_quiz_reading", |interp| {
        let db = get_db();
        
        if db.entries.is_empty() {
            interp.stack_mut().push(WofValue::string("No kanji loaded".to_string()));
            interp.stack_mut().push(WofValue::string(String::new()));
            return Ok(());
        }
        
        let idx = random_index(db.entries.len());
        let entry = &db.entries[idx];
        
        let question = format!(
            "What is the reading (romaji) of '{}' (meaning: {})?",
            entry.kanji, entry.meaning
        );
        
        interp.stack_mut().push(WofValue::string(question));
        interp.stack_mut().push(WofValue::string(entry.romaji.clone()));
        Ok(())
    });
}
