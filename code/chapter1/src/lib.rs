//! Problems in _The Algorithm Design Manual_, by S. Skiena, 3rd ed., chapter 1.
//!
//! The use of traits in this crate is not idiomatic; In a real library, the
//! associated functions would've probably been free functions taking in some
//! type implementing a trait that provided information on any graph DS.
//!
//! The goal is to simply group under a single umbrella the methods required to
//! implement a certain algorithm for a specific instance of a problem.

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    iter,
    sync::LazyLock,
};

#[derive(Debug)]
struct AdjacencyMatrix {
    inner: Vec<Vec<Edge>>,
}

#[derive(Debug)]
struct Pairs<'a> {
    /// Holds the parent of each node (where the node itself is the index).
    forest: Vec<usize>,
    /// Holds the node in the lhs of the current Cartesian product.
    current_node: Option<usize>,
    /// Holds the nodes in the same tree as [`current_node`].
    ///
    /// [`current_node`]: Pairs::current_node
    current_tree: Option<Vec<usize>>,
    /// Holds the Cartesian product of [`current_node`] with all nodes that are
    /// **not** part of [`current_tree`].
    ///
    /// [`current_node`]: Pairs::current_node
    /// [`current_tree`]: Pairs::current_tree
    current_product: Vec<(usize, usize)>,
    /// Holds the index of the pair currently being iterated over in the
    /// [`current_product`] field.
    ///
    /// [`current_product`]: Pairs::current_product
    current_iter: Option<usize>,
    /// Source graph to refer to when performing graph-level logic on the edges
    /// denoted by [`current_product`].
    ///
    /// [`current_product`]: Pairs::current_product
    src: &'a AdjacencyMatrix,
}

#[derive(Debug)]
struct AdjacencyList(HashMap<usize, HashSet<usize>>);

impl AdjacencyList {
    fn from_pairs(pairs: &Pairs) -> Self {
        let mut output = Self(HashMap::with_capacity(pairs.forest.len()));
        for ancestors in (0..pairs.forest.len()).filter_map(|node| {
            let ancestors = pairs
                .ancestors(node)
                .expect("`node` is sourced directly from the `Pairs` tree");

            (ancestors.len() > 1).then_some(ancestors)
        }) {
            for (src, dst) in ancestors.windows(2).map(|inner| (inner[0], inner[1])) {
                output
                    .0
                    .entry(src)
                    .and_modify(|adjacent_nodes| {
                        adjacent_nodes.insert(dst);
                    })
                    .or_insert_with(|| {
                        let mut adjacent_nodes = HashSet::with_capacity(pairs.forest.len());
                        adjacent_nodes.insert(dst);

                        adjacent_nodes
                    });
                output
                    .0
                    .entry(dst)
                    .or_insert_with(|| HashSet::with_capacity(pairs.forest.len()));
            }
        }

        output
    }
}

#[derive(Debug)]
struct GeoAdjacencyMatrix(Vec<Vec<GeoEdge>>);

#[derive(Debug, Clone)]
enum GeoEdge {
    NonExistent,
    Weighted { weight: usize, coord: Point2d },
}

