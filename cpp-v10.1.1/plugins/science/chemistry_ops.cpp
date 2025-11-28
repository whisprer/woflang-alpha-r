// chemistry_ops.cpp
//
// WofLang chemistry plugin for v10 API.
// Provides basic element data, atomic/molecular weights,
// temperature conversions, and Avogadro's constant.

#include "woflang.hpp"

#include <string>
#include <vector>
#include <cctype>
#include <sstream>
#include <iomanip>
#include <algorithm>
#include <iostream>
#include <cmath>    // for std::llround

using namespace woflang;

// Provide an export macro if core doesn't
#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

// --- Small periodic table subset -------------------------

struct ElementData {
    int         Z;
    const char* symbol;
    const char* name;
    double      atomic_weight; // g/mol
};

static const std::vector<ElementData> kElements = {
    {1,  "H",  "Hydrogen",      1.008},
    {2,  "He", "Helium",        4.002602},
    {3,  "Li", "Lithium",       6.94},
    {4,  "Be", "Beryllium",     9.0121831},
    {5,  "B",  "Boron",        10.81},
    {6,  "C",  "Carbon",       12.011},
    {7,  "N",  "Nitrogen",     14.007},
    {8,  "O",  "Oxygen",       15.999},
    {9,  "F",  "Fluorine",     18.998403163},
    {10, "Ne", "Neon",         20.1797},
    {11, "Na", "Sodium",       22.98976928},
    {12, "Mg", "Magnesium",    24.305},
    {13, "Al", "Aluminium",    26.9815385},
    {14, "Si", "Silicon",      28.085},
    {15, "P",  "Phosphorus",   30.973761998},
    {16, "S",  "Sulfur",       32.06},
    {17, "Cl", "Chlorine",     35.45},
    {18, "Ar", "Argon",        39.948},
    {19, "K",  "Potassium",    39.0983},
    {20, "Ca", "Calcium",      40.078},
    {26, "Fe", "Iron",         55.845},
    {29, "Cu", "Copper",       63.546},
    {30, "Zn", "Zinc",         65.38},
    {47, "Ag", "Silver",      107.8682},
    {79, "Au", "Gold",        196.966569},
    {82, "Pb", "Lead",        207.2}
};

static std::string to_lower(std::string s) {
    std::transform(s.begin(), s.end(), s.begin(),
                   [](unsigned char c) { return static_cast<char>(std::tolower(c)); });
    return s;
}

static const ElementData* find_element_by_symbol(const std::string& sym) {
    for (const auto& e : kElements) {
        if (sym == e.symbol) return &e;
    }
    return nullptr;
}

static const ElementData* find_element_by_name(const std::string& name) {
    auto needle = to_lower(name);
    for (const auto& e : kElements) {
        if (needle == to_lower(e.name)) return &e;
    }
    return nullptr;
}

static const ElementData* find_element(const std::string& token) {
    // Try symbol first (case-sensitive, normal chemistry style)
    if (const auto* e = find_element_by_symbol(token)) {
        return e;
    }
    // Try name (case-insensitive)
    if (const auto* e = find_element_by_name(token)) {
        return e;
    }
    // Try "h", "hydrogen", etc.
    if (!token.empty()) {
        std::string sym;
        sym.push_back(static_cast<char>(std::toupper(static_cast<unsigned char>(token[0]))));
        if (token.size() >= 2) {
            sym.push_back(static_cast<char>(std::tolower(static_cast<unsigned char>(token[1]))));
        }
        if (const auto* e = find_element_by_symbol(sym)) {
            return e;
        }
    }
    return nullptr;
}

// --- Helpers for working with WofValue -------------------

static bool is_string(const WofValue& v) {
    return v.type == WofType::String;
}

static bool is_numeric(const WofValue& v) {
    return v.type == WofType::Integer || v.type == WofType::Double;
}

static std::string as_string(const WofValue& v) {
    if (v.type == WofType::String) {
        return std::get<std::string>(v.value);
    }
    // Fallback: numeric to string
    if (v.type == WofType::Integer) {
        return std::to_string(std::get<int64_t>(v.value));
    }
    if (v.type == WofType::Double) {
        std::ostringstream oss;
        oss << std::get<double>(v.value);
        return oss.str();
    }
    return "";
}

// Kept for potential future use
static double as_double(const WofValue& v) {
    // Use core helper for robust numeric conversion
    return v.as_numeric();
}

