calculus



\# Approx df/dx of sin at x = 1.0 with h = 1e-4:

1.0 0.0001 +

sin            # f(x+h)

1.0 0.0001 -

sin            # f(x-h)

0.0001

derivative\_central



see how close you get to cos(1).







numeric\_diff



\# stack: f(x-h) f(x) h

diff\_backward

Central difference:



wof

Copy code

\# stack: f(x-h) f(x+h) h

diff\_central

Second derivative (Laplacian in 1D):



wof

Copy code

\# stack: f(x-h) f(x) f(x+h) h

diff\_second







catrgory\_theory



Example play:



"ObjA" cat\_obj

"ObjB" cat\_obj

"ObjA" "ObjB" "f" cat\_mor

"ObjA" "ObjB" cat\_hom show

"f" "f" cat\_comp show   # will complain because dom/cod mismatch







moses\_ops/prophecy\_ops



In the REPL, try:



wof> 1 2 3 4 5

wof> moses

wof> moses\_split

wof> prophecy



You should see the fancy stack-sea printouts and prophecies, and prophecy should leave a string on the stack you can inspect with your usual show/debug machinery.







hebrew\_ops



Example session:



wof> "shalom world"

wof> hebrew\_mode\_on

wof> hebrew\_echo        # prints RTL-mirrored version, pushes it back as string



wof> hebrews\_it         # prints the (maybe RTL) tea joke, pushes it



wof> hebrew\_mode\_off

wof> hebrews\_it         # now prints normal English







simple\_chess\_ops



Example REPL session:



wof> chess\_new

wof> chess\_show

wof> "e2e4"

wof> chess\_move      # plays e2e4 if legal, engine replies, pushes engine move

wof> "g1f3"

wof> chess\_move







neural\_chess\_ops



How to run

pip install python-chess torch numpy

python neural\_chess\_ganglion.py --mode self-play --games 10

\# or

python neural\_chess\_ganglion.py --mode human --human-plays-white







duality\_ops



This gives you an actual “duality calculus” you can play with from the REPL:



1 2 dual\_add   # 3

duality\_on

1 2 dual\_add   # -1

1 0 dual\_and   # with duality ON, => 1 (OR)









