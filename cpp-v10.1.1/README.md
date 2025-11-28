[README.md] 20251128205559

# README.md for C++ v10.1.1

hopefully the final c++ woflang…


----------------------------------------------------------------

╦ ╦┌─┐┌─┐┬  ┌─┐┌┐┌┌─┐  
║║║│ │├┤ │  ├─┤││││ ┬  
╚╩╝└─┘└  ┴─┘┴ ┴┘└┘└─┘ v10.1.1

# WofLang - A Unicode-native Stack Language  
**Creative, Symbolic, Secure Computation for the Modern Era**

----------------------------------------------------------------


## Quickstart Guide

### Requirements

- C++23 Compiler (MSVC recommended)
- CMake (latest)
- Windows-first, Linux port-friendly

### Building (Windows Example)

```sh
git clone https://github/whisprer/wooflang-alpha-r/
cd woflang-alpha-r/cpp-v10.1.1/
cmake -B build
cmake --build build --config Release
Running

cd build
./bin/woflang.exe
```
The REPL launches. You can immediately begin experimenting with Unicode-powered stack operations.

or:

```sh
git clone https://github/whisprer/wooflang-alpha-r/
cd woflang-alpha-r/cpp-v10.1.1/
cmake -G Ninja ..
ninja -v
Running

cd build
./bim/woflang.exe
```

or [advised]

```sh
git clone https://github/whisprer/wooflang-alpha-r/
cd woflang-alpha-r/cpp-v10.1.1/
./clean-n-build.sh

cd build
./bin/wolfing.exe
```

also [advised] first run:
./bin/woflang.exe --test
./bin/woflang.exe --benchmark

_if_ achieves 58/58 and 17/17 then proceed to the wonders of
**Unicode-powered stack operations**!

-----------------------------------------------------------------


✅ Modernized plugins (new-style register\_plugin API)

All of these:
Include woflang.hpp directly (so they work with your current include paths).
Use WofValue::make\_\* helpers and WofType correctly.
Avoid the old WoflangPlugin base class.
Have basic error safety / input checks and no cursed ternaries.

----------------------------------------------------------------


Some Notes Pertaining To Included files:

ops\_symbolic\_simplify\_rules.cpp

Implements:
simplify\_sum : rewrites X X + → 2 X \* when X is a Symbol and "+" is a String token.
simplify\_mul\_one : rewrites X 1 \* or 1 X \* → X.
Uses helpers: is\_symbol, is\_string, is\_integer, as\_text, as\_int.


ops\_symbolic\_solve\_patterns.cpp
Simple demo stub:
pattern\_solve : prints a message; you can extend it with real pattern-matching later.

quantum\_ops.cpp
Uses a proper std::mt19937 RNG.

Ops:
|ψ⟩ : pushes a random qubit {0,1} and logs.
H : discards top, pushes new random qubit.
X : flips 0 ↔ 1 for integer qubits, leaves non-integers alone with a warning.
measure : prints measurement and pushes the classical result back as an integer.

repl\_history\_commands.cpp
Keeps a global g\_repl\_history vector of strings.

Ops:
add\_history : pops a String/Symbol from stack and appends.
show\_history : prints indexed history.
clear\_history : clears the history.

repl\_suggest\_command.cpp
Uses std::mt19937 + uniform\_int\_distribution.
Op suggest prints a random suggestion like Try: 2 pi \* r \*.

wof\_markov\_math\_suggestions.cpp
Similar RNG setup.
Op markov\_suggest prints a random math-flavoured suggestion like Try: pi \* radius radius \*.

-----------------------------------------------------------------


