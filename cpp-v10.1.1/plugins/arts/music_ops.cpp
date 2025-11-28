// music_ops.cpp - WofLang music theory helper plugin
//
// This plugin focuses on *theory* and pattern generation, not audio I/O.
// It exposes a handful of stack operations that work with integers and
// strings to describe scales, intervals and rhythms.
//
// All functions work directly on the WofLang data stack without relying on
// WofValue::make_* helpers, to keep them compatible with the v10 core.

#include <algorithm>
#include <array>
#include <cmath>
#include <cstdint>
#include <cctype>
#include <stdexcept>
#include <string>
#include <utility>
#include <variant>
#include <vector>
#include <map>
#include <sstream>
#include <iostream>

#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
#define WOFLANG_PLUGIN_EXPORT extern "C"
#endif

using namespace woflang;

// ---- Core WofValue helpers --------------------------------------------------

static void ensure_stack_size(WoflangInterpreter &ip,
                              std::size_t         needed,
                              const char         *op_name) {
    if (ip.stack.size() < needed) {
        throw std::runtime_error(std::string(op_name) + ": stack underflow");
    }
}

static WofValue pop_raw(WoflangInterpreter &ip, const char *op_name) {
    ensure_stack_size(ip, 1, op_name);
    WofValue v = ip.stack.back();
    ip.stack.pop_back();
    return v;
}

static WofValue make_int64(int64_t v) {
    WofValue out;
    out.type  = WofType::Integer;
    out.value = v;
    return out;
}

static WofValue make_double(double v) {
    WofValue out;
    out.type  = WofType::Double;
    out.value = v;
    return out;
}

static WofValue make_string(const std::string &s) {
    WofValue out;
    out.type  = WofType::String;
    out.value = s;
    return out;
}

static void push_int(WoflangInterpreter &ip, int64_t v) {
    ip.stack.push_back(make_int64(v));
}

static void push_double(WoflangInterpreter &ip, double v) {
    ip.stack.push_back(make_double(v));
}

static void push_string(WoflangInterpreter &ip, const std::string &s) {
    ip.stack.push_back(make_string(s));
}

static double to_double_checked(const WofValue &v, const char *op_name) {
    if (v.type == WofType::Integer) {
        return static_cast<double>(std::get<int64_t>(v.value));
    }
    if (v.type == WofType::Double) {
        return std::get<double>(v.value);
    }
    throw std::runtime_error(std::string(op_name) + ": expected numeric value");
}

static int64_t to_int_checked(const WofValue &v, const char *op_name) {
    double d = to_double_checked(v, op_name);
    if (!std::isfinite(d)) {
        throw std::runtime_error(std::string(op_name) + ": non-finite numeric value");
    }
    return static_cast<int64_t>(d);
}

static std::string to_string_value(const WofValue &v, const char *op_name) {
    if (v.type == WofType::String) {
        return std::get<std::string>(v.value);
    }
    // Fallback: stringify numerics.
    if (v.type == WofType::Integer) {
        return std::to_string(std::get<int64_t>(v.value));
    }
    if (v.type == WofType::Double) {
        return std::to_string(std::get<double>(v.value));
    }
    throw std::runtime_error(std::string(op_name) + ": expected string or numeric");
}

static double pop_numeric(WoflangInterpreter &ip, const char *op_name) {
    WofValue v = pop_raw(ip, op_name);
    return to_double_checked(v, op_name);
}

static int64_t pop_int(WoflangInterpreter &ip, const char *op_name) {
    WofValue v = pop_raw(ip, op_name);
    return to_int_checked(v, op_name);
}

static std::string pop_string(WoflangInterpreter &ip, const char *op_name) {
    WofValue v = pop_raw(ip, op_name);
    return to_string_value(v, op_name);
}

// ---- Pitch helpers ----------------------------------------------------------

static const std::array<const char *, 12> kSemitoneNames = {
    "C", "C#", "D", "D#", "E", "F",
    "F#", "G", "G#", "A", "A#", "B"
};

