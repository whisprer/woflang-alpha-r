// =====================================================
// kanji_ops.cpp - learnkanji the fun way with woflang!
// =====================================================

#include <iostream>
#include <fstream>
#include <string>
#include <unordered_map>
#include <vector>
#include <stdexcept>
#include <random>
#include <cctype>

#include "woflang.hpp"
#include "json.hpp"  // Ensure json.hpp is in the include path

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

using json = nlohmann::json;
using woflang::WoflangInterpreter;
using woflang::WofValue;

struct KanjiEntry {
    std::string kanji;
    std::string onyomi;
    std::string romaji;
    std::string meaning;
    std::string example;
    std::string level;
};

struct KanjiDB {
    bool loaded = false;
    std::vector<KanjiEntry> all;
    std::unordered_map<std::string, KanjiEntry> byKanji;
    json metadata;
};

static KanjiDB g_db;

// --- JSON loading helpers ---

static json load_json_file(const std::string& path) {
    std::ifstream in(path);
    if (!in) {
        throw std::runtime_error("kanji_ops: can't open JSON file: " + path);
    }
    json j;
    in >> j;
    return j;
}

static void init_kanji_db_once() {
    if (g_db.loaded) return;

    const std::string paths[] = {
        "kanji_database.json",
        "./kanji_database.json",
        "./data/kanji_database.json"
    };

    json root;
    bool loaded = false;
    for (const auto& p : paths) {
        try {
            root = load_json_file(p);
            loaded = true;
            break;
        } catch (...) {
            // try next
        }
    }

    if (!loaded) {
        throw std::runtime_error("kanji_ops: cannot find kanji_database.json");
    }

    g_db.metadata = root.value("metadata", json::object());

    if (!root.contains("kanji_by_level") || !root["kanji_by_level"].is_object()) {
        throw std::runtime_error("kanji_ops: missing 'kanji_by_level' in JSON");
    }

    const json& levels = root["kanji_by_level"];
    for (auto it = levels.begin(); it != levels.end(); ++it) {
        std::string levelName = it.key();
        const json& arr = it.value();
        if (!arr.is_array()) continue;

        for (const auto& k : arr) {
            KanjiEntry e;
            e.kanji   = k.value("kanji", "");
            e.onyomi  = k.value("onyomi", "");
            e.romaji  = k.value("romaji", "");
            e.meaning = k.value("meaning", "");
            e.example = k.value("example", "");
            e.level   = levelName;

            if (e.kanji.empty()) {
                continue;
            }

            g_db.all.push_back(e);
            g_db.byKanji[e.kanji] = e;
        }
    }

    g_db.loaded = true;
}

// --- Stack helpers ---

static std::string kanjiEntryToString(const KanjiEntry& e) {
    // Format: kanji|onyomi|romaji|meaning|example|level
    return e.kanji + "|" +
           e.onyomi + "|" +
           e.romaji + "|" +
           e.meaning + "|" +
           e.example + "|" +
           e.level;
}

static void push_string(WoflangInterpreter& ip, const std::string& s) {
    ip.push(WofValue::make_string(s));
}

static std::string pop_string(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        throw std::runtime_error("stack underflow: expected string");
    }
    WofValue v = ip.stack.back();
    ip.stack.pop_back();
    if (v.type != woflang::WofType::String && v.type != woflang::WofType::Symbol) {
        throw std::runtime_error("expected string");
    }
    return std::get<std::string>(v.value);
}

// --- Ops ---

// 1) Look up by kanji glyph
static void op_kanji_info(WoflangInterpreter& ip) {
    init_kanji_db_once();

    std::string key = pop_string(ip);
    auto it = g_db.byKanji.find(key);
    if (it == g_db.byKanji.end()) {
        // Preserve your original error shape
        push_string(ip, key + "|!NOT_FOUND||||");
        return;
    }

    push_string(ip, kanjiEntryToString(it->second));
}

