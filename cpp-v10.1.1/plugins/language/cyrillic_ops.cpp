
#include <iostream>
#include <fstream>
#include <string>
#include <unordered_map>
#include <vector>
#include <stdexcept>
#include <random>

#include "woflang.hpp"
#include "json.hpp"

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
using woflang::WofType;

struct CyrillicEntry {
    std::string letter;
    std::string lower;
    std::string name_en;
    std::string translit;
    std::string phonetic;
    std::string example_native;
    std::string example_translit;
    std::string example_en;
    std::string group;
};

struct CyrillicDB {
    bool loaded = false;
    std::vector<CyrillicEntry> all;
    std::unordered_map<std::string, CyrillicEntry> byLetter;
    std::unordered_map<std::string, CyrillicEntry> byTranslit;
    json metadata;
};

static CyrillicDB g_db;

static json load_json_file(const std::string& path) {
    std::ifstream in(path);
    if (!in) {
        throw std::runtime_error("cyrillic_ops: can't open JSON file: " + path);
    }
    json j;
    in >> j;
    return j;
}

static void init_cyrillic_db_once() {
    if (g_db.loaded) return;

    const std::string paths[] = {
        "cyrillic_database.json",
        "./cyrillic_database.json",
        "./data/cyrillic_database.json",
        "../data/cyrillic_database.json"
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
        throw std::runtime_error("cyrillic_ops: cannot find cyrillic_database.json");
    }

    g_db.metadata = root.value("metadata", json::object());

    if (!root.contains("letters") || !root["letters"].is_array()) {
        throw std::runtime_error("cyrillic_ops: missing 'letters' in JSON");
    }

    for (const auto& k : root["letters"]) {
        CyrillicEntry e;
        e.letter            = k.value("letter", "");
        e.lower             = k.value("lower", "");
        e.name_en           = k.value("name_en", "");
        e.translit          = k.value("translit", "");
        e.phonetic          = k.value("phonetic", "");
        e.example_native    = k.value("example_native", "");
        e.example_translit  = k.value("example_translit", "");
        e.example_en        = k.value("example_en", "");
        e.group             = k.value("group", "");

        if (e.letter.empty()) {
            continue;
        }

        g_db.all.push_back(e);
        g_db.byLetter[e.letter] = e;
        if (!e.lower.empty()) {
            g_db.byLetter[e.lower] = e;
        }
        if (!e.translit.empty()) {
            g_db.byTranslit[e.translit] = e;
        }
    }

    g_db.loaded = true;
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
    if (v.type != WofType::String && v.type != WofType::Symbol) {
        throw std::runtime_error("expected string");
    }
    return std::get<std::string>(v.value);
}

static std::string entry_to_string(const CyrillicEntry& e) {
    // Format: letter|lower|name_en|translit|phonetic|example_native|example_translit|example_en|group
    return e.letter + "|" +
           e.lower + "|" +
           e.name_en + "|" +
           e.translit + "|" +
           e.phonetic + "|" +
           e.example_native + "|" +
           e.example_translit + "|" +
           e.example_en + "|" +
           e.group;
}

// Lookup op: argument can be letter (upper or lower) OR transliteration
static void op_cyrillic_info(WoflangInterpreter& ip) {
    init_cyrillic_db_once();

    std::string key = pop_string(ip);

    // try by exact letter
    auto itL = g_db.byLetter.find(key);
    if (itL != g_db.byLetter.end()) {
        push_string(ip, entry_to_string(itL->second));
        return;
    }

    // try transliteration
    auto itT = g_db.byTranslit.find(key);
    if (itT != g_db.byTranslit.end()) {
        push_string(ip, entry_to_string(itT->second));
        return;
    }

    push_string(ip, "!NOT_FOUND|" + key + "|||||||");
}