#[derive(Debug, Clone, PartialEq)]
struct Point2d {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Dfs {
    graph: AdjacencyList,
    stack: Vec<usize>,
    discovered: Vec<bool>,
    current_iter: Option<usize>,
}

#[derive(Debug)]
struct PairsError {
    inner: PairsErrorType,
}

#[derive(Debug)]
enum PairsErrorType {
    IndexOutOfBounds(String),
}

#[derive(Clone, PartialEq, Debug)]
enum Edge {
    NonExistent,
    Weighted(usize),
}

#[derive(Debug)]
struct AdjacencyMatrixError {
    inner: AdjacencyMatrixErrorType,
}

#[derive(Debug)]
enum AdjacencyMatrixErrorType {
    NonSquareMatrix(String),
    IncompleteGraph(String),
    DirectedGraph(String),
    SelfLoops(String),
    NonMatchingCoordinates(String),
}

macro_rules! matrix {
    ($($($weight:literal),+);+ $(;)?) => {
        AdjacencyMatrix::new(&[$(vec![$({
            match $weight.cmp(&0) {
                Ordering::Equal => Edge::NonExistent,
                Ordering::Greater => Edge::Weighted($weight),
                _ => unimplemented!(
                    "edges are forced to be `usize` in the `Ordering::Greater` branch so this \
                    cannot happen",
                ),
            }
        }),+]),+])
    };
}

macro_rules! geomatrix {
    ($($(($x:literal, $y:literal, $weight:literal)),+);+ $(;)?) => {{
        GeoAdjacencyMatrix::new(&[$(vec![$({
            match $weight.cmp(&0) {
                Ordering::Equal => GeoEdge::NonExistent,
                Ordering::Greater => GeoEdge::Weighted {
                    weight: $weight,
                    coord: Point2d { x: $x, y: $y },
                },
                _ => unimplemented!(
                    "edges are forced to be `usize` in the `Ordering::Greater` branch so this \
                    cannot happen",
                )
            }
        }),+]),+])
    }};
}

macro_rules! build_error {
    () => {{
        compile_error!(
            r#"The macro supports one of the following enum error variants, each a sublist item of
its overarching enum type:
- `AdjacencyMatrixErrorType`:
    - `NonSquareMatrix`
    - `IncompleteGraph`
    - `DirectedGraph`
    - `SelfLoops`
    - `NonMatchingCoordinates`
- `PairsErrorType`:
    - `IndexOutOfBounds`"#
        )
    }};
    (NonSquareMatrix) => {{
        AdjacencyMatrixErrorType::NonSquareMatrix(String::from(
            "matrix is not square; adjacency matrices must be square",
        ))
    }};
    (IncompleteGraph) => {{
        AdjacencyMatrixErrorType::IncompleteGraph(String::from(
            "matrix contains more nonexistent edges than it should; this is not a complete graph",
        ))
    }};
    (DirectedGraph) => {{
        AdjacencyMatrixErrorType::DirectedGraph(String::from(
            "matrix contains different values above and below the main diagonal; this is not an \
            undirected graph",
        ))
    }};
    (SelfLoops) => {{
        AdjacencyMatrixErrorType::SelfLoops(String::from(
            "matrix contains self-loops; this is not a simple graph",
        ))
    }};
    (NonMatchingCoordinates) => {{
        AdjacencyMatrixErrorType::NonMatchingCoordinates(String::from(
            "matrix contains non-matching coordinates on symmetric nodes; check the `geomatrix!` \
            macro call for inconsistencies between values above and below the main diagonal",
        ))
    }};
    (IndexOutOfBounds) => {{ PairsErrorType::IndexOutOfBounds(String::from("ufds doesn't contain such index element")) }};
}

macro_rules! ensure_or {
    ($check:expr, $error:tt$(,)?) => {{ $check.then_some(()).ok_or_else(|| build_error!($error)) }};
}

impl From<AdjacencyMatrixErrorType> for AdjacencyMatrixError {
    fn from(value: AdjacencyMatrixErrorType) -> Self {
        Self { inner: value }
    }
}

impl From<PairsErrorType> for PairsError {
    fn from(value: PairsErrorType) -> Self {
        Self { inner: value }
    }
}

impl AdjacencyMatrix {
    fn new(input: &[Vec<Edge>]) -> Result<Self, AdjacencyMatrixError> {
        ensure_or!(input.len() > 1, NonSquareMatrix)?;
        for (idx, vertex) in input.iter().enumerate() {
            ensure_or!(vertex.len() == input.len(), NonSquareMatrix)?;

            let row_vec: Vec<_> = vertex
                .iter()
                .enumerate()
                .filter(|(_, edge)| matches!(edge, Edge::Weighted(_)))
                .collect();

            ensure_or!(
                row_vec.iter().all(|&(inner_idx, _)| inner_idx != idx),
                SelfLoops,
            )?;
            ensure_or!(row_vec.len() == vertex.len() - 1, IncompleteGraph)?;

            ensure_or!(
                row_vec.iter().all(|&(inner_idx, edge)| {
                    let Edge::Weighted(weight) = *edge else {
                        unreachable!("there should at least be one weighted edge")
                    };
                    let Edge::Weighted(symmetric_weight) = input[inner_idx][idx] else {
                        unreachable!("there should at least be one symmetric weighted edge")
                    };

                    weight == symmetric_weight
                }),
                DirectedGraph,
            )?;
        }

        Ok(Self {
            inner: input.into(),
        })
    }
}

impl<'a> Pairs<'a> {
    fn new(src: &'a AdjacencyMatrix) -> Self {
        Self {
            forest: (0..src.inner.len()).collect(),
            current_node: None,
            current_tree: None,
            current_product: vec![],
            current_iter: None,
            src,
        }
    }
}

impl Pairs<'_> {
    fn unite(&mut self, this: usize, other: usize) -> Result<(), PairsError> {
        if !self.same(this, other)? {
            self.forest[other] = this;
        }

        Ok(())
    }

