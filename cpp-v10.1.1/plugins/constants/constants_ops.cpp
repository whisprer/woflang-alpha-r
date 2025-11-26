// =======================================================
// constants_ops.cpp - liest of useful constants on dmeand
// =======================================================

#include <iostream>
#include <fstream>
#include <string>
#include <unordered_map>
#include <variant>
#include <vector>
#include <stdexcept>
#include <random>
#include <cctype>   // <- was <ctype>, which does not exist in libstdc++

#include "woflang.hpp"
#include "json.hpp" // Ensure this is in your include path

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

struct ConstantValue {
    enum class Type { Double, String } type;
    std::variant<double, std::string> value;
    std::string unit;  // optional
    std::string desc;  // optional
};

struct ConstantEntry {
    std::string name;
    std::string symbol;
    int opcode;
    ConstantValue value;
    std::string category;
    std::string desc;
    std::string unit;
};

struct ConstantsDB {
    bool loaded = false;
    std::unordered_map<std::string, ConstantEntry> byName;
    std::unordered_map<int, ConstantEntry> byOpcode;
    std::vector<std::string> categories;
};

static ConstantsDB g_constants_db;

static json load_json_file_or_throw(const std::string& path) {
    std::ifstream fin(path);
    if (!fin) {
        throw std::runtime_error("constants module: cannot open JSON file: " + path);
    }
    json j;
    fin >> j;
    return j;
}

static void load_constants_db() {
    if (g_constants_db.loaded) return;

    const std::string paths[] = {
        "wof_constants_module.json",
        "./wof_constants_module.json",
        "./data/wof_constants_module.json"
    };

    json root;
    bool loaded = false;
    for (const auto& p : paths) {
        try {
            root = load_json_file_or_throw(p);
            loaded = true;
            break;
        } catch (...) {
            // try next
        }
    }

    if (!loaded) {
        throw std::runtime_error("constants module: cannot find wof_constants_module.json");
    }

    if (root.contains("metadata") && root["metadata"].contains("categories")) {
        auto& cats = root["metadata"]["categories"];
        if (cats.is_array()) {
            for (const auto& c : cats) {
                if (c.is_string()) {
                    g_constants_db.categories.push_back(c.get<std::string>());
                }
            }
        }
    }

    if (!root.contains("constants") || !root["constants"].is_object()) {
        throw std::runtime_error("constants module: missing 'constants' in JSON");
    }

    for (auto& cat : root["constants"].items()) {
        const std::string& category = cat.key();
        const json& arr = cat.value();
        if (!arr.is_array()) continue;

        for (const auto& c : arr) {
            ConstantEntry entry;
            entry.category = category;
            entry.name     = c.value("name", "");
            entry.symbol   = c.value("symbol", "");
            entry.opcode   = c.value("opcode", -1);
            entry.desc     = c.value("description", "");
            entry.unit     = c.value("unit", "");

            if (c.contains("value")) {
                if (c["value"].is_number()) {
                    entry.value.type  = ConstantValue::Type::Double;
                    entry.value.value = c["value"].get<double>();
                } else if (c["value"].is_string()) {
                    entry.value.type  = ConstantValue::Type::String;
                    entry.value.value = c["value"].get<std::string>();
                }
            }

            if (entry.name.empty() || entry.opcode < 0) {
                continue;
            }

            g_constants_db.byName[entry.name]   = entry;
            g_constants_db.byOpcode[entry.opcode] = entry;
        }
    }

    g_constants_db.loaded = true;
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

static int pop_int(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        throw std::runtime_error("stack underflow: expected int");
    }
    WofValue v = ip.stack.back();
    ip.stack.pop_back();
    if (v.type == woflang::WofType::Integer) {
        return static_cast<int>(std::get<int64_t>(v.value));
    }
    if (v.type == woflang::WofType::Double) {
        return static_cast<int>(std::get<double>(v.value));
    }
    throw std::runtime_error("expected numeric type for int");
}

static void push_double(WoflangInterpreter& ip, double d) {
    ip.push(WofValue::make_double(d));
}

static void push_string(WoflangInterpreter& ip, const std::string& s) {
    ip.push(WofValue::make_string(s));
}

static void op_const_by_name(WoflangInterpreter& ip) {
    load_constants_db();
    std::string name = pop_string(ip);
    auto it = g_constants_db.byName.find(name);
    if (it == g_constants_db.byName.end()) {
        push_string(ip, "!NOT_FOUND: " + name);
        return;
    }
    const ConstantEntry& c = it->second;
    if (c.value.type == ConstantValue::Type::Double) {
        push_double(ip, std::get<double>(c.value.value));
    } else {
        push_string(ip, std::get<std::string>(c.value.value));
    }
}

static void op_const_by_opcode(WoflangInterpreter& ip) {
    load_constants_db();
    int code = pop_int(ip);
    auto it = g_constants_db.byOpcode.find(code);
    if (it == g_constants_db.byOpcode.end()) {
        push_string(ip, "!NOT_FOUND: opcode " + std::to_string(code));
        return;
    }
    const ConstantEntry& c = it->second;
    if (c.value.type == ConstantValue::Type::Double) {
        push_double(ip, std::get<double>(c.value.value));
    } else {
        push_string(ip, std::get<std::string>(c.value.value));
    }
}

static void op_const_categories(WoflangInterpreter& ip) {
    load_constants_db();
    std::string cats = "Categories: ";
    for (const auto& c : g_constants_db.categories) {
        cats += c + ", ";
    }
    if (cats.size() > std::string("Categories: ").size()) {
        // Remove trailing ", "
        cats.erase(cats.size() - 2);
    }
    push_string(ip, cats);
}

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("const_by_name", [](WoflangInterpreter& ip) {
        try {
            op_const_by_name(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("const_by_name error: ") + e.what()));
        }
    });

    interp.register_op("const_by_opcode", [](WoflangInterpreter& ip) {
        try {
            op_const_by_opcode(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("const_by_opcode error: ") + e.what()));
        }
    });

    interp.register_op("const_categories", [](WoflangInterpreter& ip) {
        try {
            op_const_categories(ip);
        } catch (const std::exception& e) {
            ip.push(WofValue::make_string(std::string("const_categories error: ") + e.what()));
        }
    });

    std::cout << "[wof_constants_module] registered const_by_name, const_by_opcode, const_categories ops\n";
}
