logic\_ops
Real logical ops (and, or, xor, not, implies, eq, neq, gt, lt, gte, lte) instead of the “not yet implemented” stubs.



Keeps all your original ops and semantics:

and, or, xor, not, implies

eq, neq

gt, lt, gte, lte

Uses the current WoflangInterpreter + stack API.

Implements the same truthiness and string-aware equality you had.





expo-log
Proper exp / ln / log / log10 / log2 wired into the stack, using the same numeric semantics as the rest of v10.



fractal\_ops
mandelbrot — escape-time iteration count for a point in the complex plane
sierpinski — ASCII Sierpinski triangle (same classic bitwise pattern)
hausdorff — generic self-similar fractal dimension: D = log(N) / log(scale)



calculus\_ops
New numeric ops:
derivative\_central
Usage: f(x+h) f(x-h) h derivative\_central
Stack before: \[..., fp, fm, h]
Pops h, fm, fp
Pushes central difference:
(𝑓(𝑥+ℎ)−𝑓(𝑥−ℎ))/(2ℎ)

derivative\_forward
Usage: f(x+h) f(x) h derivative\_forward
Stack before: \[..., fp, f0, h]
Pops h, f0, fp
Pushes forward diff:
(𝑓(𝑥+ℎ)−𝑓(𝑥))/ℎ

derivative\_backward
Usage: f(x) f(x-h) h derivative\_backward
Stack before: \[..., f0, fm, h]
Pops h, fm, f0
Pushes backward diff:
(𝑓(𝑥)−𝑓(𝑥−ℎ))/ℎ

slope (simple secant slope)
Usage: y2 y1 x2 x1 slope
Stack before: \[..., y2, y1, x2, x1]
Pops x1, x2, y1, y2



Pushes(𝑦2−𝑦1)/(𝑥2−𝑥1)

For backwards compatibility with the v9 stub names, also kept:
derivative → prints a hint and does not modify the stack
integral → prints a hint and does not modify the stack





greek\_math\_ops

Provides ops:



greek\_info

Stack: \[name-or-letter] (string)

Pops a string like "alpha", "Α", "β", "omega", etc.

Pushes a JSON-ish info string about that letter.

greek\_random

Stack: \[] -> \[info-string]

Picks a random Greek letter and pushes an info string.

greek\_groups

Prints vowels vs consonants (no stack use).

greek\_quiz

Zero-arg “mental quiz”: prints a random letter \& its position, but hides the name/pron until “answer” line. No stack effects.





discrete\_math\_ops

Ops:



factorial

Stack: \[n] -> \[n!] (as double, n clamped to \[0, 170])

nCr

Stack: \[n k] -> \[C(n, k)] (double)

Uses stable multiplicative formula, reduces k when k > n/2.

nPr

Stack: \[n k] -> \[P(n, k)] (double)

gcd

Stack: \[a b] -> \[gcd(a, b)] (double; a,b rounded to int64)

lcm

Stack: \[a b] -> \[lcm(a, b)] (double; uses gcd)

fib

Stack: \[n] -> \[F\_n] (double; iterative, clamps n into sane range)





numeric\_diff\_ops



diff\_forward

diff\_backward

diff\_central

diff\_second





gradient\_hessian\_ops



This gives you:

grad2\_central



hess2\_central

All in 2D, using central differences.



Stack conventions

grad2\_central



Stack before (bottom → top):

f(x−h, y)

f(x+h, y)

f(x, y−h)

f(x, y+h)

h



Stack after:

∂f/∂x

∂f/∂y



hess2\_central



Stack before (bottom → top):

f(x−h, y−h)

f(x−h, y)

f(x−h, y+h)

f(x, y−h)

f(x, y)

f(x, y+h)

f(x+h, y−h)

f(x+h, y)

f(x+h, y+h)

h



Stack after (bottom → top, relative to previous contents):

f\_xx

f\_yy

f\_xy



with:

𝑓𝑥𝑥≈(𝑓(𝑥+ℎ,𝑦)−2𝑓(𝑥,𝑦)+𝑓(𝑥−ℎ,𝑦))/ℎ2fxx

𝑓𝑦𝑦≈(𝑓(𝑥,𝑦+ℎ)−2𝑓(𝑥,𝑦)+𝑓(𝑥,𝑦−ℎ))/ℎ2fyy



