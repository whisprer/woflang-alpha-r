//! Music theory operations for Woflang.
//!
//! This plugin focuses on theory and pattern generation, not audio I/O.
//! Provides scales, chords, intervals, rhythms, and frequency calculations.
//!
//! ## Operations
//!
//! - `scale_info` - Get scale notes (root mode → description)
//! - `build_scale` - Build a scale (root scale_type → description)
//! - `chord_tones` - Get chord tones (root chord_type → description)
//! - `interval_semitones` - Get interval distance (note1 note2 → semitones)
//! - `interval_info` - Get interval name (midi1 midi2 → description)
//! - `midi_name` - MIDI note to name (midi → "C4")
//! - `note_freq` - MIDI to frequency (midi [a4_hz] → Hz)
//! - `bpm_ms` - BPM to milliseconds (bpm [division] → ms)
//! - `euclid_pattern` - Euclidean rhythm (pulses steps → pattern)
//! - `polyrhythm` - Polyrhythm pattern (a b → ASCII pattern)
//! - `edo_freq` - Equal division of octave (degree edo base → Hz)
//! - `swing_ms` - Swing delay (bpm swing_ratio → ms)

use woflang_core::{InterpreterContext, WofValue};
use woflang_runtime::Interpreter;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// PITCH HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Semitone names in 12-TET.
static SEMITONE_NAMES: &[&str] = &[
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
];

/// Convert a note name to pitch class (0-11).
fn note_name_to_pc(name: &str) -> Result<i32, String> {
    let n: String = name.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_uppercase())
        .filter(|c| !c.is_ascii_digit())
        .collect();
    
    // Handle enharmonic equivalents
    match n.as_str() {
        "CB" => Ok(11),  // C♭ -> B
        "B#" => Ok(0),   // B# -> C
        "EB" | "E♭" => Ok(3),
        "BB" | "B♭" => Ok(10),
        "AB" | "A♭" => Ok(8),
        "DB" | "D♭" => Ok(1),
        "GB" | "G♭" => Ok(6),
        _ => {
            for (i, &sn) in SEMITONE_NAMES.iter().enumerate() {
                if n == sn {
                    return Ok(i as i32);
                }
            }
            Err(format!("Unrecognised pitch name '{}'", name))
        }
    }
}

/// Convert pitch class to note name.
fn pc_to_note_name(pc: i32) -> &'static str {
    let pc = ((pc % 12) + 12) % 12;
    SEMITONE_NAMES[pc as usize]
}

/// Convert MIDI note number to name (e.g., 60 -> "C4").
fn midi_to_name(midi: i32) -> String {
    let semi = ((midi % 12) + 12) % 12;
    let octave = (midi / 12) - 1;
    format!("{}{}", SEMITONE_NAMES[semi as usize], octave)
}

/// Convert MIDI note to frequency.
fn midi_to_freq(midi: i32, a4: f64) -> f64 {
    // MIDI 69 == A4
    let n = (midi - 69) as f64;
    a4 * 2.0_f64.powf(n / 12.0)
}

