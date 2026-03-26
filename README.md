# `algorist`

This is a repo with my solutions to the book `The Algorithm Design Manual`.
Right now, it only contains solutions to some of chapter 1's problem set,
because I've been focusing on meeting the author's challenge on implementing a
fairly decent TSP heuristic.

So far, two simple heuristic were implemented, and work is now focused on a
third heuristic based on a triangulation, an MST, a DFS, and finally a k-opt
tour optimization with simulated annealing. The triangulation work is done, and
the MST work is also done. The implementations are actually interesting because
they touch on aspects of computational geometry and (for the MST) implement
themselves heuristic for different input graphs.
