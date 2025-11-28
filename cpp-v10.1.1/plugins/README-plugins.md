Plugin System Overview

This folder is a wild playground for modular hacks, extensions, and computational experiments built on top of your woflang interpreter core. Each plugin is a standalone module—usually a .cpp file, grouped by category—baked with its own op set, dirty tricks, and stack conventions. You wanna add math ops, graph theory, music tools, or metaphysics meta-shamanism? Knock yourself out, fren: just drop a new file in the right category and wire it up in CMake.



Folder and File Structure

Main plugin categories and their example ops:



Category	Example File(s)	Sample Ops/Concepts

arts	musicops.cpp	MIDI, bpm, scales, chords

constants	constantsops.cpp	constants, lookup

crypto	cryptoops.cpp, modexpops.cpp	crypto primitives, prime checking

equationsolver	symboliccalcops.cpp	symbolic math and equation solving

games	neuralchessganlgion.py, simplechessops.cpp	chess, AI, game logic

graph	graphcoloringops.cpp	graph algorithms \& searches

language	kanjiops.cpp, cyrillicops.cpp	language/script utilities

logic	logicops.cpp, categorytheoryops.cpp	stacks, logic, category theory

markov	markovsuggestops.cpp	markov and pattern suggestions

math	calculusops.cpp, trigops.cpp	calculus, trig, numerics

metaphysics	dualityops.cpp, entropyops.cpp	duality, entropy, metaphysical ops

quantum	quantumops.cpp	quantum-flavored stunts

science	chemistryops.cpp	chemistry tools

sigil-utils	forbiddenstackslayerops.cpp	arcane stack/voodoo utils

sigils	hebrewops.cpp, glitchmodeops.cpp	sigil/weird occult ops

util	stackops.cpp, assertops.cpp	assert, stack fiddling, I/O debug

See file-structure-plugins.md for a tree dump of all files and categories.



Plugin Coding \& Integration

Pattern: Each plugin is a file, e.g. math/geomtransformops.cpp. Use the provided example archetypal plugin for scaffold/boilerplate.



Exports: Every plugin must provide a single registerplugin function, registering any number of stack ops via interp.registerop("opname", ...);.



Stack API: Interact with the interpreter via the v10 API (WoflangInterpreter, WofValue). Use helper methods for type access and coercion.



CMake: Add your plugin source file as a new target or to an existing category target in CMakeLists.txt for that folder.



No segfaults, no global state leaks—keep things clean, bruv. Leave the stack in a sane state even on error.



Example Usage and Test Cases

Each plugin offers ops callable via your REPL. Here's what you can try for a few highlight plugins:



Calculus (calculusops.cpp)



Central diff:



text

1.0001 sin   0.9999 sin   0.0002 derivativecentral

Approximates 

d

f

d

x

dx

df

&nbsp; for 

sin

⁡

(

x

)

sin(x) at 

x

=

1

x=1 using h=0.0001.



Music Theory (musicops.cpp)



Get MIDI frequency:



text

60 notefreq

Pushes frequency of MIDI note 60 (C4).



Generate polyrhythm:



text

3 2 polyrhythm

ASCII 3:2 polyrhythm pattern.



Logic (logicops.cpp)



Boolean logic ops, e.g.:



text

1 0 and   1 0 or   1 not

Graph Theory (graph.cpp)\*



Build and color a graph:



text

3 'g' graphcolnew

0 1 'g' graphcoladdedge

1 2 'g' graphcoladdedge

'g' graphcolorgreedy

Greedy Welsh–Powell coloring; summary report and number of colors.



Chess (simplechessops.cpp)



Start a new game:



text

chessnew chessshow

Make a move and get engine reply:



text

e2e4 chessmove

Metaphysics and Occult (dualityops.cpp, hebrewops.cpp)



Toggle and use duality modes:



text

dualityon 2 2 dualadd

Hebrew string tricks:



text

hebrewmodeon "shalom" hebrewecho

More usage examples are in testing.md.



Future Additions

See future-adds.md for ops and test/feature roadmap on deck: numeric integration, more physics/chem tools, advanced rhythms, and clever logic/cat theory demos are all fair game.

