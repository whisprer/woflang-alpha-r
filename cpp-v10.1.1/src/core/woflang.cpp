#include "woflang.hpp"

#include <cctype>
#include <cmath>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>

#ifndef _WIN32
#  include <dlfcn.h>
#endif

namespace woflang {

// ======== WofValue implementation ========

WofValue WofValue::make_int(std::int64_t v) {
    WofValue w;
    w.type  = WofType::Integer;
    w.value = v;
    return w;
}

WofValue WofValue::make_double(double v) {
    WofValue w;
    w.type  = WofType::Double;
    w.value = v;
    return w;
}

WofValue WofValue::make_string(const std::string& s) {
    WofValue w;
    w.type  = WofType::String;
    w.value = s;
    return w;
}

WofValue WofValue::make_symbol(const std::string& s) {
    WofValue w;
    w.type  = WofType::Symbol;
    w.value = s;
    return w;
}

bool WofValue::operator==(const WofValue& other) const {
    if (type != other.type) {
        return false;
    }
    if (unit && other.unit) {
        if (unit->name != other.unit->name || unit->scale != other.unit->scale) {
            return false;
        }
    } else if (unit || other.unit) {
        return false;
    }

    switch (type) {
        case WofType::Unknown:
            return true;
        case WofType::Integer:
            return std::get<std::int64_t>(value) == std::get<std::int64_t>(other.value);
        case WofType::Double:
            return std::get<double>(value) == std::get<double>(other.value);
        case WofType::String:
        case WofType::Symbol:
            return std::get<std::string>(value) == std::get<std::string>(other.value);
    }
    return false;
}

std::string WofValue::to_string() const {
    switch (type) {
        case WofType::Unknown:
            return "<unknown>";
        case WofType::Integer:
            return std::to_string(std::get<std::int64_t>(value));
        case WofType::Double: {
            std::ostringstream oss;
            oss << std::get<double>(value);
            return oss.str();
        }
        case WofType::String:
            return "\"" + std::get<std::string>(value) + "\"";
        case WofType::Symbol:
            return std::get<std::string>(value);
    }
    return "<invalid>";
}

bool WofValue::is_numeric() const {
    return type == WofType::Integer || type == WofType::Double;
}

double WofValue::as_numeric() const {
    switch (type) {
        case WofType::Integer:
            return static_cast<double>(std::get<std::int64_t>(value));
        case WofType::Double:
            return std::get<double>(value);
        case WofType::String:
        case WofType::Symbol:
            // In numeric contexts, non-numeric values are treated as 0.0
            return 0.0;
        case WofType::Unknown:
            return 0.0;
    }
    return 0.0;
}

// ======== Internal helpers (anonymous namespace) ========

namespace {

bool is_number(const std::string& token) {
    if (token.empty()) {
        return false;
    }
    char* endptr = nullptr;
    std::strtod(token.c_str(), &endptr);
    return endptr == token.c_str() + token.size();
}

bool is_integer(const std::string& token) {
    if (token.empty()) {
        return false;
    }
    std::size_t i = 0;
    if (token[0] == '-' || token[0] == '+') {
        i = 1;
    }
    if (i == token.size()) {
        return false;
    }
    for (; i < token.size(); ++i) {
        if (!std::isdigit(static_cast<unsigned char>(token[i]))) {
            return false;
        }
    }
    return true;
}

std::vector<std::string> simple_tokenize(const std::string& line) {
    std::vector<std::string> tokens;
    std::string              current;
    bool                     in_string = false;

    for (char ch : line) {
        if (in_string) {
            if (ch == '"') {
                in_string = false;
                tokens.push_back(current);
                current.clear();
            } else {
                current.push_back(ch);
            }
        } else {
            if (std::isspace(static_cast<unsigned char>(ch))) {
                if (!current.empty()) {
                    tokens.push_back(current);
                    current.clear();
                }
            } else if (ch == '"') {
                if (!current.empty()) {
                    tokens.push_back(current);
                    current.clear();
                }
                in_string = true;
            } else {
                current.push_back(ch);
            }
        }
    }

    if (!current.empty()) {
        tokens.push_back(current);
    }

    return tokens;
}

} // anonymous namespace

// ======== WoflangInterpreter implementation ========

WoflangInterpreter::WoflangInterpreter() {
    // Core arithmetic operators
    register_op("+", [](WoflangInterpreter& interp) {
        if (!interp.stack_has(2)) {
            throw std::runtime_error("Stack underflow for +");
        }
        WofValue b = interp.pop();
        WofValue a = interp.pop();

        if (a.is_numeric() && b.is_numeric()) {
            double res = a.as_numeric() + b.as_numeric();
            interp.push(WofValue::make_double(res));
        } else {
            std::string sa = a.to_string();
            std::string sb = b.to_string();
            interp.push(WofValue::make_string(sa + sb));
        }
    });

    register_op("-", [](WoflangInterpreter& interp) {
        if (!interp.stack_has(2)) {
            throw std::runtime_error("Stack underflow for -");
        }
        WofValue b = interp.pop();
        WofValue a = interp.pop();
        double   res = a.as_numeric() - b.as_numeric();
        interp.push(WofValue::make_double(res));
    });

    register_op("*", [](WoflangInterpreter& interp) {
        if (!interp.stack_has(2)) {
            throw std::runtime_error("Stack underflow for *");
        }
        WofValue b = interp.pop();
        WofValue a = interp.pop();
        double   res = a.as_numeric() * b.as_numeric();
        interp.push(WofValue::make_double(res));
    });

    register_op("/", [](WoflangInterpreter& interp) {
        if (!interp.stack_has(2)) {
            throw std::runtime_error("Stack underflow for /");
        }
        WofValue b = interp.pop();
        WofValue a = interp.pop();
        double   denom = b.as_numeric();
        if (denom == 0.0) {
            throw std::runtime_error("Division by zero");
        }
        double res = a.as_numeric() / denom;
        interp.push(WofValue::make_double(res));
    });

    // Stack inspection helper
    register_op(".s", [](WoflangInterpreter& interp) {
        interp.print_stack();
    });
}

WoflangInterpreter::~WoflangInterpreter() {
#ifdef _WIN32
    for (auto handle : plugin_handles) {
        if (handle) {
            FreeLibrary(handle);
        }
    }
#else
    for (auto handle : plugin_handles) {
        if (handle) {
            dlclose(handle);
        }
    }
#endif
}

void WoflangInterpreter::push(const WofValue& v) {
    stack.push_back(v);
}

bool WoflangInterpreter::stack_has(std::size_t n) const {
    return stack.size() >= n;
}

const std::vector<WofValue>& WoflangInterpreter::get_stack() const {
    return stack;
}

WofValue WoflangInterpreter::pop() {
    if (stack.empty()) {
        throw std::runtime_error("Stack underflow in pop()");
    }
    WofValue v = stack.back();
    stack.pop_back();
    return v;
}

std::int64_t WoflangInterpreter::pop_int() {
    WofValue v = pop();
    if (v.type == WofType::Integer) {
        return std::get<std::int64_t>(v.value);
    }
    if (v.type == WofType::Double) {
        return static_cast<std::int64_t>(std::get<double>(v.value));
    }
    throw std::runtime_error("pop_int: value is not numeric");
}

double WoflangInterpreter::pop_double() {
    WofValue v = pop();
    return v.as_numeric();
}

double WoflangInterpreter::pop_numeric() {
    WofValue v = pop();
    return v.as_numeric();
}

std::string WoflangInterpreter::pop_string() {
    WofValue v = pop();
    if (v.type == WofType::String || v.type == WofType::Symbol) {
        return std::get<std::string>(v.value);
    }
    return v.to_string();
}

std::string WoflangInterpreter::pop_symbol() {
    WofValue v = pop();
    if (v.type == WofType::Symbol) {
        return std::get<std::string>(v.value);
    }
    throw std::runtime_error("pop_symbol: value is not a symbol");
}

bool WoflangInterpreter::pop_bool() {
    WofValue v = pop();
    if (v.type == WofType::Integer) {
        return std::get<std::int64_t>(v.value) != 0;
    }
    if (v.type == WofType::Double) {
        return std::get<double>(v.value) != 0.0;
    }
    if (v.type == WofType::String || v.type == WofType::Symbol) {
        const auto& s = std::get<std::string>(v.value);
        return !s.empty() && s != "0" && s != "false";
    }
    return false;
}

// ========== Op registration and execution ==========

void WoflangInterpreter::register_op(const std::string& name, WofOpHandler handler) {
    ops[name] = std::move(handler);
}

void WoflangInterpreter::dispatch_token(const std::string& token) {
    // String literal?
    if (!token.empty() && token.front() == '"' && token.back() == '"' && token.size() >= 2) {
        std::string inner = token.substr(1, token.size() - 2);
        push(WofValue::make_string(inner));
        return;
    }

    // Number?
    if (is_integer(token)) {
        std::int64_t iv = std::stoll(token);
        push(WofValue::make_int(iv));
        return;
    }
    if (is_number(token)) {
        double dv = std::stod(token);
        push(WofValue::make_double(dv));
        return;
    }

    // Registered op?
    auto it = ops.find(token);
    if (it != ops.end()) {
        it->second(*this);
        return;
    }

    // Otherwise treat as symbol
    push(WofValue::make_symbol(token));
}

void WoflangInterpreter::exec_line(const std::string& line) {
    auto tokens = simple_tokenize(line);
    for (const auto& t : tokens) {
        dispatch_token(t);
    }
}

void WoflangInterpreter::exec_script(const std::filesystem::path& filename) {
    std::ifstream in(filename);
    if (!in) {
        throw std::runtime_error("Cannot open script: " + filename.string());
    }
    std::string line;
    while (std::getline(in, line)) {
        exec_line(line);
    }
}

// ========== Plugin loading (new + legacy) ==========

void WoflangInterpreter::load_plugin(const std::filesystem::path& path) {
#ifdef _WIN32
    HMODULE handle = LoadLibraryW(path.wstring().c_str());
    if (!handle) {
        throw std::runtime_error("Failed to load plugin: " + path.string());
    }

    using RegisterPluginFunc = void (*)(WoflangInterpreter&);
    using CreatePluginFunc   = WoflangPlugin* (*)();

    FARPROC raw_reg    = GetProcAddress(handle, "register_plugin");
    FARPROC raw_create = nullptr;

    if (raw_reg) {
        auto register_func = reinterpret_cast<RegisterPluginFunc>(raw_reg);
        register_func(*this);
    } else {
        raw_create = GetProcAddress(handle, "create_plugin");
        if (!raw_create) {
            FreeLibrary(handle);
            throw std::runtime_error("No register_plugin/create_plugin in plugin: " + path.string());
        }
        auto create_func = reinterpret_cast<CreatePluginFunc>(raw_create);
        std::unique_ptr<WoflangPlugin> plugin(create_func());
        plugin->register_ops(*this);
        plugin_objects.push_back(std::move(plugin));
    }

    plugin_handles.push_back(handle);

#else
    void* handle = dlopen(path.string().c_str(), RTLD_NOW);
    if (!handle) {
        throw std::runtime_error(std::string("Failed to load plugin: ") + dlerror());
    }

    dlerror(); // clear
    using RegisterPluginFunc = void (*)(WoflangInterpreter&);
    using CreatePluginFunc   = WoflangPlugin* (*)();

    auto raw_reg = dlsym(handle, "register_plugin");
    const char* err = dlerror();

    if (!err && raw_reg) {
        auto register_func = reinterpret_cast<RegisterPluginFunc>(raw_reg);
        register_func(*this);
    } else {
        dlerror(); // clear
        auto raw_create = dlsym(handle, "create_plugin");
        err = dlerror();
        if (err || !raw_create) {
            dlclose(handle);
            throw std::runtime_error(std::string("No register_plugin/create_plugin in plugin: ") + path.string());
        }
        auto create_func = reinterpret_cast<CreatePluginFunc>(raw_create);
        std::unique_ptr<WoflangPlugin> plugin(create_func());
        plugin->register_ops(*this);
        plugin_objects.push_back(std::move(plugin));
    }

    plugin_handles.push_back(handle);
#endif
}

void WoflangInterpreter::load_plugins(const std::filesystem::path& dir) {
    namespace fs = std::filesystem;
    if (!fs::exists(dir) || !fs::is_directory(dir)) {
        return;
    }

    for (const auto& entry : fs::directory_iterator(dir)) {
        if (!entry.is_regular_file()) {
            continue;
        }
#ifdef _WIN32
        if (entry.path().extension() == ".dll") {
            load_plugin(entry.path());
        }
#else
        if (entry.path().extension() == ".so" || entry.path().extension() == ".dylib") {
            load_plugin(entry.path());
        }
#endif
    }
}

// ========== REPL and stack display ==========

void WoflangInterpreter::repl() {
    std::cout << "Welcome to woflang. Type 'quit' to exit.\n";
    std::string line;
    while (true) {
        std::cout << "wof> ";
        if (!std::getline(std::cin, line)) {
            break;
        }
        if (line == "quit") {
            break;
        }
        if (line == "clear") {
            clear_stack();
            continue;
        }
        if (line == "show") {
            print_stack();
            continue;
        }
        try {
            exec_line(line);
        } catch (const std::exception& ex) {
            std::cout << "Error: " << ex.what() << '\n';
        }
    }
}

void WoflangInterpreter::print_stack() const {
    if (stack.empty()) {
        std::cout << "[stack is empty]\n";
        return;
    }

    std::cout << "Stack (top → bottom):\n";
    for (auto it = stack.rbegin(); it != stack.rend(); ++it) {
        std::cout << "  - " << it->to_string() << '\n';
    }
}

void WoflangInterpreter::clear_stack() {
    stack.clear();
}

} // namespace woflang