    /// Finds the root of the given `this` node in `self.forest`.
    ///
    /// Returns the same node if the node makes up a single-vertex tree.
    /// Otherwise, returns the root node by following the parent relationship in
    /// the same tree.
    fn find(&self, this: usize) -> Result<usize, PairsError> {
        ensure_or!(this < self.forest.len(), IndexOutOfBounds)?;
        match self.forest[this] {
            val if val == this => Ok(this),
            other => self.find(other),
        }
    }

    fn same(&self, this: usize, other: usize) -> Result<bool, PairsError> {
        let (this, other) = (self.find(this)?, self.find(other)?);

        Ok(this == other)
    }

    fn ancestors(&self, this: usize) -> Result<Vec<usize>, PairsError> {
        let (this_root, mut parent, mut ancestors) = (self.find(this)?, this, vec![this]);

        while parent != this_root {
            parent = self.forest[parent];
            ancestors.push(parent);
        }

        ancestors.reverse();

        Ok(ancestors)
    }

    fn build_tree_from(&mut self, this: usize) -> Result<(), PairsError> {
        ensure_or!(this < self.forest.len(), IndexOutOfBounds)?;
        self.current_tree = Some(
            iter::repeat_n(this, self.forest.len())
                .zip(0..self.forest.len())
                .filter_map(|(this, other)| {
                    self.same(this, other)
                        .expect(
                            "both `this` and `other` should be within-bounds, as `this` was \
                            checked at the start of the function and `other` is sourced from a \
                            range over `self.chains`",
                        )
                        .then_some(other)
                })
                .collect(),
        );

        Ok(())
    }

    fn cartesian_product(&mut self) -> Result<(), PairsError> {
        static ERROR_MSG: LazyLock<&str> =
            LazyLock::new(|| "this method should not be called outside iterator chains");

        let current_tree = self.current_tree.as_ref().expect(&ERROR_MSG);
        let others: Vec<_> = self
            .forest
            .iter()
            .enumerate()
            .filter_map(|(node, _)| {
                current_tree
                    .iter()
                    .all(|&tree_node| tree_node != node)
                    .then_some(node)
            })
            .collect();

        self.current_product = iter::repeat_n(self.current_node.expect(&ERROR_MSG), others.len())
            .zip(others)
            .collect();

        Ok(())
    }

    // NOTE: this exists as a replacement for the `min()` override of
    //       `Iterator`, as that doesn't seem to resolve to the overridden
    //       implementation when used in `tsp()` of `TspClosestPair`.
    fn min_fix(&mut self) -> Option<<Self as Iterator>::Item> {
        self.min_by_key(|&(node1, node2)| {
            let Edge::Weighted(weight) = self.src.inner[node1][node2] else {
                unreachable!(
                    "no node considered in the `Pairs` iterator should be `Edge::NonExistent`",
                )
            };

            weight
        })
    }