static int note_name_to_pc(const std::string &name) {
    // Normalise: uppercase, remove whitespace
    std::string n;
    n.reserve(name.size());
    for (char ch : name) {
        if (ch == ' ' || ch == '\t') continue;
        n.push_back(static_cast<char>(std::toupper(static_cast<unsigned char>(ch))));
    }
    // Strip octave digits if present.
    while (!n.empty() && std::isdigit(static_cast<unsigned char>(n.back()))) {
        n.pop_back();
    }
    if (n == "CB") return 11; // C♭ -> B
    if (n == "B#") return 0;  // B# -> C
    if (n == "EB") return 3;  // E♭
    if (n == "BB") return 10; // B♭
    if (n == "AB") return 8;  // A♭
    if (n == "DB") return 1;  // D♭
    if (n == "GB") return 6;  // G♭

    for (std::size_t i = 0; i < kSemitoneNames.size(); ++i) {
        if (n == kSemitoneNames[i]) {
            return static_cast<int>(i);
        }
    }
    throw std::runtime_error("music: unrecognised pitch name '" + name + "'");
}

static int note_name_to_semitone(const std::string &name) {
    // For 12-TET pitch-class work, semitone == pitch-class index.
    return note_name_to_pc(name);
}

static std::string pc_to_note_name(int pc) {
    pc %= 12;
    if (pc < 0) pc += 12;
    return std::string(kSemitoneNames[static_cast<std::size_t>(pc)]);
}

// ---- Scale generation -------------------------------------------------------

static std::vector<int> build_scale(const std::vector<int> &steps,
                                    int                     root_pc,
                                    std::size_t             length) {
    std::vector<int> out;
    out.reserve(length);
    int acc = root_pc;
    out.push_back(root_pc);
    std::size_t idx = 0;
    while (out.size() < length) {
        acc += steps[idx];
        out.push_back(acc % 12);
        idx = (idx + 1) % steps.size();
    }
    return out;
}

static std::string describe_scale(const std::string &root,
                                  const std::string &mode) {
    int root_pc = note_name_to_pc(root);

    // step patterns in semitones
    const std::vector<int> ionian  = {2, 2, 1, 2, 2, 2, 1}; // major
    const std::vector<int> aeolian = {2, 1, 2, 2, 1, 2, 2}; // natural minor
    const std::vector<int> dorian  = {2, 1, 2, 2, 2, 1, 2};
    const std::vector<int> phryg   = {1, 2, 2, 2, 1, 2, 2};
    const std::vector<int> lydian  = {2, 2, 2, 1, 2, 2, 1};
    const std::vector<int> mixol   = {2, 2, 1, 2, 2, 1, 2};
    const std::vector<int> locrian = {1, 2, 2, 1, 2, 2, 2};

    std::vector<int> scale;
    std::string mode_norm;
    mode_norm.reserve(mode.size());
    for (char c : mode) {
        mode_norm.push_back(static_cast<char>(std::tolower(static_cast<unsigned char>(c))));
    }

    if (mode_norm == "major" || mode_norm == "ionian") {
        scale = build_scale(ionian, root_pc, 7);
    } else if (mode_norm == "minor" || mode_norm == "aeolian") {
        scale = build_scale(aeolian, root_pc, 7);
    } else if (mode_norm == "dorian") {
        scale = build_scale(dorian, root_pc, 7);
    } else if (mode_norm == "phrygian") {
        scale = build_scale(phryg, root_pc, 7);
    } else if (mode_norm == "lydian") {
        scale = build_scale(lydian, root_pc, 7);
    } else if (mode_norm == "mixolydian") {
        scale = build_scale(mixol, root_pc, 7);
    } else if (mode_norm == "locrian") {
        scale = build_scale(locrian, root_pc, 7);
    } else if (mode_norm == "pentatonic_major") {
        const std::vector<int> steps = {2, 2, 3, 2, 3};
        scale = build_scale(steps, root_pc, 5);
    } else if (mode_norm == "pentatonic_minor") {
        const std::vector<int> steps = {3, 2, 2, 3, 2};
        scale = build_scale(steps, root_pc, 5);
    } else {
        throw std::runtime_error("music: unknown mode '" + mode + "'");
    }

    std::string out = root + " " + mode + " scale: ";
    for (std::size_t i = 0; i < scale.size(); ++i) {
        if (i) out += ' ';
        out += pc_to_note_name(scale[i]);
    }
    return out;
}

