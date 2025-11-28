// ===============================================================
// category_theory_ops.cpp
// Tiny in-memory category playground for WofLang
//
// Provides basic operations to define and inspect a small category:
//   - cat_obj          : declare an object
//   - cat_mor         : declare a morphism
//   - cat_comp        : compose two morphisms (if composable)
//   - cat_hom         : list hom-set Hom(A, B) as a string
//   - cat_show        : summary of current category
//   - cat_clear       : clear all objects and morphisms
//
// Stack conventions:
//
//   cat_obj
//     "A"          --         (register object A)
//
//   cat_mor
//     "A" "B" "f"  --         (register morphism f : A -> B)
//
//   cat_comp
//     "g" "f"      -- "g ∘ f" (if cod(f) == dom(g))
//
//   cat_hom
//     "A" "B"      -- "Hom(A,B) = { ... }"
//
//   cat_show
//     (nothing)    -- summary string
//
//   cat_clear
//     (nothing)    -- (clears internal registry)
//
// ===============================================================

#include "woflang.hpp"

#include <algorithm>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using namespace woflang;

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT extern "C" __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT extern "C"
# endif
#endif

namespace {

// ---------------------------------------------------------------
// Internal state for a tiny category
// ---------------------------------------------------------------
struct Morphism {
    std::string name;
    std::string from;
    std::string to;
};

struct CategoryState {
    std::vector<std::string> objects;
    std::vector<Morphism>    morphisms;

    bool has_object(const std::string& obj) const {
        return std::find(objects.begin(), objects.end(), obj) != objects.end();
    }

    void add_object(const std::string& obj) {
        if (!has_object(obj)) {
            objects.push_back(obj);
        }
    }

    const Morphism* find_morphism(const std::string& name) const {
        for (const auto& m : morphisms) {
            if (m.name == name) {
                return &m;
            }
        }
        return nullptr;
    }

    void add_morphism(const std::string& name,
                      const std::string& from,
                      const std::string& to) {
        // ensure objects exist
        add_object(from);
        add_object(to);

        // overwrite if same name already exists
        for (auto& m : morphisms) {
            if (m.name == name) {
                m.from = from;
                m.to   = to;
                return;
            }
        }
        morphisms.push_back(Morphism{name, from, to});
    }

    std::vector<std::string> hom(const std::string& from,
                                 const std::string& to) const {
        std::vector<std::string> result;
        for (const auto& m : morphisms) {
            if (m.from == from && m.to == to) {
                result.push_back(m.name);
            }
        }
        return result;
    }

    void clear() {
        objects.clear();
        morphisms.clear();
    }

    std::string summary() const {
        std::ostringstream oss;
        oss << "Category summary:\n";

        oss << "Objects: ";
        if (objects.empty()) {
            oss << "{}\n";
        } else {
            oss << "{ ";
            for (size_t i = 0; i < objects.size(); ++i) {
                if (i > 0) oss << ", ";
                oss << objects[i];
            }
            oss << " }\n";
        }

        oss << "Morphisms:\n";
        if (morphisms.empty()) {
            oss << "  (none)\n";
        } else {
            for (const auto& m : morphisms) {
                oss << "  " << m.name << " : " << m.from << " -> " << m.to << "\n";
            }
        }
        return oss.str();
    }
};

// One global category sandbox for the plugin
static CategoryState g_cat;

// ---------------------------------------------------------------
// Stack helpers
// ---------------------------------------------------------------
bool expect_string_top(WoflangInterpreter& interp,
                       std::string& out,
                       const char* op_name,
                       const char* what) {
    auto& st = interp.stack;
    if (st.empty()) {
        std::cout << op_name << ": need " << what << " (string)\n";
        return false;
    }
    WofValue v = st.back();
    st.pop_back();

    if (v.type != WofType::String) {
        std::cout << op_name << ": " << what << " must be a string\n";
        return false;
    }
    out = std::get<std::string>(v.value);
    return true;
}

} // namespace

// ---------------------------------------------------------------
// Plugin entry point (v10 API)
// ---------------------------------------------------------------

WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // -----------------------------------------------------------
    // cat_obj : "A" -- ()
    // -----------------------------------------------------------
    interp.register_op("cat_obj", [](WoflangInterpreter& interp) {
        std::string obj;
        if (!expect_string_top(interp, obj, "cat_obj", "object name")) {
            return;
        }
        g_cat.add_object(obj);
        std::cout << "[category_theory] added object: " << obj << "\n";
    });

    // -----------------------------------------------------------
    // cat_mor : "A" "B" "f" -- ()
    // defines f : A -> B
    // -----------------------------------------------------------
    interp.register_op("cat_mor", [](WoflangInterpreter& interp) {
        auto& st = interp.stack;
        if (st.size() < 3) {
            std::cout << "cat_mor: need 3 strings (from, to, name)\n";
            return;
        }

        std::string name;
        std::string to;
        std::string from;

        // pop in reverse order (top to bottom)
        if (!expect_string_top(interp, name, "cat_mor", "morphism name")) return;
        if (!expect_string_top(interp, to,   "cat_mor", "codomain object")) return;
        if (!expect_string_top(interp, from, "cat_mor", "domain object"))   return;

        g_cat.add_morphism(name, from, to);
        std::cout << "[category_theory] added morphism: "
                  << name << " : " << from << " -> " << to << "\n";
    });

    // -----------------------------------------------------------
    // cat_comp : "g" "f" -- "g ∘ f"
    // composition valid iff cod(f) == dom(g)
    // -----------------------------------------------------------
    interp.register_op("cat_comp", [](WoflangInterpreter& interp) {
        std::string f_name;
        std::string g_name;

        if (!expect_string_top(interp, g_name, "cat_comp", "g (second morphism)")) return;
        if (!expect_string_top(interp, f_name, "cat_comp", "f (first morphism)"))  return;

        const Morphism* f = g_cat.find_morphism(f_name);
        const Morphism* g = g_cat.find_morphism(g_name);

        if (!f || !g) {
            std::cout << "cat_comp: unknown morphism(s): "
                      << f_name << ", " << g_name << "\n";
            return;
        }

        if (f->to != g->from) {
            std::cout << "cat_comp: cannot compose "
                      << g_name << " ∘ " << f_name
                      << " because cod(f) = " << f->to
                      << " != dom(g) = " << g->from << "\n";
            return;
        }

        std::string comp_name = g_name + " ∘ " + f_name;

        // Explicit WofValue instead of implicit emplace_back
        WofValue out;
        out.type  = WofType::String;
        out.value = comp_name;
        interp.stack.push_back(out);
    });

    // -----------------------------------------------------------
    // cat_hom : "A" "B" -- "Hom(A,B) = {...}"
    // -----------------------------------------------------------
    interp.register_op("cat_hom", [](WoflangInterpreter& interp) {
        std::string to;
        std::string from;

        if (!expect_string_top(interp, to,   "cat_hom", "codomain object")) return;
        if (!expect_string_top(interp, from, "cat_hom", "domain object"))   return;

        auto homset = g_cat.hom(from, to);

        std::ostringstream oss;
        oss << "Hom(" << from << "," << to << ") = {";
        for (size_t i = 0; i < homset.size(); ++i) {
            if (i > 0) oss << ", ";
            oss << homset[i];
        }
        oss << "}";

        WofValue out;
        out.type  = WofType::String;
        out.value = oss.str();
        interp.stack.push_back(out);
    });

    // -----------------------------------------------------------
    // cat_show : -- summary-string
    // -----------------------------------------------------------
    interp.register_op("cat_show", [](WoflangInterpreter& interp) {
        std::string s = g_cat.summary();

        WofValue out;
        out.type  = WofType::String;
        out.value = s;
        interp.stack.push_back(out);
    });

    // -----------------------------------------------------------
    // cat_clear : -- ()
    // -----------------------------------------------------------
    interp.register_op("cat_clear", [](WoflangInterpreter& /*interp*/) {
        g_cat.clear();
        std::cout << "[category_theory] category cleared\n";
    });

    std::cout << "[category_theory_ops] Plugin loaded.\n";
}