// Kept for potential future use (just a logging helper now)
static void ensure_stack_size(WoflangInterpreter& ip,
                              std::size_t n,
                              const char* op_name)
{
    if (ip.stack.size() < n) {
        std::cerr << "[chemistry_ops] op \"" << op_name
                  << "\": not enough values on stack (need "
                  << n << ", have " << ip.stack.size() << ")\n";
    }
}

// --- Operations ------------------------------------------

// element_info: (key -> description-string)
// key can be: symbol ("H"), full name ("Hydrogen"), or atomic number (1, 6, ...).
static void op_element_info(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        std::cerr << "[chemistry_ops] op \"element_info\": stack is empty\n";
        ip.stack.emplace_back(WofValue::make_string("error: empty stack"));
        return;
    }

    WofValue key_val = ip.stack.back();
    ip.stack.pop_back();

    const ElementData* elem = nullptr;

    if (is_string(key_val)) {
        std::string token = as_string(key_val);
        elem = find_element(token);
    } else if (is_numeric(key_val)) {
        int Z = static_cast<int>(std::llround(key_val.as_numeric()));
        for (const auto& e : kElements) {
            if (e.Z == Z) { elem = &e; break; }
        }
    }

    std::ostringstream oss;
    if (!elem) {
        oss << "Unknown element: " << as_string(key_val);
    } else {
        oss << elem->name << " (" << elem->symbol << "), Z = " << elem->Z
            << ", atomic weight ≈ " << std::fixed << std::setprecision(5)
            << elem->atomic_weight << " g/mol";
    }

    ip.stack.emplace_back(WofValue::make_string(oss.str()));
}

// atomic_weight: (key -> atomic_weight)
// key as above.
static void op_atomic_weight(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        std::cerr << "[chemistry_ops] op \"atomic_weight\": stack is empty\n";
        ip.stack.emplace_back(WofValue::make_double(0.0));
        return;
    }

    WofValue key_val = ip.stack.back();
    ip.stack.pop_back();

    const ElementData* elem = nullptr;

    if (is_string(key_val)) {
        elem = find_element(as_string(key_val));
    } else if (is_numeric(key_val)) {
        int Z = static_cast<int>(std::llround(key_val.as_numeric()));
        for (const auto& e : kElements) {
            if (e.Z == Z) { elem = &e; break; }
        }
    }

    double weight = elem ? elem->atomic_weight : 0.0;
    if (!elem) {
        std::cerr << "[chemistry_ops] atomic_weight: unknown element: "
                  << as_string(key_val) << "\n";
    }
    ip.stack.emplace_back(WofValue::make_double(weight));
}

// molecular_weight: (formula-string -> molar-mass)
// Very simple parser: supports e.g. "H2O", "CO2", "C6H12O6".
static void op_molecular_weight(WoflangInterpreter& ip) {
    if (ip.stack.empty()) {
        std::cerr << "[chemistry_ops] op \"molecular_weight\": stack is empty\n";
        ip.stack.emplace_back(WofValue::make_double(0.0));
        return;
    }

    WofValue formula_val = ip.stack.back();
    ip.stack.pop_back();

    std::string formula = as_string(formula_val);
    if (formula.empty()) {
        ip.stack.emplace_back(WofValue::make_double(0.0));
        return;
    }

    double total = 0.0;
    std::size_t i = 0;
    const std::size_t n = formula.size();

    while (i < n) {
        if (!std::isalpha(static_cast<unsigned char>(formula[i]))) {
            std::cerr << "[chemistry_ops] molecular_weight: invalid char '"
                      << formula[i] << "' in formula \"" << formula << "\"\n";
            ip.stack.emplace_back(WofValue::make_double(0.0));
            return;
        }

        // Element symbol: capital + optional lowercase
        std::string sym;
        sym.push_back(static_cast<char>(std::toupper(
            static_cast<unsigned char>(formula[i])
        )));
        ++i;
        if (i < n && std::islower(static_cast<unsigned char>(formula[i]))) {
            sym.push_back(static_cast<char>(std::tolower(
                static_cast<unsigned char>(formula[i])
            )));
            ++i;
        }

        const ElementData* elem = find_element_by_symbol(sym);
        if (!elem) {
            std::cerr << "[chemistry_ops] molecular_weight: unknown symbol \""
                      << sym << "\" in \"" << formula << "\"\n";
            ip.stack.emplace_back(WofValue::make_double(0.0));
            return;
        }

        // Optional numeric count
        int count = 0;
        while (i < n && std::isdigit(static_cast<unsigned char>(formula[i]))) {
            count = count * 10 + (formula[i] - '0');
            ++i;
        }
        if (count == 0) count = 1;

        total += elem->atomic_weight * static_cast<double>(count);
    }

    ip.stack.emplace_back(WofValue::make_double(total));
}