// ---- Rhythm helpers ---------------------------------------------------------

static std::string describe_euclidean(int pulses, int steps) {
    // Bjorklund-style distribution (simplified): generate a binary pattern.
    if (pulses <= 0 || steps <= 0 || pulses > steps) {
        throw std::runtime_error("euclid: require 0 < pulses <= steps");
    }

    std::vector<int> out(steps, 0);
    int idx = 0;
    for (int i = 0; i < pulses; ++i) {
        out[idx] = 1;
        idx = (idx + steps / pulses) % steps;
    }

    std::string s;
    s.reserve(static_cast<std::size_t>(steps) * 2);
    for (int v : out) {
        s.push_back(v ? 'x' : '-');
    }
    return s;
}

// ---- Stack-level operations (basic helpers) ---------------------------------

static void op_scale_info(WoflangInterpreter &ip) {
    // Stack: root mode  --  description-string
    const char *op = "scale_info";
    std::string mode = pop_string(ip, op);
    std::string root = pop_string(ip, op);

    std::string desc = describe_scale(root, mode);
    push_string(ip, desc);
}

static void op_interval_semitones(WoflangInterpreter &ip) {
    // Stack: note2 note1  --  semitone_distance (note2 - note1)
    const char *op = "interval_semitones";
    std::string n2 = pop_string(ip, op);
    std::string n1 = pop_string(ip, op);

    int pc1 = note_name_to_pc(n1);
    int pc2 = note_name_to_pc(n2);
    int dist = pc2 - pc1;
    // Wrap to [-6, +6] for a compact interval
    while (dist > 6)  dist -= 12;
    while (dist < -6) dist += 12;

    push_int(ip, dist);
}

static void op_euclid_pattern(WoflangInterpreter &ip) {
    // Stack: pulses steps  --  pattern_string (x/-)
    const char *op = "euclid_pattern";
    int64_t steps  = pop_int(ip, op);
    int64_t pulses = pop_int(ip, op);

    std::string pat = describe_euclidean(static_cast<int>(pulses),
                                         static_cast<int>(steps));
    push_string(ip, pat);
}

static void op_music_help(WoflangInterpreter &ip) {
    (void)ip;
    // Intentionally no-op – can be used as a probe / documentation hook.
}

// ---- Extra music helpers (from original plugin) -----------------------------

static std::string semitone_to_note_name(int semi) {
    static const char* names[] = {
        "C", "C#",
        "D", "D#",
        "E",
        "F", "F#",
        "G", "G#",
        "A", "A#",
        "B"
    };
    semi %= 12;
    if (semi < 0) semi += 12;
    return names[semi];
}

static std::string midi_to_name(int midi) {
    int semi = midi % 12;
    if (semi < 0) semi += 12;
    int octave = (midi / 12) - 1;
    std::ostringstream oss;
    oss << semitone_to_note_name(semi) << octave;
    return oss.str();
}

static double midi_to_freq(int midi, double a4 = 440.0) {
    // MIDI 69 == A4
    double n = static_cast<double>(midi - 69);
    return a4 * std::pow(2.0, n / 12.0);
}

// Interval names in 12-TET
static std::string interval_name(int semitones) {
    int s = semitones % 12;
    if (s < 0) s += 12;
    switch (s) {
        case 0:  return "unison / perfect prime";
        case 1:  return "minor second";
        case 2:  return "major second";
        case 3:  return "minor third";
        case 4:  return "major third";
        case 5:  return "perfect fourth";
        case 6:  return "tritone (aug. fourth / dim. fifth)";
        case 7:  return "perfect fifth";
        case 8:  return "minor sixth";
        case 9:  return "major sixth";
        case 10: return "minor seventh";
        case 11: return "major seventh";
        default: return "compound interval";
    }
}

// ---------- Scale / chord definitions ----------