/// Get interval name from semitone distance.
fn interval_name(semitones: i32) -> &'static str {
    let s = ((semitones % 12) + 12) % 12;
    match s {
        0 => "unison / perfect prime",
        1 => "minor second",
        2 => "major second",
        3 => "minor third",
        4 => "major third",
        5 => "perfect fourth",
        6 => "tritone (aug. fourth / dim. fifth)",
        7 => "perfect fifth",
        8 => "minor sixth",
        9 => "major sixth",
        10 => "minor seventh",
        11 => "major seventh",
        _ => "compound interval",
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SCALE AND CHORD DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════

struct ScaleDef {
    name: &'static str,
    degrees: &'static [i32],
}

struct ChordDef {
    name: &'static str,
    intervals: &'static [i32],
}

fn get_scales() -> HashMap<&'static str, ScaleDef> {
    let mut m = HashMap::new();
    m.insert("major", ScaleDef { name: "Major (Ionian)", degrees: &[0,2,4,5,7,9,11] });
    m.insert("ionian", ScaleDef { name: "Major (Ionian)", degrees: &[0,2,4,5,7,9,11] });
    m.insert("natural_minor", ScaleDef { name: "Natural minor (Aeolian)", degrees: &[0,2,3,5,7,8,10] });
    m.insert("aeolian", ScaleDef { name: "Natural minor (Aeolian)", degrees: &[0,2,3,5,7,8,10] });
    m.insert("harmonic_minor", ScaleDef { name: "Harmonic minor", degrees: &[0,2,3,5,7,8,11] });
    m.insert("melodic_minor", ScaleDef { name: "Melodic minor (asc.)", degrees: &[0,2,3,5,7,9,11] });
    m.insert("dorian", ScaleDef { name: "Dorian", degrees: &[0,2,3,5,7,9,10] });
    m.insert("phrygian", ScaleDef { name: "Phrygian", degrees: &[0,1,3,5,7,8,10] });
    m.insert("lydian", ScaleDef { name: "Lydian", degrees: &[0,2,4,6,7,9,11] });
    m.insert("mixolydian", ScaleDef { name: "Mixolydian", degrees: &[0,2,4,5,7,9,10] });
    m.insert("locrian", ScaleDef { name: "Locrian", degrees: &[0,1,3,5,6,8,10] });
    m.insert("pentatonic_major", ScaleDef { name: "Major pentatonic", degrees: &[0,2,4,7,9] });
    m.insert("pentatonic_minor", ScaleDef { name: "Minor pentatonic", degrees: &[0,3,5,7,10] });
    m.insert("blues", ScaleDef { name: "Blues (hexatonic)", degrees: &[0,3,5,6,7,10] });
    m.insert("whole_tone", ScaleDef { name: "Whole-tone", degrees: &[0,2,4,6,8,10] });
    m.insert("chromatic", ScaleDef { name: "Chromatic", degrees: &[0,1,2,3,4,5,6,7,8,9,10,11] });
    m
}

fn get_chords() -> HashMap<&'static str, ChordDef> {
    let mut m = HashMap::new();
    m.insert("maj", ChordDef { name: "Major triad", intervals: &[0,4,7] });
    m.insert("min", ChordDef { name: "Minor triad", intervals: &[0,3,7] });
    m.insert("dim", ChordDef { name: "Diminished triad", intervals: &[0,3,6] });
    m.insert("aug", ChordDef { name: "Augmented triad", intervals: &[0,4,8] });
    m.insert("sus2", ChordDef { name: "Suspended 2nd", intervals: &[0,2,7] });
    m.insert("sus4", ChordDef { name: "Suspended 4th", intervals: &[0,5,7] });
    m.insert("maj7", ChordDef { name: "Major 7th", intervals: &[0,4,7,11] });
    m.insert("min7", ChordDef { name: "Minor 7th", intervals: &[0,3,7,10] });
    m.insert("7", ChordDef { name: "Dominant 7th", intervals: &[0,4,7,10] });
    m.insert("dim7", ChordDef { name: "Diminished 7th", intervals: &[0,3,6,9] });
    m.insert("m7b5", ChordDef { name: "Half-diminished 7th", intervals: &[0,3,6,10] });
    m.insert("add9", ChordDef { name: "Add 9", intervals: &[0,4,7,14] });
    m.insert("6", ChordDef { name: "Sixth chord", intervals: &[0,4,7,9] });
    m
}