    fn dfs(&self) -> Dfs {
        let mut root = self
            .forest
            .iter()
            .enumerate()
            .filter_map(|(node, &parent)| (node == parent).then_some(node));
        assert_eq!(
            root.clone().count(),
            1,
            "this method should be called only once there's a single tree left in the forest"
        );

        Dfs {
            graph: AdjacencyList::from_pairs(self),
            stack: {
                let mut output = Vec::with_capacity(self.forest.len());
                output.push(
                    self.forest[root
                        .next()
                        .expect("this should yield the tree root sourced directly from `self`")],
                );

                output
            },
            discovered: {
                let mut output = Vec::with_capacity(self.forest.len());
                output.resize(self.forest.len(), false);

                output
            },
            current_iter: None,
        }
    }
}

impl Iterator for Pairs<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_node {
            None => {
                self.current_node = Some(0);
                self.build_tree_from(0).expect(
                    "the operation should be infallible because the index is a constant \
                    that's always within bounds",
                );
                self.cartesian_product().expect(
                    "the product should be an infallible operation if the source graph is \
                    valid",
                );
                self.current_iter = Some(0);
            }
            Some(mut current_node) => {
                if let Some(ref mut val) = self.current_iter {
                    match val.cmp(&&mut (self.current_product.len() - 1)) {
                        Ordering::Less => *val += 1,
                        Ordering::Equal => {
                            static ERROR_MSG: LazyLock<&str> = LazyLock::new(|| {
                                "this operation should be infallible if the iteration indices \
                                (`self.current_node` and `self.current_iter`) have been correctly \
                                handled"
                            });

                            current_node =
                                (current_node < self.forest.len() - 1).then_some(current_node)? + 1;

                            self.current_node = Some(current_node);
                            self.build_tree_from(current_node).expect(&ERROR_MSG);
                            self.cartesian_product().expect(&ERROR_MSG);
                            self.current_iter = Some(0);
                        }
                        _ => unreachable!(
                            "the collection index should have been reset in the `Ordering::Equal` \
                            branch"
                        ),
                    }
                } else {
                    unreachable!(
                        "iteration should have already started if `self.current_node` != `None`"
                    )
                }
            }
        }

        Some(
            self.current_product[self
                .current_iter
                .expect("`self.current_iter` should be infallible at this point")],
        )
    }

    // NOTE: this doesn't seem to resolve to the overridden implementation when
    //       used in `tsp()` from `TSPClosestPair`, so the `min_fix()` method on
    //       `Pairs` is being used instead as a drop-in replacement.
    // TODO: ask in Rust community forums about this behavior, and if it a bug
    //       then report it with the write up you have in the notes under
    //       ~/algorist.
    fn min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let matrix = &self.src.inner;
        self.min_by_key(|&(node1, node2)| {
            let Edge::Weighted(weight) = matrix[node1][node2] else {
                unreachable!(
                    "no node considered in the `Pairs` iterator should be `Edge::NonExistent`",
                )
            };

            weight
        })
    }
}

impl Iterator for Dfs {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_iter {
            None => {
                self.current_iter = Some(self.stack[0]);
                self.discovered[self.stack[0]] = true;
            }
            Some(ref mut current_vertex) => {
                for adjacent_vertex in &self.graph.0[current_vertex] {
                    if !self.discovered[*adjacent_vertex] {
                        self.stack.push(*adjacent_vertex);
                    }
                }

                *current_vertex = self.stack.pop()?;
                self.discovered[*current_vertex] = true;
            }
        }

        self.current_iter
    }
}