≈(f(x,y+h)−2f(x,y)+f(x,y−h))/h2

𝑓𝑥𝑦≈(𝑓(𝑥+ℎ,𝑦+ℎ)−𝑓(𝑥+ℎ,𝑦−ℎ)−𝑓(𝑥−ℎ,𝑦+ℎ)+𝑓(𝑥−ℎ,𝑦−ℎ))/(4ℎ2)fxy	​





geom\_tranform\_ops



2\. plugins/math/geom\_transform\_ops.cpp



This gives you:

translate2d

scale2d

rotate2d\_rad

rotate2d\_deg

cart\_to\_polar

polar\_to\_cart



All strictly numeric, stack-based.

Stack conventions

translate2d

Before: x y dx dy

After: x' y' where:

x' = x + dx

y' = y + dy



scale2d

Before: x y sx sy

After: x' y' where:

x' = x \* sx

y' = y \* sy



rotate2d\_rad

Before: x y θ\_rad

After: x' y' where:

x' = x cos θ − y sin θ

y' = x sin θ + y cos θ



rotate2d\_deg

Before: x y θ\_deg

After: x' y' (same formulas, but θ is degrees; we convert internally).

cart\_to\_polar

Before: x y



After: r θ\_rad with:

r = hypot(x, y)

θ = atan2(y, x)



polar\_to\_cart

Before: r θ\_rad

After: x y with:

x = r cos θ

y = r sin θ



Preserves:

translate2d (x,y,dx,dy → x',y')

scale2d

rotate2d\_rad

rotate2d\_deg

cart\_to\_polar

polar\_to\_cart





graph\_theory\_ops



Ops provided (stack order is left→right, op consumes from the right):



graph\_new

Stack: num\_nodes:int name:string graph\_new

Creates an undirected graph name with num\_nodes nodes, labeled 0..num\_nodes-1.

Overwrites any existing graph with the same name.



graph\_add\_edge

Stack: u:int v:int name:string graph\_add\_edge

Adds an undirected edge between u and v in graph name.



graph\_degree

Stack: node:int name:string graph\_degree

Pushes the degree of node in graph name (as integer).



graph\_clear

Stack: name:string graph\_clear

Deletes the named graph (no-op if it doesn’t exist).



Here’s a drop-in replacement that:

Keeps all existing ops: graph\_new, graph\_add\_edge, graph\_degree, graph\_clear

Removes the dead WoflangPlugin inheritance

Uses the current v10 API (woflang::WoflangInterpreter, WofValue, WofType)





graph\_search\_ops



Ops provided:



graph\_bfs\_reach

Stack: start:int name:string graph\_bfs\_reach

Performs BFS from start in graph name, pushes count of reachable nodes (int).



graph\_path\_exists

Stack: dst:int start:int name:string graph\_path\_exists

Pushes 1 if there is a path, 0 otherwise.



graph\_shortest\_path\_len

Stack: dst:int start:int name:string graph\_shortest\_path\_len

Pushes the shortest path length in edges, or -1 if unreachable.





graph\_coloring\_ops



Ops



All ops use a named undirected graph stored inside this plugin.



Stack order is: left → right, op consumes from the right.



graph\_col\_new

(num\_nodes:int name:string graph\_col\_new) → creates / resets graph name with nodes 0..n-1.



graph\_col\_add\_edge

(u:int v:int name:string graph\_col\_add\_edge) → add undirected edge (u, v).



graph\_color\_greedy

(name:string graph\_color\_greedy) → runs greedy Welsh–Powell-style colouring.

Pushes:



summary:string (multi-line report of node → colour)



num\_colors:int (number of distinct colours used)



graph\_col\_clear

(name:string graph\_col\_clear) → delete graph name from this plugin’s registry.



graph\_col\_new

graph\_col\_add\_edge

graph\_color\_greedy

graph\_col\_clear





graph\_shortest\_path\_ops



Ops



graph\_w\_new

(num\_nodes:int name:string graph\_w\_new)

New undirected weighted graph name.



graph\_w\_add\_edge

(weight:double v:int u:int name:string graph\_w\_add\_edge)

Add undirected edge (u, v) with weight weight.



graph\_w\_shortest

(dst:int start:int name:string graph\_w\_shortest)

Runs Dijkstra from start to dst.

