//! Cyrillic alphabet operations for Woflang.
//!
//! Provides Russian Cyrillic letter lookup, quiz, and transliteration tools.
//! Uses the COMPLETE embedded database of the 33-letter Russian alphabet.

use std::collections::HashMap;
use std::sync::OnceLock;
use serde::Deserialize;
use woflang_core::{InterpreterContext, WofValue};
use woflang_runtime::Interpreter;

/// Embed the FULL cyrillic database at compile time.
const CYRILLIC_JSON: &str = include_str!("../../data/cyrillic_database.json");

/// JSON structure for deserialization.
#[derive(Debug, Deserialize)]
struct CyrillicDatabase {
    metadata: CyrillicMetadata,
    letters: Vec<CyrillicJson>,
}

#[derive(Debug, Deserialize)]
struct CyrillicMetadata {
    description: String,
    total_letters: u32,
    groups: HashMap<String, u32>,
}

#[derive(Debug, Deserialize, Clone)]
struct CyrillicJson {
    letter: String,
    lower: String,
    name_en: String,
    translit: String,
    phonetic: String,
    example_native: String,
    example_translit: String,
    example_en: String,
    group: String,
}

/// Runtime cyrillic entry.
#[derive(Debug, Clone)]
pub struct CyrillicEntry {
    pub letter: String,
    pub lower: String,
    pub name_en: String,
    pub translit: String,
    pub phonetic: String,
    pub example_native: String,
    pub example_translit: String,
    pub example_en: String,
    pub group: String,
}

impl From<CyrillicJson> for CyrillicEntry {
    fn from(j: CyrillicJson) -> Self {
        Self {
            letter: j.letter,
            lower: j.lower,
            name_en: j.name_en,
            translit: j.translit,
            phonetic: j.phonetic,
            example_native: j.example_native,
            example_translit: j.example_translit,
            example_en: j.example_en,
            group: j.group,
        }
    }
}

impl CyrillicEntry {
    /// Format as pipe-delimited string (matches C++ format).
    fn to_pipe_string(&self) -> String {
        format!("{}|{}|{}|{}|{}|{}|{}|{}|{}", 
            self.letter, self.lower, self.name_en, self.translit, self.phonetic,
            self.example_native, self.example_translit, self.example_en, self.group)
    }
}

/// Loaded cyrillic database.
struct CyrillicDb {
    entries: Vec<CyrillicEntry>,
    by_letter: HashMap<String, usize>,
    by_translit: HashMap<String, usize>,
    metadata: CyrillicMetadata,
}

impl CyrillicDb {
    fn load() -> Result<Self, String> {
        let db: CyrillicDatabase = serde_json::from_str(CYRILLIC_JSON)
            .map_err(|e| format!("Failed to parse cyrillic JSON: {}", e))?;
        
        let mut entries = Vec::new();
        let mut by_letter = HashMap::new();
        let mut by_translit = HashMap::new();
        
        for letter_json in db.letters {
            let idx = entries.len();
            let entry = CyrillicEntry::from(letter_json);
            
            // Index by upper and lower case
            by_letter.insert(entry.letter.clone(), idx);
            if !entry.lower.is_empty() {
                by_letter.insert(entry.lower.clone(), idx);
            }
            
            // Index by transliteration (skip special cases)
            if !entry.translit.is_empty() 
               && entry.translit != "hard_sign" 
               && entry.translit != "soft_sign" {
                by_translit.insert(entry.translit.clone(), idx);
            }
            
            entries.push(entry);
        }
        
        Ok(Self {
            entries,
            by_letter,
            by_translit,
            metadata: db.metadata,
        })
    }
}

