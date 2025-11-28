#pragma once

#include <cstdint>
#include <string>
#include <variant>
#include <vector>
#include <unordered_map>
#include <filesystem>
#include <functional>
#include <memory>
#include <stdexcept>

#ifdef _WIN32
#  include <windows.h>
#endif

namespace woflang {

// Optional unit metadata used by some plugins (chemistry, etc.)
struct Unit {
    std::string name;
    double      scale{1.0};
};

enum class WofType {
    Unknown,
    Integer,
    Double,
    String,
    Symbol
};

struct WofValue {
    WofType type{WofType::Unknown};
    std::variant<std::monostate, std::int64_t, double, std::string> value{};
    std::shared_ptr<Unit> unit{};

    static WofValue make_int(std::int64_t v);
    static WofValue make_double(double v);
    static WofValue make_string(const std::string& s);
    static WofValue make_symbol(const std::string& s);

    std::string to_string() const;
    bool is_numeric() const;
    double as_numeric() const;

    bool operator==(const WofValue& other) const;
};

class WoflangInterpreter;

using WofOpHandler = std::function<void(WoflangInterpreter&)>;

#ifdef _WIN32
using PluginHandle = HMODULE;
#else
using PluginHandle = void*;
#endif

class WoflangPlugin {
public:
    virtual ~WoflangPlugin() = default;
    virtual void register_ops(WoflangInterpreter& interp) = 0;
};

class WoflangInterpreter {
public:
    WoflangInterpreter();
    ~WoflangInterpreter();

    // Public stack: legacy plugins (e.g. hebrew_ops) directly access interp.stack.
    std::vector<WofValue> stack;

    // Stack helpers used by plugins and core
    void push(const WofValue& v);
    bool stack_has(std::size_t n) const;
    const std::vector<WofValue>& get_stack() const;

    WofValue pop();
    std::int64_t pop_int();
    double pop_double();
    double pop_numeric();
    std::string pop_string();
    std::string pop_symbol();
    bool pop_bool();

    void print_stack() const;
    void clear_stack();

    // Op registration and execution
    void register_op(const std::string& name, WofOpHandler handler);
    void dispatch_token(const std::string& token);
    void exec_line(const std::string& line);
    void exec_script(const std::filesystem::path& filename);
    void repl();

    // Plugin loading
    void load_plugin(const std::filesystem::path& path);
    void load_plugins(const std::filesystem::path& dir);

    // Simple error helper used by core and plugins
    [[noreturn]] void error(const std::string& msg) const {
        throw std::runtime_error(msg);
    }

    const std::unordered_map<std::string, WofOpHandler>& get_ops() const {
        return ops;
    }

private:
    std::unordered_map<std::string, WofOpHandler> ops;
    std::vector<PluginHandle>                     plugin_handles;
    std::vector<std::unique_ptr<WoflangPlugin>>   plugin_objects; // for old-style create_plugin
};

#ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
#else
#  define WOFLANG_PLUGIN_EXPORT
#endif

} // namespace woflang