/// Build scale notes string.
fn build_scale_notes(root: &str, scale_key: &str) -> String {
    let scales = get_scales();
    let scale_key_lower = scale_key.to_lowercase();
    
    let def = match scales.get(scale_key_lower.as_str()) {
        Some(d) => d,
        None => return format!("Unknown scale type: {}", scale_key),
    };
    
    let root_semi = match note_name_to_pc(root) {
        Ok(pc) => pc,
        Err(e) => return e,
    };
    
    let notes: Vec<&str> = def.degrees.iter()
        .map(|&deg| pc_to_note_name(root_semi + deg))
        .collect();
    
    format!("{} on {}: {}", def.name, root, notes.join(" "))
}

/// Build chord notes string.
fn build_chord_notes(root: &str, chord_key: &str) -> String {
    let chords = get_chords();
    
    let def = match chords.get(chord_key) {
        Some(d) => d,
        None => return format!("Unknown chord type: {}", chord_key),
    };
    
    let root_semi = match note_name_to_pc(root) {
        Ok(pc) => pc,
        Err(e) => return e,
    };
    
    let notes: Vec<&str> = def.intervals.iter()
        .map(|&iv| pc_to_note_name(root_semi + iv))
        .collect();
    
    format!("{} on {}: {}", def.name, root, notes.join(" "))
}

// ═══════════════════════════════════════════════════════════════════════════
// RHYTHM HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Generate Euclidean rhythm pattern.
fn euclidean_pattern(pulses: i32, steps: i32) -> Result<String, String> {
    if pulses <= 0 || steps <= 0 || pulses > steps {
        return Err("euclid: require 0 < pulses <= steps".to_string());
    }
    
    let mut out = vec![false; steps as usize];
    let mut idx = 0usize;
    let step_size = steps / pulses;
    
    for _ in 0..pulses {
        out[idx] = true;
        idx = (idx + step_size as usize) % steps as usize;
    }
    
    Ok(out.iter().map(|&b| if b { 'x' } else { '-' }).collect())
}

