# `algorist`

This is a repo with my solutions to the book `The Algorithm Design Manual`.
Right now, it only contains solutions to some of chapter 1's problem set,
because I've been focusing on meeting the author's challenge on implementing a
fairly decent TSP heuristic (this is one of the problems in the first chapter.)

So far, two simple heuristics were implemented, and work is now focused on a
third heuristic based on a triangulation, an MST, a DFS, and finally, a k-opt
optimization tour with simulated annealing. The triangulation work is done, and
the MST work is also done. The implementations are actually interesting because
they touch on aspects of computational geometry and (in the case of the MST)
implement themselves heuristics for different input graphs.

There's also other work in this repo concerning my short stint with the book
_Program Proofs_, though that's been put on halt for the time being.

## Current work

Right now, work is focused on implementing a better `find_ring()` primitive, as
the prior work was based on a handwritten proof that didn't provide much
efficency nor numerical stability. For that, I'm reading the book
_Matrix Computations_, which should hopefully provide me with the required
knowledge to implement an efficient algorithm for Gaussian elimination on the
input system of equations for that routine.