Pushes:



path\_string:string like "0 -> 3 -> 5 (dist=7.5)" or "unreachable"



distance:double (or -1.0 if no path)





stack\_ops



Ops provided



stack\_dup – duplicate the top of the stack

stack\_swap – swap the top two elements

stack\_drop – pop and discard the top

stack\_clear – clear the whole stack

stack\_depth – push the current stack depth as a number





assert\_ops



Ops provided



assert\_true – pop 1 value, treats it as numeric; fails if value == 0

assert\_eq – pop 2 values, compare numerically; fails if they differ beyond epsilon

assert\_near – pop 3 values: value, expected, epsilon; fails if |value - expected| > epsilon





modexp\_ops



Ops provided:



modexp – compute modular exponentiation

Stack: a b m -- (a^b mod m)



modinv – modular inverse via extended GCD

Stack: a m -- inv where a \* inv ≡ 1 (mod m)





prime\_ops



Ops provided:



is\_prime – primality test (deterministic trial division for 64-bit range)

Stack: n -- 0|1



next\_prime – next prime ≥ n

Stack: n -- p



prime\_factors – trial division factorisation, pushes factors (ascending)

Stack: n -- f1 f2 ... fk (nothing else; factors only)





entropy\_ops



entropy

Computes Shannon entropy (base-2, in bits) over the entire stack by treating each value as a symbol.



chaos

Randomly shuffles the stack in place.



order

Stably orders the stack: numeric values first (ascending), then non-numeric in original order.





chemistry\_ops



Implement:

element\_info

atomic\_weight

molecular\_weight

temp\_convert

avogadro



Use the current core API (WofValue::make\_double, make\_string, as\_numeric, WofType).

Behave safely on bad input (no throws, no crashes, just friendly error messages + reasonable stack behaviour).





music\_ops



gives you note ↔ MIDI ↔ frequency tools (12-TET + generic EDO),

builds scales \& chords by name,

describes intervals,

computes BPM → ms timing and swing offsets,

generates a simple polyrhythm pattern,

and has a little call/response helper for melodic ideas.



You can either:

replace the contents of your existing music.cpp with this, or

save it as plugins/arts/music\_ops.cpp and wire it up in CMake as a separate plugin target.



Quick usage cheatsheet (from the REPL)



Assuming this plugin is built as music or music\_ops and auto-loaded:

Timing / rhythm

120 "1/8" bpm\_ms → ms duration of an 8th note at 120 BPM

120 0.66 swing\_ms → how many ms you delay the off-beat for swung 8ths



Pitch / intervals

60 note\_freq → frequency of MIDI 60 (C4) in Hz

69 432 note\_freq → A4 at 432 Hz reference

60 midi\_name → "C4"

64 60 interval\_info → "4 semitones (major third)"



Scales / chords

"C" "major" build\_scale → Major (Ionian) on C: C D E F G A B

"A" "harmonic\_minor" build\_scale → notes of A harmonic minor

"C" "maj7" chord\_tones → Major 7th on C: C E G B

"E" "m7b5" chord\_tones → E half-dim 7th tones



Polyrhythms

3 2 polyrhythm → multi-line string showing 3:2 pattern as ASCII



Microtonal (EDO)

3 19 440 edo\_freq → step 3 of 19-TET above 440 Hz

7 24 440 edo\_freq → 24-TET microstep freq



Phrase writing

"C E G" "E G C" call\_response\_hint → small call-and-response suggestion string



It exposes these ops:



scale\_info

interval\_semitones

euclid\_pattern

music\_help

bpm\_ms

note\_freq

midi\_name

interval\_info

build\_scale

chord\_tones

polyrhythm

edo\_freq

swing\_ms

call\_response\_hint



All the theory helpers (scale/chord tables, interval names, polyrhythm generator, MIDI→name/freq, etc.) are preserved.





moses\_ops



moses

Just analyzes the current stack and prints a “parting” visualisation, without mutating.

moses\_split

Actually partitions the stack into left/right halves with a clear separator marker (a string value) in the middle so you can see/use the two sides.





prophecy\_ops



Uses the new woflang.hpp include path.

Uses WofType/WofValue directly instead of any obsolete helpers.

Adds a tiny bit of stack integration: it pushes the prophecy string as a String value on the stack as well as printing it.





hebrew\_ops