/// Generate polyrhythm ASCII pattern.
fn polyrhythm_pattern(a: i32, b: i32) -> String {
    if a <= 0 || b <= 0 {
        return "Polyrhythm requires positive integers".to_string();
    }
    
    // Find LCM
    let mut lcm = a.max(b);
    while lcm <= a * b {
        if lcm % a == 0 && lcm % b == 0 {
            break;
        }
        lcm += 1;
    }
    
    let step_a = lcm / a;
    let step_b = lcm / b;
    
    let top: String = (0..lcm)
        .map(|i| if i % step_a == 0 { 'X' } else { '.' })
        .collect();
    
    let bottom: String = (0..lcm)
        .map(|i| if i % step_b == 0 { 'X' } else { '.' })
        .collect();
    
    format!("{} : {} polyrhythm\nA: {}\nB: {}", a, b, top, bottom)
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION
// ═══════════════════════════════════════════════════════════════════════════

/// Register all music operations.
pub fn register(interp: &mut Interpreter) {
    // ─────────────────────────────────────────────────────────────────────
    // SCALES
    // ─────────────────────────────────────────────────────────────────────
    
    // Build a scale (root scale_type → description)
    // Stack: "C" "major" → "Major (Ionian) on C: C D E F G A B"
    interp.register("build_scale", |interp| {
        let scale_key = interp.stack_mut().pop()?.as_string()?;
        let root = interp.stack_mut().pop()?.as_string()?;
        let result = build_scale_notes(&root, &scale_key);
        interp.stack_mut().push(WofValue::string(result));
        Ok(())
    });

    // Legacy scale_info (same as build_scale but different arg order)
    // Stack: "major" "C" → description
    interp.register("scale_info", |interp| {
        let mode = interp.stack_mut().pop()?.as_string()?;
        let root = interp.stack_mut().pop()?.as_string()?;
        let result = build_scale_notes(&root, &mode);
        interp.stack_mut().push(WofValue::string(result));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // CHORDS
    // ─────────────────────────────────────────────────────────────────────
    
    // Build chord tones (root chord_type → description)
    // Stack: "C" "maj7" → "Major 7th on C: C E G B"
    interp.register("chord_tones", |interp| {
        let chord_key = interp.stack_mut().pop()?.as_string()?;
        let root = interp.stack_mut().pop()?.as_string()?;
        let result = build_chord_notes(&root, &chord_key);
        interp.stack_mut().push(WofValue::string(result));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // INTERVALS
    // ─────────────────────────────────────────────────────────────────────
    
    // Get interval in semitones between two notes
    // Stack: "C" "E" → 4
    interp.register("interval_semitones", |interp| {
        let n2 = interp.stack_mut().pop()?.as_string()?;
        let n1 = interp.stack_mut().pop()?.as_string()?;
        
        let pc1 = note_name_to_pc(&n1).unwrap_or(0);
        let pc2 = note_name_to_pc(&n2).unwrap_or(0);
        let mut dist = pc2 - pc1;
        
        // Wrap to [-6, +6]
        while dist > 6 { dist -= 12; }
        while dist < -6 { dist += 12; }
        
        interp.stack_mut().push(WofValue::integer(dist as i64));
        Ok(())
    });

    // Get interval info from MIDI notes
    // Stack: 60 64 → "4 semitones (major third)"
    interp.register("interval_info", |interp| {
        let upper = interp.stack_mut().pop()?.as_int()? as i32;
        let lower = interp.stack_mut().pop()?.as_int()? as i32;
        let semi = upper - lower;
        let result = format!("{} semitones ({})", semi, interval_name(semi));
        interp.stack_mut().push(WofValue::string(result));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // MIDI/FREQUENCY
    // ─────────────────────────────────────────────────────────────────────
    
    // MIDI note number to name
    // Stack: 60 → "C4"
    interp.register("midi_name", |interp| {
        let midi = interp.stack_mut().pop()?.as_int()? as i32;
        interp.stack_mut().push(WofValue::string(midi_to_name(midi)));
        Ok(())
    });

    // MIDI note to frequency (optional A4 reference)
    // Stack: 69 → 440.0  or  69 432.0 → 432.0
    interp.register("note_freq", |interp| {
        // Check if we have an A4 reference on stack
        let val = interp.stack_mut().pop()?;
        let (midi, a4) = if let Ok(top) = interp.stack().peek() {
            // Two values: midi is below, a4 is val
            let a4 = val.as_float().unwrap_or(440.0);
            let midi = interp.stack_mut().pop()?.as_int()? as i32;
            (midi, a4)
        } else {
            // One value: just midi, use default A4=440
            let midi = val.as_int()? as i32;
            (midi, 440.0)
        };
        
        let freq = midi_to_freq(midi, a4);
        interp.stack_mut().push(WofValue::Float(freq));
        Ok(())
    });

    // EDO frequency (equal division of octave)
    // Stack: degree edo base_freq → Hz
    interp.register("edo_freq", |interp| {
        let base = interp.stack_mut().pop()?.as_float()?;
        let edo = interp.stack_mut().pop()?.as_int()? as i32;
        let degree = interp.stack_mut().pop()?.as_int()? as i32;
        
        if edo <= 0 {
            interp.stack_mut().push(WofValue::Float(0.0));
            return Ok(());
        }
        
        let freq = base * 2.0_f64.powf(degree as f64 / edo as f64);
        interp.stack_mut().push(WofValue::Float(freq));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // RHYTHM/TEMPO
    // ─────────────────────────────────────────────────────────────────────
    
    // BPM to milliseconds (optional division string)
    // Stack: 120 → 500.0  or  120 "1/8" → 250.0
    interp.register("bpm_ms", |interp| {
        let val = interp.stack_mut().pop()?;
        
        let (bpm, div) = if let Ok(s) = val.as_string() {
            // Division string on top
            let bpm = interp.stack_mut().pop()?.as_float()?;
            (bpm, s)
        } else {
            // Just BPM
            (val.as_float()?, "1/4".to_string())
        };
        
        if bpm <= 0.0 {
            interp.stack_mut().push(WofValue::Float(0.0));
            return Ok(());
        }
        
        let beats_per_sec = bpm / 60.0;
        let quarter_ms = 1000.0 / beats_per_sec;
        
        let factor = match div.as_str() {
            "1/1" => 4.0,
            "1/2" => 2.0,
            "1/4" => 1.0,
            "1/8" => 0.5,
            "1/16" => 0.25,
            "1/32" => 0.125,
            _ => 1.0,
        };
        
        interp.stack_mut().push(WofValue::Float(quarter_ms * factor));
        Ok(())
    });

    // Euclidean rhythm pattern
    // Stack: 3 8 → "x--x--x-"
    interp.register("euclid_pattern", |interp| {
        let steps = interp.stack_mut().pop()?.as_int()? as i32;
        let pulses = interp.stack_mut().pop()?.as_int()? as i32;
        
        let pattern = euclidean_pattern(pulses, steps)
            .unwrap_or_else(|e| e);
        interp.stack_mut().push(WofValue::string(pattern));
        Ok(())
    });

    // Polyrhythm pattern
    // Stack: 3 2 → ASCII pattern
    interp.register("polyrhythm", |interp| {
        let b = interp.stack_mut().pop()?.as_int()? as i32;
        let a = interp.stack_mut().pop()?.as_int()? as i32;
        let pattern = polyrhythm_pattern(a, b);
        interp.stack_mut().push(WofValue::string(pattern));
        Ok(())
    });

    // Swing delay calculation
    // Stack: bpm swing_ratio → ms_delay
    interp.register("swing_ms", |interp| {
        let swing = interp.stack_mut().pop()?.as_float()?;
        let bpm = interp.stack_mut().pop()?.as_float()?;
        
        if bpm <= 0.0 {
            interp.stack_mut().push(WofValue::Float(0.0));
            return Ok(());
        }
        
        let beats_per_sec = bpm / 60.0;
        let eighth_ms = (1000.0 / beats_per_sec) * 0.5;
        let straight_offset = eighth_ms;
        let swung_offset = eighth_ms * (2.0 * swing);
        let delta = swung_offset - straight_offset;
        
        interp.stack_mut().push(WofValue::Float(delta));
        Ok(())
    });

    // ─────────────────────────────────────────────────────────────────────
    // HELP
    // ─────────────────────────────────────────────────────────────────────
    
    interp.register("music_help", |_interp| {
        println!("Music Theory Operations:");
        println!();
        println!("  Scales & Chords:");
        println!("    \"C\" \"major\" build_scale   → scale description");
        println!("    \"C\" \"maj7\" chord_tones    → chord tones");
        println!();
        println!("  Intervals:");
        println!("    \"C\" \"E\" interval_semitones → 4");
        println!("    60 64 interval_info        → \"4 semitones (major third)\"");
        println!();
        println!("  MIDI/Frequency:");
        println!("    60 midi_name               → \"C4\"");
        println!("    69 note_freq               → 440.0 Hz");
        println!("    3 19 440 edo_freq          → 19-TET frequency");
        println!();
        println!("  Rhythm:");
        println!("    120 bpm_ms                 → 500.0 ms (quarter note)");
        println!("    120 \"1/8\" bpm_ms           → 250.0 ms (eighth note)");
        println!("    3 8 euclid_pattern         → \"x--x--x-\"");
        println!("    3 2 polyrhythm             → ASCII polyrhythm");
        println!();
        println!("  Scale types: major, minor, dorian, phrygian, lydian,");
        println!("               mixolydian, locrian, pentatonic_major,");
        println!("               pentatonic_minor, blues, whole_tone, chromatic");
        println!();
        println!("  Chord types: maj, min, dim, aug, sus2, sus4,");
        println!("               maj7, min7, 7, dim7, m7b5, add9, 6");
        Ok(())
    });
}