impl GeoAdjacencyMatrix {
    fn new(inner: &[Vec<GeoEdge>]) -> Result<Self, AdjacencyMatrixError> {
        ensure_or!(inner.len() > 1, NonSquareMatrix)?;
        for (idx, vertex) in inner.iter().enumerate() {
            ensure_or!(vertex.len() == inner.len(), NonSquareMatrix)?;

            let row_vec: Vec<_> = vertex
                .iter()
                .enumerate()
                .filter(|(_, edge)| matches!(edge, GeoEdge::Weighted { .. }))
                .collect();

            ensure_or!(
                row_vec.iter().all(|&(inner_idx, _)| inner_idx != idx),
                SelfLoops,
            )?;
            ensure_or!(row_vec.len() == vertex.len() - 1, IncompleteGraph)?;

            ensure_or!(
                row_vec.iter().all(|&(inner_idx, edge)| {
                    let GeoEdge::Weighted { weight, .. } = *edge else {
                        unreachable!(
                            "any `NonExistent` node under consideration has been filtered out so \
                            this execution branch should not be possible",
                        );
                    };
                    let GeoEdge::Weighted {
                        weight: symmetric_weight,
                        ..
                    } = inner[inner_idx][idx]
                    else {
                        unimplemented!(
                            "this should be caught when traversing the next row as the symmetric \
                            node is always forward in the input array, but the graph checking \
                            logic relies on traversing each row serially so at this point it is \
                            not yet knonw that the next row would've thrown an \
                            `AdjacencyErrorType::IncompleteGraph`",
                        );
                    };

                    weight == symmetric_weight
                }),
                DirectedGraph,
            )?;

            // Even though the running conditions for the following iterator
            // chain are the same as those of the above sequence, this is
            // required to be separate because the type of error thrown is
            // different (and this is not meant to be used as a real library, so
            // no greater effort is put into designing efficient routines for
            // failure propagation.)
            ensure_or!(
                row_vec.iter().all(|&(inner_idx, edge)| {
                    let GeoEdge::Weighted { coord, .. } = edge else {
                        unreachable!(
                            "any `NonExistent` node under consideration has been filtered out so \
                            this should not be possible",
                        );
                    };
                    let GeoEdge::Weighted {
                        coord: ref symmetric_coord,
                        ..
                    } = inner[inner_idx][idx]
                    else {
                        unimplemented!(
                            "this should be caught when traversing the next row as the symmetric \
                            node is always forward in the input array, but the graph checking \
                            logic relies on traversing each row serially so at this point it is \
                            not yet knonw that the next row would've thrown an \
                            `AdjacencyErrorType::IncompleteGraph`",
                        );
                    };

                    coord == symmetric_coord
                }),
                NonMatchingCoordinates,
            )?;
        }

        Ok(Self(inner.to_owned()))
    }
}

trait TspNearestNeighbor {
    fn tsp(&self) -> Vec<usize>;
}

trait TspClosestPair {
    fn pairs(&self) -> Pairs<'_>;

    fn tsp(&self) -> Vec<usize>;
}

trait TspMstDfs {
    fn triangulate(&mut self) -> Result<(), AdjacencyMatrixError>;
    fn mst(input: &Self) -> Vec<usize>;
    fn dfs(input: &Self) -> Vec<usize>;

    fn tsp(&self) -> Vec<usize>;
}

impl TspNearestNeighbor for AdjacencyMatrix {
    fn tsp(&self) -> Vec<usize> {
        let mut visited = vec![false; self.inner.len()];
        let mut output = vec![];
        let mut current_idx = 0;

        while visited.iter().any(|visited| !visited) {
            let current = &self.inner[current_idx];

            output.push(current_idx);
            visited[current_idx] = true;

            (current_idx, _) = current
                .iter()
                .enumerate()
                .filter_map(|(idx, edge)| {
                    (!visited[idx] && matches!(edge, Edge::Weighted(_))).then_some((idx, edge))
                })
                .min_by(|(_, elem1), (_, elem2)| {
                    let Edge::Weighted(weight1) = elem1 else {
                        unreachable!("matrix elements yielded here should have a weight")
                    };
                    let Edge::Weighted(weight2) = elem2 else {
                        unreachable!("matrix elements yielded here should have a weight")
                    };

                    weight1.cmp(weight2)
                })
                .unwrap_or((0, &Edge::NonExistent));
        }

        output.push(current_idx);

        output
    }
}