Features:



hebrew\_mode\_on / hebrew\_mode\_off

Toggles a static bool inside the plugin (simple, local “mode”).



hebrew\_echo

Pops a value, turns it into a string, emits a Right-to-Left mirrored version when mode is on, and pushes the transformed string back.



hebrews\_it

Emits the tea joke, conditioned by the mode (normal vs RTL-mirrored), and pushes the string on the stack.



Uses your standard types: WoflangInterpreter, WofValue, WofType, interp.stack, interp.register\_op(...).





simple\_chess\_ops



Here’s a self-contained simple chess plugin that:

Keeps a single global game state (start position, side to move).

Supports basic legal move generation for all pieces (no castling, no en passant; promotion = queen only).

Rejects illegal moves (wrong shape, off board, blocked, capture own piece).

Has a tiny 3-ply alpha-beta search for the engine reply.

Integrates cleanly with your v10 plugin API (woflang.hpp, WoflangPlugin, WoflangInterpreter, WofValue, WofType, interp.stack, interp.register\_op).



Ops provided



chess\_new

Reset to the standard start position (white to move).

chess\_show

Print an ASCII board and side to move.

chess\_move

Expects a string on the top of the stack like "e2e4" or "g1f3".

If the move is legal for the current side, it is played.

Then the engine computes a reply (3-ply) for the new side to move and plays it (if any legal move exists).

The engine’s move string (like "e7e5") is pushed as a string on the stack.

If there is no reply (checkmate / stalemate), a message is printed and an empty string is pushed.





neural\_chess\_ops



a self-contained Python implementation that:

Uses python-chess for full rules:

castling, en passant, promotions, legal move gen, game termination.

Implements a “Ganglion Brain” composed of:

A CNN over 12×8×8 board planes.

A GRU (RNN) over move history.

An LSTM over a cellular automaton grid (CA) that evolves each step.

Wraps those three in a synchronized “Ganglion” module:

CA is updated with board input each turn.

CNN, GRU history, and LSTM( CA ) all produce embeddings that are fused.

Fused embedding feeds:

a policy head → logits over 4096 possible from→to squares.

a value head → scalar evaluation.

Adds a small discriminator network to form a GAN-like pair:

Generator = GanglionBrain (policy/value).

Discriminator = judges (board, move) pairs as “plausible vs implausible”.



Provides:

Human vs AI play (--mode human).

Self-play training over N games (--mode self-play --games N).

All with no placeholders, fully wired and runnable.



Deps: python-chess, torch, numpy

Install:

pip install python-chess torch numpy





crypto\_ops



This keeps your current prime\_check behavior, and adds:

rand\_u64 – 64-bit random integer

rand\_range – random integer in \[min, max]

hash64 – FNV-1a 64-bit hash of a string

xor\_cipher – toy XOR stream cipher (string ⊕ key)

b64encode / b64decode – Base64 string encode/decode

dh\_demo – small, deterministic Diffie–Hellman demo with stack output





pattern\_solver\_ops



pattern\_solve expects three values on the stack:

expr pattern replacement pattern\_solve

(i.e. expr at the bottom, replacement on top – normal postfix order.)

Treats all three as strings.

Uses std::regex to find pattern in expr and replace with replacement.

Pushes the transformed expression back as a string.

Logs how many replacements were made.





duality\_ops



Here I wire duality into actual behavior, within the constraints of “plugin only”:

Maintains a global g\_duality\_enabled flag.



Exposes:

duality\_on / duality\_off / duality\_toggle – control the mode.

dual\_add – if duality is off: a b dual\_add => a + b; if on: a b dual\_add => a - b.

dual\_and – if duality is off: boolean AND; if on: boolean OR.

dual\_or – if duality is off: boolean OR; if on: boolean AND.

dual\_not – always logical NOT, but logs that it’s reflecting duality mode.

dual\_logic – given a formula string, swaps textual and/or, true/false (a syntactic dual).



This gives you real, inspectable dual behavior without needing to modify the core dispatch machinery.





kanji\_ops



kanji\_info: pop a kanji char; push "kanji|onyomi|romaji|meaning|example|level".

kanji\_search\_meaning: pop English keyword; push array of packed matches.

kanji\_random: pop optional level filter; push one packed kanji string.

kanji\_levels: push metadata summary string.