struct ScaleDef {
    std::string name;
    std::vector<int> degrees; // semitone offsets from root
};

struct ChordDef {
    std::string name;
    std::vector<int> intervals; // semitone offsets from root
};

static const std::map<std::string, ScaleDef> kScales = {
    {"major",            {"Major (Ionian)",           {0,2,4,5,7,9,11}}},
    {"ionian",           {"Major (Ionian)",           {0,2,4,5,7,9,11}}},
    {"natural_minor",    {"Natural minor (Aeolian)",  {0,2,3,5,7,8,10}}},
    {"aeolian",          {"Natural minor (Aeolian)",  {0,2,3,5,7,8,10}}},
    {"harmonic_minor",   {"Harmonic minor",           {0,2,3,5,7,8,11}}},
    {"melodic_minor",    {"Melodic minor (asc.)",     {0,2,3,5,7,9,11}}},
    {"dorian",           {"Dorian",                   {0,2,3,5,7,9,10}}},
    {"phrygian",         {"Phrygian",                 {0,1,3,5,7,8,10}}},
    {"lydian",           {"Lydian",                   {0,2,4,6,7,9,11}}},
    {"mixolydian",       {"Mixolydian",               {0,2,4,5,7,9,10}}},
    {"locrian",          {"Locrian",                  {0,1,3,5,6,8,10}}},
    {"pentatonic_major", {"Major pentatonic",         {0,2,4,7,9}}},
    {"pentatonic_minor", {"Minor pentatonic",         {0,3,5,7,10}}},
    {"blues",            {"Blues (hexatonic)",        {0,3,5,6,7,10}}},
    {"whole_tone",       {"Whole-tone",               {0,2,4,6,8,10}}},
    {"chromatic",        {"Chromatic",                {0,1,2,3,4,5,6,7,8,9,10,11}}}
};

static const std::map<std::string, ChordDef> kChords = {
    {"maj",   {"Major triad",          {0,4,7}}},
    {"min",   {"Minor triad",          {0,3,7}}},
    {"dim",   {"Diminished triad",     {0,3,6}}},
    {"aug",   {"Augmented triad",      {0,4,8}}},
    {"sus2",  {"Suspended 2nd",        {0,2,7}}},
    {"sus4",  {"Suspended 4th",        {0,5,7}}},
    {"maj7",  {"Major 7th",            {0,4,7,11}}},
    {"min7",  {"Minor 7th",            {0,3,7,10}}},
    {"7",     {"Dominant 7th",         {0,4,7,10}}},
    {"dim7",  {"Diminished 7th",       {0,3,6,9}}},
    {"m7b5",  {"Half-diminished 7th",  {0,3,6,10}}},
    {"add9",  {"Add 9",                {0,4,7,14}}},
    {"6",     {"Sixth chord",          {0,4,7,9}}},
};

static std::string build_scale_notes(const std::string& rootName, const std::string& scaleKey) {
    auto it = kScales.find(scaleKey);
    if (it == kScales.end()) {
        return "Unknown scale type: " + scaleKey;
    }
    int rootSemi = note_name_to_semitone(rootName);
    const auto& def = it->second;

    std::ostringstream oss;
    oss << def.name << " on " << rootName << ": ";
    bool first = true;
    for (int deg : def.degrees) {
        int semi = (rootSemi + deg) % 12;
        if (!first) oss << " ";
        oss << semitone_to_note_name(semi);
        first = false;
    }
    return oss.str();
}

static std::string build_chord_notes(const std::string& rootName, const std::string& chordKey) {
    auto it = kChords.find(chordKey);
    if (it == kChords.end()) {
        return "Unknown chord type: " + chordKey;
    }
    int rootSemi = note_name_to_semitone(rootName);
    const auto& def = it->second;

    std::ostringstream oss;
    oss << def.name << " on " << rootName << ": ";
    bool first = true;
    for (int iv : def.intervals) {
        int semi = (rootSemi + iv) % 12;
        if (!first) oss << " ";
        oss << semitone_to_note_name(semi);
        first = false;
    }
    return oss.str();
}