impl TspClosestPair for AdjacencyMatrix {
    fn pairs(&self) -> Pairs<'_> {
        Pairs::new(self)
    }

    fn tsp(&self) -> Vec<usize> {
        let mut pairs_iter = self.pairs();
        for _ in 1..self.inner.len() {
            let (node1, node2) = pairs_iter.min_fix().expect(
                "there should always be a minimum value given the loop runs for n - 1 \
                iterations, where n is the number of nodes in the graph, and the underlying ufds \
                decreases its number of disjoint trees by a factor of 1 on each iteration (i.e. on \
                each call to `unite()` with the nodes yielded by `min_fix()`)",
            );

            static ERROR_MSG: LazyLock<&str> = LazyLock::new(|| {
                "`node2` was just sourced through `min_fix()` so the operation should be \
                infallible"
            });

            // If the node to be `unite()`d is not a root node, then make it a
            // root node by reversing the parent node of its ancestors.
            if pairs_iter.find(node2).expect(&ERROR_MSG) != node2 {
                let ancestors = pairs_iter.ancestors(node2).expect(&ERROR_MSG);
                (0..ancestors.len() - 1).for_each(|current| {
                    pairs_iter.forest[ancestors[current]] = ancestors[current + 1];
                });

                pairs_iter.forest[node2] = node2;
            }
            pairs_iter.unite(node1, node2).expect(
                "the node indices are sourced directly from the iterator itself so the \
                operation should be infallible",
            );

            // Resets the state of the iterator to force cycling with updated
            // state on the next iteration of the overarching loop.
            pairs_iter.current_node = None;
        }

        pairs_iter.dfs().collect()
    }
}