// Random letter, optional group filter on top of stack
static void op_cyrillic_random(WoflangInterpreter& ip) {
    init_cyrillic_db_once();

    std::string filter;
    if (!ip.stack.empty() &&
        (ip.stack.back().type == WofType::String ||
         ip.stack.back().type == WofType::Symbol)) {
        filter = pop_string(ip);
    }

    std::vector<const CyrillicEntry*> candidates;
    candidates.reserve(g_db.all.size());

    if (filter.empty()) {
        for (const auto& e : g_db.all) {
            candidates.push_back(&e);
        }
    } else {
        for (const auto& e : g_db.all) {
            if (e.group == filter) {
                candidates.push_back(&e);
            }
        }
    }

    if (candidates.empty()) {
        push_string(ip, "!NO_MATCH|" + filter + "|||||||");
        return;
    }

    static std::mt19937 gen{std::random_device{}()};
    std::uniform_int_distribution<std::size_t> dist(0, candidates.size() - 1);
    const CyrillicEntry* e = candidates[dist(gen)];
    push_string(ip, entry_to_string(*e));
}

// Summary of groups / metadata
static void op_cyrillic_groups(WoflangInterpreter& ip) {
    init_cyrillic_db_once();

    std::string summary = "Cyrillic DB: ";
    if (g_db.metadata.contains("description") && g_db.metadata["description"].is_string()) {
        summary += g_db.metadata["description"].get<std::string>() + " ";
    }
    if (g_db.metadata.contains("total_letters") && g_db.metadata["total_letters"].is_number_integer()) {
        summary += "(total letters: " + std::to_string(g_db.metadata["total_letters"].get<int>()) + ") ";
    }
    if (g_db.metadata.contains("groups") && g_db.metadata["groups"].is_object()) {
        summary += "Groups: ";
        const auto& groups = g_db.metadata["groups"];
        for (auto it = groups.begin(); it != groups.end(); ++it) {
            if (!it.value().is_number_integer()) continue;
            summary += it.key() + "=" + std::to_string(it.value().get<int>()) + ", ";
        }
        if (summary.size() >= 2 && summary.substr(summary.size() - 2) == ", ") {
            summary.erase(summary.size() - 2);
        }
    }

    push_string(ip, summary);
}

// Simple quiz helper: push a textual question and the expected answer on stack:
// [question_string expected_answer]
static void op_cyrillic_quiz(WoflangInterpreter& ip) {
    init_cyrillic_db_once();

    if (g_db.all.empty()) {
        push_string(ip, "No letters loaded");
        push_string(ip, "");
        return;
    }

    static std::mt19937 gen{std::random_device{}()};
    std::uniform_int_distribution<std::size_t> dist(0, g_db.all.size() - 1);
    const CyrillicEntry& e = g_db.all[dist(gen)];

    std::string question = "What is the sound / transliteration of letter '" + e.letter +
                           "' (example: " + e.example_native + " = " + e.example_en + ")?";
    std::string answer = e.translit;

    // Convention: push question, then expected answer so user code can compare
    push_string(ip, question);
    push_string(ip, answer);
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("cyrillic_info", [](WoflangInterpreter& ip) {
        try {
            op_cyrillic_info(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("cyrillic_info error: ") + e.what()));
        }
    });

    interp.register_op("cyrillic_random", [](WoflangInterpreter& ip) {
        try {
            op_cyrillic_random(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("cyrillic_random error: ") + e.what()));
        }
    });

    interp.register_op("cyrillic_groups", [](WoflangInterpreter& ip) {
        try {
            op_cyrillic_groups(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("cyrillic_groups error: ") + e.what()));
        }
    });

    interp.register_op("cyrillic_quiz", [](WoflangInterpreter& ip) {
        try {
            op_cyrillic_quiz(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("cyrillic_quiz error: ") + e.what()));
        }
    });

    std::cout << "[cyrillic_ops] registered cyrillic_info, cyrillic_random, cyrillic_groups, cyrillic_quiz\n";
}