// temp_convert:
// Supports: C->K, K->C, C->F, F->C, K->F, F->K, with a few alias spellings.
//
// Stack patterns (most flexible):
//   <value> <mode-string> temp_convert
// or
//   <mode-string> <value> temp_convert
//
// where mode-string examples: "C->F", "F_to_C", "K2C", case-insensitive.
static void op_temp_convert(WoflangInterpreter& ip) {
    if (ip.stack.size() < 2) {
        std::cerr << "[chemistry_ops] op \"temp_convert\": need value and mode\n";
        return;
    }

    // Try to detect which of the top 2 is the mode.
    WofValue top  = ip.stack.back(); ip.stack.pop_back();
    WofValue next = ip.stack.back(); ip.stack.pop_back();

    std::string mode;
    WofValue value_val;

    if (is_string(top) && is_numeric(next)) {
        mode = as_string(top);
        value_val = next;
    } else if (is_string(next) && is_numeric(top)) {
        mode = as_string(next);
        value_val = top;
    } else if (is_string(top) && is_string(next)) {
        // Weird, but pick top as mode and try to parse next as number.
        mode = as_string(top);
        try {
            double parsed = std::stod(as_string(next));
            value_val = WofValue::make_double(parsed);
        } catch (...) {
            std::cerr << "[chemistry_ops] temp_convert: cannot parse numeric value\n";
            ip.stack.emplace_back(WofValue::make_double(0.0));
            return;
        }
    } else {
        // Fallback: treat 'top' as mode-string-converted, next as numeric
        mode = as_string(top);
        value_val = next;
    }

    std::string m = to_lower(mode);
    // Normalize shorthand
    auto normalize = [](std::string s) {
        for (auto& ch : s) {
            if (ch == ' ' || ch == '_') ch = '>';
            if (ch == '2') ch = '>';
        }
        return s;
    };
    m = normalize(m);

    double t = value_val.as_numeric();
    double result = t;

    auto starts_with = [](const std::string& s, const char* pfx) {
        return s.rfind(pfx, 0) == 0;
    };

    if (starts_with(m, "c>k")) {          // Celsius -> Kelvin
        result = t + 273.15;
    } else if (starts_with(m, "k>c")) {   // Kelvin -> Celsius
        result = t - 273.15;
    } else if (starts_with(m, "c>f")) {   // Celsius -> Fahrenheit
        result = t * 9.0 / 5.0 + 32.0;
    } else if (starts_with(m, "f>c")) {   // Fahrenheit -> Celsius
        result = (t - 32.0) * 5.0 / 9.0;
    } else if (starts_with(m, "k>f")) {   // Kelvin -> Fahrenheit
        result = (t - 273.15) * 9.0 / 5.0 + 32.0;
    } else if (starts_with(m, "f>k")) {   // Fahrenheit -> Kelvin
        result = (t - 32.0) * 5.0 / 9.0 + 273.15;
    } else {
        std::cerr << "[chemistry_ops] temp_convert: unknown mode \""
                  << mode << "\" — returning original value\n";
        // Put original back and bail.
        ip.stack.emplace_back(value_val);
        return;
    }

    ip.stack.emplace_back(WofValue::make_double(result));
}

// avogadro: ( -- N_A )
static void op_avogadro(WoflangInterpreter& ip) {
    constexpr double NA = 6.02214076e23; // exact definition
    ip.stack.emplace_back(WofValue::make_double(NA));
}

// --- Plugin registration (no WoflangPlugin base) --------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    interp.register_op("element_info",     [](WoflangInterpreter& ip) { op_element_info(ip);     });
    interp.register_op("atomic_weight",    [](WoflangInterpreter& ip) { op_atomic_weight(ip);    });
    interp.register_op("molecular_weight", [](WoflangInterpreter& ip) { op_molecular_weight(ip); });
    interp.register_op("temp_convert",     [](WoflangInterpreter& ip) { op_temp_convert(ip);     });
    interp.register_op("avogadro",         [](WoflangInterpreter& ip) { op_avogadro(ip);         });

    std::cout << "[chemistry_ops] Chemistry plugin loaded.\n";
}