// Simple polyrhythm visualiser: returns string like "3 : 2\nA: X..X..\nB: X...."
static std::string polyrhythm_pattern(int a, int b) {
    if (a <= 0 || b <= 0) return "Polyrhythm requires positive integers";
    int lcm = a * b;
    for (int i = std::max(a, b); i <= a * b; ++i) {
        if (i % a == 0 && i % b == 0) { lcm = i; break; }
    }

    std::string top, bottom;
    top.reserve(lcm);
    bottom.reserve(lcm);

    int stepA = lcm / a;
    int stepB = lcm / b;

    for (int i = 0; i < lcm; ++i) {
        top.push_back((i % stepA == 0) ? 'X' : '.');
    }
    for (int i = 0; i < lcm; ++i) {
        bottom.push_back((i % stepB == 0) ? 'X' : '.');
    }

    std::ostringstream oss;
    oss << a << " : " << b << " polyrhythm\n"
        << "A: " << top << "\n"
        << "B: " << bottom;
    return oss.str();
}

// ---------- Plugin entry point ----------------------------------------------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter &interp) {
    // Legacy ops (intervals, scales, Euclidean rhythms)
    interp.register_op("scale_info", [](WoflangInterpreter &ip) {
        op_scale_info(ip);
    });

    interp.register_op("interval_semitones", [](WoflangInterpreter &ip) {
        op_interval_semitones(ip);
    });

    interp.register_op("euclid_pattern", [](WoflangInterpreter &ip) {
        op_euclid_pattern(ip);
    });

    interp.register_op("music_help", [](WoflangInterpreter &ip) {
        op_music_help(ip);
    });

    // bpm_ms: [bpm (number), optional division string "1/4", "1/8", ...] -> ms
    interp.register_op("bpm_ms", [](WoflangInterpreter& ip) {
        std::string div = "1/4";
        if (!ip.stack.empty() && ip.stack.back().type == WofType::String) {
            div = to_string_value(ip.stack.back(), "bpm_ms");
            ip.stack.pop_back();
        }
        double bpm = pop_numeric(ip, "bpm_ms");
        if (bpm <= 0.0) {
            push_double(ip, 0.0);
            return;
        }

        double beatsPerSecond = bpm / 60.0;
        double quarterMs = 1000.0 / beatsPerSecond; // duration of quarter note

        double factor = 1.0;
        if (div == "1/1")       factor = 4.0;
        else if (div == "1/2")  factor = 2.0;
        else if (div == "1/4")  factor = 1.0;
        else if (div == "1/8")  factor = 0.5;
        else if (div == "1/16") factor = 0.25;
        else if (div == "1/32") factor = 0.125;
        else                    factor = 1.0;

        double ms = quarterMs * factor;
        push_double(ip, ms);
    });

    // note_freq: [midi, (optional A4 reference Hz)] -> frequency Hz
    //   Example: 69 440 note_freq   -> 440.0
    interp.register_op("note_freq", [](WoflangInterpreter& ip) {
        double a4 = 440.0;
        if (!ip.stack.empty()) {
            const WofValue &top = ip.stack.back();
            if (top.type == WofType::Integer || top.type == WofType::Double) {
                a4 = to_double_checked(top, "note_freq");
                ip.stack.pop_back();
            }
        }
        int midi = static_cast<int>(pop_numeric(ip, "note_freq"));
        double f = midi_to_freq(midi, a4);
        push_double(ip, f);
    });

    // midi_name: [midi] -> "C4"
    interp.register_op("midi_name", [](WoflangInterpreter& ip) {
        int midi = static_cast<int>(pop_numeric(ip, "midi_name"));
        push_string(ip, midi_to_name(midi));
    });

    // interval_info: [upper_midi, lower_midi] -> "X semitones (name)"
    interp.register_op("interval_info", [](WoflangInterpreter& ip) {
        int upper = static_cast<int>(pop_numeric(ip, "interval_info"));
        int lower = static_cast<int>(pop_numeric(ip, "interval_info"));
        int semi = upper - lower;
        std::ostringstream oss;
        oss << semi << " semitones (" << interval_name(semi) << ")";
        push_string(ip, oss.str());
    });

    // build_scale: [root_name, scale_key] -> "Scale-name on Root: notes..."
    //   Example: "C" "major" build_scale
    interp.register_op("build_scale", [](WoflangInterpreter& ip) {
        std::string scaleKey = pop_string(ip, "build_scale");
        std::string root     = pop_string(ip, "build_scale");
        if (root.empty() || scaleKey.empty()) {
            push_string(ip, "[music] build_scale: missing args");
            return;
        }
        push_string(ip, build_scale_notes(root, scaleKey));
    });

    // chord_tones: [root_name, chord_key] -> "Chord-name on Root: notes..."
    //   Example: "C" "maj7" chord_tones
    interp.register_op("chord_tones", [](WoflangInterpreter& ip) {
        std::string chordKey = pop_string(ip, "chord_tones");
        std::string root     = pop_string(ip, "chord_tones");
        if (root.empty() || chordKey.empty()) {
            push_string(ip, "[music] chord_tones: missing args");
            return;
        }
        push_string(ip, build_chord_notes(root, chordKey));
    });

    // polyrhythm: [a, b] -> ASCII pattern string
    //   Example: 3 2 polyrhythm
    interp.register_op("polyrhythm", [](WoflangInterpreter& ip) {
        int b = static_cast<int>(pop_numeric(ip, "polyrhythm"));
        int a = static_cast<int>(pop_numeric(ip, "polyrhythm"));
        push_string(ip, polyrhythm_pattern(a, b));
    });

    // edo_freq: [degree, edo, base_freq] -> Hz (microtonal equal division of octave)
    //   Example: 3 19 440 edo_freq  -> 3rd step of 19-TET above 440
    interp.register_op("edo_freq", [](WoflangInterpreter& ip) {
        double base   = pop_numeric(ip, "edo_freq");             // e.g. 440.0
        int    edo    = static_cast<int>(pop_numeric(ip, "edo_freq"));   // e.g. 19, 24...
        int    degree = static_cast<int>(pop_numeric(ip, "edo_freq"));   // step index
        if (edo <= 0) {
            push_double(ip, 0.0);
            return;
        }
        double f = base * std::pow(2.0,
                                   static_cast<double>(degree) /
                                   static_cast<double>(edo));
        push_double(ip, f);
    });

    // swing_ms: [bpm, swing_ratio] -> ms delay of off-beat (approx)
    //   swing_ratio ~0.5 = straight, ~0.66 = triplet feel
    interp.register_op("swing_ms", [](WoflangInterpreter& ip) {
        double swing = pop_numeric(ip, "swing_ms"); // 0.5..0.7 typical
        double bpm   = pop_numeric(ip, "swing_ms");
        if (bpm <= 0.0) {
            push_double(ip, 0.0);
            return;
        }
        double beatsPerSecond = bpm / 60.0;
        double eighthMs = (1000.0 / beatsPerSecond) * 0.5; // 8th note is half a quarter
        double straightOffset = eighthMs;
        double swungOffset = eighthMs * (2.0 * swing); // crude but useful
        double delta = swungOffset - straightOffset;
        push_double(ip, delta);
    });

    // call_response_hint: [call_motif, response_motif] -> little text suggestion
    interp.register_op("call_response_hint", [](WoflangInterpreter& ip) {
        std::string response = pop_string(ip, "call_response_hint");
        std::string call     = pop_string(ip, "call_response_hint");
        std::ostringstream oss;
        oss << "Call/response idea:\n"
            << "  Call:      " << call << "\n"
            << "  Response:  " << response << "\n"
            << "Try leaving space after the call, then answer with a "
            << "rhythmically simpler, slightly higher phrase.";
        push_string(ip, oss.str());
    });

    std::cout << "[music] Music theory ops loaded: "
              << "scale_info interval_semitones euclid_pattern music_help "
              << "bpm_ms note_freq midi_name interval_info build_scale "
              << "chord_tones polyrhythm edo_freq swing_ms call_response_hint\n";
}
