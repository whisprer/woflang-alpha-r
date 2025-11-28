// ======================================================
// crypto_ops.cpp - Cryptography and Encoding Operations
// (extended for Woflang v10.1.1 core API)
// ======================================================

#include <cstdint>
#include <cmath>
#include <iostream>
#include <random>
#include <stdexcept>
#include <string>
#include <vector>
#include <variant>

#include "woflang.hpp"

#ifndef WOFLANG_PLUGIN_EXPORT
# ifdef _WIN32
#  define WOFLANG_PLUGIN_EXPORT __declspec(dllexport)
# else
#  define WOFLANG_PLUGIN_EXPORT __attribute__((visibility("default")))
# endif
#endif

using woflang::WoflangInterpreter;
using woflang::WofValue;
using woflang::WofType;

// -------------------------- helpers --------------------------

namespace {

// numeric helper (same idea as your existing version)
std::int64_t to_int64(const WofValue& v, const char* ctx) {
    double d = v.as_numeric();
    if (!std::isfinite(d)) {
        throw std::runtime_error(std::string(ctx) + ": argument must be finite");
    }
    return static_cast<std::int64_t>(d);
}

double to_double(const WofValue& v, const char* ctx) {
    double d = v.as_numeric();
    if (!std::isfinite(d)) {
        throw std::runtime_error(std::string(ctx) + ": argument must be finite");
    }
    return d;
}

std::string to_string_value(const WofValue& v, const char* ctx) {
    if (v.type == WofType::String) {
        return std::get<std::string>(v.value);
    }
    // Fallback: stringify numeric values
    double d = v.as_numeric();
    if (!std::isfinite(d)) {
        throw std::runtime_error(std::string(ctx) + ": value must be finite or string");
    }
    return std::to_string(d);
}

// Simple deterministic trial-division primality test for 64-bit integers
bool is_prime_int64(std::int64_t n) {
    if (n <= 1) return false;
    if (n <= 3) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;

    // 6k ± 1 wheel
    for (std::int64_t i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) {
            return false;
        }
    }
    return true;
}

// RNG: single engine reused across calls
std::mt19937_64& rng_engine() {
    static std::mt19937_64 eng{std::random_device{}()};
    return eng;
}

// FNV-1a 64-bit hash
std::uint64_t fnv1a_64(const std::string& s) {
    const std::uint64_t FNV_OFFSET = 1469598103934665603ull;
    const std::uint64_t FNV_PRIME  = 1099511628211ull;

    std::uint64_t h = FNV_OFFSET;
    for (unsigned char c : s) {
        h ^= static_cast<std::uint64_t>(c);
        h *= FNV_PRIME;
    }
    return h;
}

// Base64 alphabet
const char* b64_alphabet() {
    return "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
}

std::string base64_encode(const std::string& input) {
    const unsigned char* data = reinterpret_cast<const unsigned char*>(input.data());
    const std::size_t len = input.size();
    const char* alphabet = b64_alphabet();

    std::string out;
    out.reserve(((len + 2) / 3) * 4);

    std::size_t i = 0;
    while (i + 3 <= len) {
        std::uint32_t n = (static_cast<std::uint32_t>(data[i]) << 16) |
                          (static_cast<std::uint32_t>(data[i + 1]) << 8) |
                          (static_cast<std::uint32_t>(data[i + 2]));
        i += 3;

        out.push_back(alphabet[(n >> 18) & 0x3F]);
        out.push_back(alphabet[(n >> 12) & 0x3F]);
        out.push_back(alphabet[(n >> 6)  & 0x3F]);
        out.push_back(alphabet[n & 0x3F]);
    }

    std::size_t rem = len - i;
    if (rem == 1) {
        std::uint32_t n = static_cast<std::uint32_t>(data[i]) << 16;
        out.push_back(alphabet[(n >> 18) & 0x3F]);
        out.push_back(alphabet[(n >> 12) & 0x3F]);
        out.push_back('=');
        out.push_back('=');
    } else if (rem == 2) {
        std::uint32_t n = (static_cast<std::uint32_t>(data[i]) << 16) |
                          (static_cast<std::uint32_t>(data[i + 1]) << 8);
        out.push_back(alphabet[(n >> 18) & 0x3F]);
        out.push_back(alphabet[(n >> 12) & 0x3F]);
        out.push_back(alphabet[(n >> 6)  & 0x3F]);
        out.push_back('=');
    }

    return out;
}