impl TspMstDfs for GeoAdjacencyMatrix {
    fn triangulate(&mut self) -> Result<(), AdjacencyMatrixError> {
        let mut points: Vec<_> = self
            .0
            .iter()
            .take(2)
            .enumerate()
            .flat_map(|(vertex, edges)| {
                edges
                    .iter()
                    .take(if vertex == 0 { edges.len() } else { 1 })
                    .enumerate()
                    .filter_map(|(vertex, edge)| {
                        if let GeoEdge::Weighted { coord, .. } = edge {
                            Some((vertex, coord))
                        } else {
                            None
                        }
                    })
            })
            .collect();

        let (mut upper_hull, mut lower_hull, mut triangulation) = (
            Vec::with_capacity(points.len().div_ceil(2)),
            Vec::with_capacity(points.len().div_ceil(2)),
            Vec::with_capacity(points.len()),
        );

        triangulation.resize_with(points.len(), || {
            let mut output = Vec::with_capacity(points.len());
            output.resize(points.len(), GeoEdge::NonExistent);

            output
        });

        fn build_hull<'a>(
            triangulation: &mut [Vec<GeoEdge>],
            hull: &mut Vec<(usize, &'a Point2d)>,
            compare: impl Fn(f64, f64) -> bool,
            points: &[(usize, &'a Point2d)],
            edge_src: &[Vec<GeoEdge>],
        ) {
            for &(vertex, point) in points {
                while hull.len() > 2
                    && let Some((rm, _)) = hull.pop_if(|(_, last)| compare(last.y, point.y))
                {
                    let &(prev, _) = hull
                        .last()
                        .expect("The hull should have at least two points here.");
                    let post = vertex;

                    (triangulation[prev][rm], triangulation[post][rm]) =
                        (edge_src[prev][rm].clone(), edge_src[post][rm].clone());
                }

                hull.push((vertex, point));
            }
        }

        // Sort increasingly by both x- and y-components and build the upper
        // hull.
        points.sort_unstable_by(
            |&(_, coord1), &(_, coord2)| match coord1.x.total_cmp(&coord2.x) {
                order @ (Ordering::Less | Ordering::Greater) => order,
                Ordering::Equal => coord1.y.total_cmp(&coord2.y),
            },
        );
        build_hull(
            &mut triangulation,
            &mut upper_hull,
            |last, point| last <= point,
            &points,
            &self.0,
        );

        // Sort increasingly by x-component and decreasingly by y-component, and
        // build the lower hull.
        points.sort_unstable_by(
            |&(_, coord1), &(_, coord2)| match coord1.x.total_cmp(&coord2.x) {
                order @ (Ordering::Less | Ordering::Greater) => order,
                Ordering::Equal => match coord1.y.total_cmp(&coord2.y) {
                    Ordering::Less => Ordering::Greater,
                    Ordering::Greater => Ordering::Less,
                    equal => equal,
                },
            },
        );
        build_hull(
            &mut triangulation,
            &mut lower_hull,
            |last, point| last >= point,
            &points,
            &self.0,
        );

        todo!("Add perimeter edges of both `upper_hull` and `lower_hull` to `triangulation`");

        todo!(
            "Improve the triangulation with the local maxima algorithm for building Delauney \
            triangulations from other triangulations in Skiena, page 631.",
        );
    }
    fn mst(_input: &Self) -> Vec<usize> {
        todo!();
    }
    fn dfs(_input: &Self) -> Vec<usize> {
        todo!();
    }

    fn tsp(&self) -> Vec<usize> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_graph() {
        assert!(
            matrix! {
                0, 2;
                2, 0;
            }
            .is_ok(),
            "should've been an ok graph with two vertices and one weight 2 edge between them",
        );
    }

    #[test]
    fn basic_geometric_graph() {
        assert!(
            geomatrix! {
                (0., 0., 0), (1., 1., 2);
                (1., 1., 2), (0., 0., 0);
            }
            .is_ok(),
            "should've been an ok graph with 2 nodes layed out like the corners defining a \
            rectangle",
        )
    }

    #[test]
    fn basic_directed_graph() {
        assert!(
            matrix! {
                0, 2;
                3, 0;
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::DirectedGraph(_))),
            "should've thrown an error about the graph not being undirected",
        );
    }

    #[test]
    fn basic_geometric_directed_graph() {
        assert!(
            geomatrix! {
                (0., 0., 0), (1., 1., 2);
                (1., 1., 3), (0., 0., 0);
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::DirectedGraph(_))),
            "should've thrown an error about the graph not being undirected",
        );
    }