// 2) Search meanings (case-insensitive substring)
static void op_kanji_search_meaning(WoflangInterpreter& ip) {
    init_kanji_db_once();

    std::string query = pop_string(ip);
    if (query.empty()) {
        push_string(ip, "!NO_RESULTS|||||");
        return;
    }

    auto to_lower_inplace = [](std::string& s) {
        for (char& c : s) {
            c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
        }
    };

    std::string qLower = query;
    to_lower_inplace(qLower);

    std::size_t results_count = 0;
    for (const auto& e : g_db.all) {
        std::string mLower = e.meaning;
        to_lower_inplace(mLower);

        if (mLower.find(qLower) != std::string::npos) {
            push_string(ip, kanjiEntryToString(e));
            ++results_count;
        }
    }

    if (results_count == 0) {
        push_string(ip, "!NO_RESULTS|||||");
    }
}

// 3) Random kanji, optionally filtered by level prefix
static void op_kanji_random(WoflangInterpreter& ip) {
    init_kanji_db_once();

    std::string filter;
    if (!ip.stack.empty() &&
        (ip.stack.back().type == woflang::WofType::String ||
         ip.stack.back().type == woflang::WofType::Symbol)) {
        filter = pop_string(ip);
    }

    std::vector<const KanjiEntry*> candidates;
    candidates.reserve(g_db.all.size());

    if (filter.empty()) {
        for (const auto& e : g_db.all) {
            candidates.push_back(&e);
        }
    } else {
        for (const auto& e : g_db.all) {
            // "starts with" semantics for level
            if (e.level.rfind(filter, 0) == 0) {
                candidates.push_back(&e);
            }
        }
    }

    if (candidates.empty()) {
        push_string(ip, "!NO_MATCH|Filter: " + filter + "||||");
        return;
    }

    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::uniform_int_distribution<int> dist(
        0,
        static_cast<int>(candidates.size()) - 1
    );

    const KanjiEntry* e = candidates[dist(gen)];
    push_string(ip, kanjiEntryToString(*e));
}

// 4) Summary of levels/metadata
static void op_kanji_levels(WoflangInterpreter& ip) {
    init_kanji_db_once();

    std::string summary = "Kanji DB summary: ";

    if (g_db.metadata.contains("total_kanji") && g_db.metadata["total_kanji"].is_number_integer()) {
        summary += "Total Kanji: " + std::to_string(g_db.metadata["total_kanji"].get<int>()) + ". ";
    }
    if (g_db.metadata.contains("description") && g_db.metadata["description"].is_string()) {
        summary += g_db.metadata["description"].get<std::string>() + ". ";
    }
    if (g_db.metadata.contains("levels") && g_db.metadata["levels"].is_object()) {
        summary += "Levels: ";
        const auto& levels = g_db.metadata["levels"];
        for (auto it = levels.begin(); it != levels.end(); ++it) {
            const std::string levelName = it.key();
            if (!it.value().is_number_integer()) continue;
            int count = it.value().get<int>();
            summary += levelName + ": " + std::to_string(count) + ", ";
        }
        // Trim trailing ", " if present
        if (summary.size() >= 2 && summary.substr(summary.size() - 2) == ", ") {
            summary.erase(summary.size() - 2);
        }
    }

    push_string(ip, summary);
}

// --- Plugin entry point ---

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("kanji_info", [](WoflangInterpreter& ip) {
        try {
            op_kanji_info(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("kanji_info error: ") + e.what()));
        }
    });

    interp.register_op("kanji_search_meaning", [](WoflangInterpreter& ip) {
        try {
            op_kanji_search_meaning(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("kanji_search_meaning error: ") + e.what()));
        }
    });

    interp.register_op("kanji_random", [](WoflangInterpreter& ip) {
        try {
            op_kanji_random(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("kanji_random error: ") + e.what()));
        }
    });

    interp.register_op("kanji_levels", [](WoflangInterpreter& ip) {
        try {
            op_kanji_levels(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("kanji_levels error: ") + e.what()));
        }
    });

    std::cout << "[kanji_ops] plugin registered ops: kanji_info, kanji_search_meaning, kanji_random, kanji_levels\n";
}