/// Get the cyrillic database (lazy initialization).
fn get_db() -> &'static CyrillicDb {
    static DB: OnceLock<CyrillicDb> = OnceLock::new();
    DB.get_or_init(|| {
        CyrillicDb::load().expect("Failed to load embedded cyrillic database")
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

/// Register cyrillic operations.
pub fn register(interp: &mut Interpreter) {
    // ═══════════════════════════════════════════════════════════════
    // CYRILLIC LOOKUP
    // ═══════════════════════════════════════════════════════════════
    
    // Look up by letter OR transliteration
    // Stack: key → info_string
    interp.register("cyrillic_info", |interp| {
        let key = interp.stack_mut().pop()?.as_string()?;
        let db = get_db();
        
        // Try by letter first
        if let Some(&idx) = db.by_letter.get(&key) {
            let entry = &db.entries[idx];
            interp.stack_mut().push(WofValue::string(entry.to_pipe_string()));
            return Ok(());
        }
        
        // Try by transliteration
        if let Some(&idx) = db.by_translit.get(&key) {
            let entry = &db.entries[idx];
            interp.stack_mut().push(WofValue::string(entry.to_pipe_string()));
            return Ok(());
        }
        
        interp.stack_mut().push(WofValue::string(format!("!NOT_FOUND|{}|||||||", key)));
        Ok(())
    });

    // Random letter (optionally filtered by group)
    // Stack: [group_filter] → info_string
    interp.register("cyrillic_random", |interp| {
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
        
        let candidates: Vec<&CyrillicEntry> = if let Some(f) = &filter {
            db.entries.iter().filter(|e| e.group == *f).collect()
        } else {
            db.entries.iter().collect()
        };
        
        if candidates.is_empty() {
            interp.stack_mut().push(WofValue::string(
                format!("!NO_MATCH|{}|||||||", filter.unwrap_or_default())
            ));
            return Ok(());
        }
        
        let idx = random_index(candidates.len());
        interp.stack_mut().push(WofValue::string(candidates[idx].to_pipe_string()));
        Ok(())
    });

    // Get summary of groups/metadata
    // Stack: → summary_string
    interp.register("cyrillic_groups", |interp| {
        let db = get_db();
        
        let groups_str = db.metadata.groups.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", ");
        
        let summary = format!(
            "Cyrillic DB: {} (total letters: {}). Groups: {}",
            db.metadata.description,
            db.metadata.total_letters,
            groups_str
        );
        interp.stack_mut().push(WofValue::string(summary));
        Ok(())
    });

    // Get count of letters in database
    // Stack: → count
    interp.register("cyrillic_count", |interp| {
        let db = get_db();
        interp.stack_mut().push(WofValue::integer(db.entries.len() as i64));
        Ok(())
    });

    // Quiz: push question and expected answer
    // Stack: → question answer
    interp.register("cyrillic_quiz", |interp| {
        let db = get_db();
        
        if db.entries.is_empty() {
            interp.stack_mut().push(WofValue::string("No letters loaded".to_string()));
            interp.stack_mut().push(WofValue::string(String::new()));
            return Ok(());
        }
        
        let idx = random_index(db.entries.len());
        let entry = &db.entries[idx];
        
        let question = format!(
            "What is the sound / transliteration of letter '{}' (example: {} = {})?",
            entry.letter, entry.example_native, entry.example_en
        );
        
        interp.stack_mut().push(WofValue::string(question));
        interp.stack_mut().push(WofValue::string(entry.translit.clone()));
        Ok(())
    });

    // Reverse quiz: given transliteration, what's the letter?
    // Stack: → question answer
    interp.register("cyrillic_quiz_reverse", |interp| {
        let db = get_db();
        
        // Filter out hard/soft signs for reverse quiz
        let candidates: Vec<_> = db.entries.iter()
            .filter(|e| !e.translit.is_empty() 
                       && e.translit != "hard_sign" 
                       && e.translit != "soft_sign")
            .collect();
        
        if candidates.is_empty() {
            interp.stack_mut().push(WofValue::string("No letters loaded".to_string()));
            interp.stack_mut().push(WofValue::string(String::new()));
            return Ok(());
        }
        
        let idx = random_index(candidates.len());
        let entry = candidates[idx];
        
        let question = format!(
            "What Cyrillic letter has the sound '{}'?",
            entry.translit
        );
        
        interp.stack_mut().push(WofValue::string(question));
        interp.stack_mut().push(WofValue::string(entry.letter.clone()));
        Ok(())
    });

    // ═══════════════════════════════════════════════════════════════
    // TRANSLITERATION
    // ═══════════════════════════════════════════════════════════════
    
    // Transliterate Cyrillic text to Latin
    // Stack: cyrillic_text → latin_text
    interp.register("translit_ru", |interp| {
        let text = interp.stack_mut().pop()?.as_string()?;
        let db = get_db();
        
        let result: String = text.chars().map(|c| {
            let s = c.to_string();
            if let Some(&idx) = db.by_letter.get(&s) {
                let entry = &db.entries[idx];
                // Handle special cases
                match entry.translit.as_str() {
                    "hard_sign" | "soft_sign" => String::new(),
                    t => t.to_string(),
                }
            } else {
                s
            }
        }).collect();
        
        interp.stack_mut().push(WofValue::string(result));
        Ok(())
    });

    // Check if character is Cyrillic
    // Stack: char → 1|0
    interp.register("is_cyrillic?", |interp| {
        let text = interp.stack_mut().pop()?.as_string()?;
        let is_cyr = text.chars().next().map(|c| {
            matches!(c, '\u{0400}'..='\u{04FF}')
        }).unwrap_or(false);
        interp.stack_mut().push(WofValue::integer(if is_cyr { 1 } else { 0 }));
        Ok(())
    });

    // Get all vowels
    // Stack: → vowels_string
    interp.register("cyrillic_vowels", |interp| {
        let db = get_db();
        let vowel_letters = ["А", "Е", "Ё", "И", "О", "У", "Ы", "Э", "Ю", "Я"];
        let vowels: Vec<_> = db.entries.iter()
            .filter(|e| vowel_letters.contains(&e.letter.as_str()))
            .map(|e| e.letter.as_str())
            .collect();
        interp.stack_mut().push(WofValue::string(vowels.join(" ")));
        Ok(())
    });

    // Get all consonants
    // Stack: → consonants_string
    interp.register("cyrillic_consonants", |interp| {
        let db = get_db();
        let vowel_letters = ["А", "Е", "Ё", "И", "О", "У", "Ы", "Э", "Ю", "Я", "Ъ", "Ь"];
        let consonants: Vec<_> = db.entries.iter()
            .filter(|e| !vowel_letters.contains(&e.letter.as_str()))
            .map(|e| e.letter.as_str())
            .collect();
        interp.stack_mut().push(WofValue::string(consonants.join(" ")));
        Ok(())
    });

    // List all letters with their transliterations
    // Stack: → formatted_list
    interp.register("cyrillic_list", |interp| {
        let db = get_db();
        let list: Vec<String> = db.entries.iter()
            .map(|e| format!("{}/{} = {}", e.letter, e.lower, e.translit))
            .collect();
        interp.stack_mut().push(WofValue::string(list.join("\n")));
        Ok(())
    });
}