    #[test]
    fn malformed_matrix_graph() {
        assert!(
            matrix! {
                0, 2, 3;
                0, 2;
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::NonSquareMatrix(_))),
            "should've thrown an error about the matrix not being square, or a matrix for that \
            matter",
        );
    }

    #[test]
    fn malformed_geometric_matrix_graph() {
        assert!(
            geomatrix! {
                (0., 0., 0), (0., 0., 2), (0., 0., 3);
                (0., 0., 0), (0., 0., 2);
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::NonSquareMatrix(_))),
            "should've thrown an error about the matrix not being square, or a matrix for that \
            matter",
        );
    }

    #[test]
    fn basic_nonsimple_graph() {
        assert!(
            matrix! {
                1, 2;
                2, 1;
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::SelfLoops(_))),
            "should've thrown an error about the graph having self-loops (i.e. the main diagonal \
            is not made out of zeroes)",
        );
    }

    #[test]
    fn basic_geometric_nonsimple_graph() {
        assert!(
            geomatrix! {
                (0., 0., 1), (0., 0., 2);
                (0., 0., 2), (0., 0., 1);
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::SelfLoops(_))),
            "should've thrown an error about the graph having self-loops (i.e. the main diagonal \
            is not made out of zeroes)",
        );
    }

    #[test]
    fn basic_incomplete_graph() {
        assert!(
            matrix! {
                0, 0;
                2, 0;
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::IncompleteGraph(_))),
            "should've thrown an error about the graph not having as many edges as a complete, \
            simple graph is expected to have (i.e. the matrix has zeroes outside the main \
            diagonal)",
        );
    }

    #[test]
    fn basic_geometric_incomplete_graph() {
        assert!(
            geomatrix! {
                (0., 0., 0), (0., 0., 0);
                (0., 0., 2), (0., 0., 0);
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::IncompleteGraph(_))),
            "should've thrown an error about the graph not having as many edges as a complete, \
            simple graph is expected to have (i.e. the matrix has zeroes outside the main \
            diagonal)",
        );
    }

    #[test]
    fn basic_geometric_nonmatching_coordinates_graph() {
        assert!(
            geomatrix! {
                (0., 0., 0), (0., 1., 2);
                (1., 0., 2), (0., 0., 0);
            }
            .is_err_and(|err| matches!(
                err.inner,
                AdjacencyMatrixErrorType::NonMatchingCoordinates(_)
            )),
            "should've thrown an error about the graph not having matching coordinates on \
            symmetric vertices (this is not required to check for an undirected graph because the \
            coordinates only serve as satellite data during construction of the adjacency matrix)",
        )
    }

    #[test]
    fn tsp_nearest_neighbor1() -> Result<(), AdjacencyMatrixError> {
        assert_eq!(
            TspNearestNeighbor::tsp(&matrix! {
                0, 1, 3;
                1, 0, 4;
                3, 4, 0;
            }?),
            vec![0, 1, 2, 0],
        );

        Ok(())
    }

    #[test]
    fn tsp_nearest_neighbor2() -> Result<(), AdjacencyMatrixError> {
        assert_eq!(
            TspNearestNeighbor::tsp(&matrix! {
                0, 3, 4, 4, 2;
                3, 0, 4, 2, 2;
                4, 4, 0, 3, 2;
                4, 2, 3, 0, 2;
                2, 2, 2, 2, 0;
            }?),
            vec![0, 4, 1, 3, 2, 0],
        );

        Ok(())
    }

    #[test]
    fn tsp_closest_pair1() -> Result<(), AdjacencyMatrixError> {
        let input = TspClosestPair::tsp(&matrix! {
            0, 1, 3;
            1, 0, 4;
            3, 4, 0;
        }?);
        assert!(input == vec![0, 1, 2, 0] || input == vec![0, 2, 1, 0]);

        Ok(())
    }

    #[test]
    fn tsp_closest_pair2() -> Result<(), AdjacencyMatrixError> {
        let input = TspClosestPair::tsp(&matrix! {
            0, 3, 4, 4, 2;
            3, 0, 4, 2, 2;
            4, 4, 0, 3, 2;
            4, 2, 3, 0, 2;
            2, 2, 2, 2, 0;
        }?);
        assert!(input == vec![2, 4, 1, 3, 0, 2] || input == vec![2, 4, 0, 1, 3, 2]);

        Ok(())
    }

    #[test]
    #[ignore = "the algorithm is a wip"]
    fn tsp_mst_dfs1() -> Result<(), AdjacencyMatrixError> {
        let _input = TspMstDfs::tsp(&geomatrix! {
            (0., 0., 0), (0., 1., 1), (1., 1., 3);
            (0., 1., 1), (0., 0., 0), (0., 2., 4);
            (1., 1., 3), (0., 2., 4), (0., 0., 0);
        }?);
        todo!();

        Ok(())
    }

    #[test]
    #[ignore = "the algorithm is a wip, and the test sample case is not ready yet"]
    fn tsp_mst_dfs2() -> Result<(), AdjacencyMatrixError> {
        todo!();

        Ok(())
    }
}