int b64_index(char c) {
    if ('A' <= c && c <= 'Z') return c - 'A';
    if ('a' <= c && c <= 'z') return c - 'a' + 26;
    if ('0' <= c && c <= '9') return c - '0' + 52;
    if (c == '+') return 62;
    if (c == '/') return 63;
    return -1;
}

std::string base64_decode(const std::string& input) {
    std::string out;
    std::uint32_t buffer = 0;
    int bits_collected = 0;

    for (char c : input) {
        if (c == '=') {
            break;
        }
        int val = b64_index(c);
        if (val < 0) {
            // Ignore whitespace / non-base64 chars
            continue;
        }
        buffer = (buffer << 6) | static_cast<std::uint32_t>(val);
        bits_collected += 6;
        if (bits_collected >= 8) {
            bits_collected -= 8;
            unsigned char byte = static_cast<unsigned char>((buffer >> bits_collected) & 0xFFu);
            out.push_back(static_cast<char>(byte));
        }
    }

    return out;
}

} // namespace

// -------------------------- plugin entry --------------------------

extern "C" WOFLANG_PLUGIN_EXPORT void register_plugin(WoflangInterpreter& interp) {
    // ---------------------------------------------------------------------
    // prime_check : n -- 0|1
    //
    // Pops an integer n from the Woflang stack and pushes:
    //   1 if n is prime, 0 otherwise (as integer WofValue)
    // Also logs "<n> is prime" or "<n> is not prime" to stdout.
    // ---------------------------------------------------------------------
    interp.register_op("prime_check", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.empty()) {
            throw std::runtime_error("prime_check requires a number");
        }

        WofValue v = S.back();
        S.pop_back();

        std::int64_t n = to_int64(v, "prime_check");
        bool prime = is_prime_int64(n);

        WofValue result = WofValue::make_int(prime ? 1 : 0);
        S.push_back(result);

        if (prime) {
            std::cout << n << " is prime\n";
        } else {
            std::cout << n << " is not prime\n";
        }
    });

    // ---------------------------------------------------------------------
    // rand_u64 : -- n
    //
    // Pushes a random 64-bit integer (truncated to interpreter's int).
    // ---------------------------------------------------------------------
    interp.register_op("rand_u64", [](WoflangInterpreter& ip) {
        std::uint64_t r = rng_engine()();
        // WofValue uses signed int; keep it non-negative
        std::int64_t v = static_cast<std::int64_t>(r & 0x7FFFFFFFFFFFFFFFll);
        ip.stack.push_back(WofValue::make_int(v));
    });

    // ---------------------------------------------------------------------
    // rand_range : min max -- n
    //
    // Pops max then min, pushes integer uniformly in [min, max].
    // ---------------------------------------------------------------------
    interp.register_op("rand_range", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.size() < 2) {
            throw std::runtime_error("rand_range requires min and max on stack");
        }

        WofValue vmax = S.back(); S.pop_back();
        WofValue vmin = S.back(); S.pop_back();

        std::int64_t max = to_int64(vmax, "rand_range(max)");
        std::int64_t min = to_int64(vmin, "rand_range(min)");

        if (min > max) std::swap(min, max);

        std::uniform_int_distribution<std::int64_t> dist(min, max);
        std::int64_t r = dist(rng_engine());

        S.push_back(WofValue::make_int(r));
    });

    // ---------------------------------------------------------------------
    // hash64 : str -- hash
    //
    // Computes 64-bit FNV-1a hash of the given string, returning it
    // as a numeric (int) on the stack.
    // ---------------------------------------------------------------------
    interp.register_op("hash64", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.empty()) {
            throw std::runtime_error("hash64 requires a string");
        }

        WofValue v = S.back(); S.pop_back();
        std::string s = to_string_value(v, "hash64");

        std::uint64_t h = fnv1a_64(s);
        std::cout << "hash64(\"" << s << "\") = 0x"
                  << std::hex << h << std::dec << "\n";

        // store as signed int but same bit pattern modulo sign
        std::int64_t as_int = static_cast<std::int64_t>(h);
        S.push_back(WofValue::make_int(as_int));
    });

    // ---------------------------------------------------------------------
    // xor_cipher : plaintext key -- ciphertext
    //
    // Simple XOR stream cipher: repeats the key to match plaintext length.
    // Reversible by calling xor_cipher again with same key.
    // ---------------------------------------------------------------------
    interp.register_op("xor_cipher", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.size() < 2) {
            throw std::runtime_error("xor_cipher requires plaintext and key");
        }

        WofValue vkey = S.back(); S.pop_back();
        WofValue vtext = S.back(); S.pop_back();

        std::string key  = to_string_value(vkey, "xor_cipher(key)");
        std::string text = to_string_value(vtext, "xor_cipher(text)");

        if (key.empty()) {
            throw std::runtime_error("xor_cipher key must not be empty");
        }

        std::string out;
        out.resize(text.size());

        for (std::size_t i = 0; i < text.size(); ++i) {
            unsigned char c = static_cast<unsigned char>(text[i]);
            unsigned char k = static_cast<unsigned char>(key[i % key.size()]);
            out[i] = static_cast<char>(c ^ k);
        }

        S.push_back(WofValue::make_string(out));
    });

    // ---------------------------------------------------------------------
    // b64encode : str -- b64_str
    // b64decode : b64_str -- str
    // ---------------------------------------------------------------------
    interp.register_op("b64encode", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.empty()) {
            throw std::runtime_error("b64encode requires a string");
        }
        WofValue v = S.back(); S.pop_back();
        std::string s = to_string_value(v, "b64encode");
        std::string e = base64_encode(s);
        S.push_back(WofValue::make_string(e));
    });

    interp.register_op("b64decode", [](WoflangInterpreter& ip) {
        auto& S = ip.stack;
        if (S.empty()) {
            throw std::runtime_error("b64decode requires a string");
        }
        WofValue v = S.back(); S.pop_back();
        std::string s = to_string_value(v, "b64decode");
        std::string d = base64_decode(s);
        S.push_back(WofValue::make_string(d));
    });

    // ---------------------------------------------------------------------
    // dh_demo : -- secret
    //
    // Small fixed-parameter Diffie-Hellman demo:
    //   p = 23, g = 5
    //   a = 6, b = 15
    // Pushes computed shared secret as integer.
    // ---------------------------------------------------------------------
    interp.register_op("dh_demo", [](WoflangInterpreter& ip) {
        const std::int64_t p = 23;
        const std::int64_t g = 5;
        const std::int64_t a = 6;
        const std::int64_t b = 15;

        auto modexp = [](std::int64_t base, std::int64_t exp, std::int64_t mod) {
            std::int64_t res = 1;
            base %= mod;
            while (exp > 0) {
                if (exp & 1) {
                    res = (res * base) % mod;
                }
                base = (base * base) % mod;
                exp >>= 1;
            }
            return res;
        };

        std::int64_t A = modexp(g, a, p);
        std::int64_t B = modexp(g, b, p);
        std::int64_t s1 = modexp(B, a, p);
        std::int64_t s2 = modexp(A, b, p);

        std::cout << "[dh_demo] p=" << p << " g=" << g
                  << " a=" << a << " b=" << b << "\n";
        std::cout << "[dh_demo] A=g^a mod p = " << A << "\n";
        std::cout << "[dh_demo] B=g^b mod p = " << B << "\n";
        std::cout << "[dh_demo] shared secrets: " << s1 << " and " << s2 << "\n";

        // push shared secret (they should be equal)
        ip.stack.push_back(WofValue::make_int(s1));
    });

    std::cout << "[crypto_ops] extended crypto ops registered.\n";
}
