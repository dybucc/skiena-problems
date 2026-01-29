//! Problems in _The Algorithm Design Manual_, by S. Skiena, 3rd ed., chapter 1.
//!
//! The use of traits in this crate is not idiomatic; In a real library, the
//! associated functions would've probably been free functions taking in some
//! type implementing a trait providing information on any graph DS.
//!
//! The goal is to simply group under a single umbrella the methods required to
//! implement a certain algorithm for a specific instance of a specific problem.

#![allow(dead_code, reason = "The reportedly unused items are used.")]

use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    iter,
    sync::LazyLock,
};

#[derive(Debug)]
pub struct AdjacencyMatrix {
    inner: Vec<Vec<Edge>>,
}

#[derive(Debug)]
pub struct Pairs<'a> {
    /// Holds the parent of each node (where the node itself is the index).
    pub forest: Vec<usize>,
    /// Holds the node in the lhs of the current Cartesian product.
    pub current_node: Option<usize>,
    /// Holds the nodes in the same tree as [`current_node`].
    ///
    /// [`current_node`]: Pairs::current_node
    pub current_tree: Option<Vec<usize>>,
    /// Holds the Cartesian product of [`current_node`] with all nodes that are
    /// **not** part of [`current_tree`].
    ///
    /// [`current_node`]: Pairs::current_node
    /// [`current_tree`]: Pairs::current_tree
    pub current_product: Vec<(usize, usize)>,
    /// Holds the index of the pair currently being iterated over in the
    /// [`current_product`] field.
    ///
    /// [`current_product`]: Pairs::current_product
    pub current_iter: Option<usize>,
    /// Source graph to refer to when performing graph-level logic on the edges
    /// denoted by [`current_product`].
    ///
    /// [`current_product`]: Pairs::current_product
    pub src: &'a AdjacencyMatrix,
}

#[derive(Debug)]
pub struct AdjacencyList(HashMap<usize, HashSet<usize>>);

impl AdjacencyList {
    pub fn from_pairs(pairs: &Pairs) -> Self {
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
pub struct GeoAdjacencyMatrix(Vec<Vec<GeoEdge>>);

#[derive(Debug, Clone, PartialEq)]
pub enum GeoEdge {
    NonExistent,
    Weighted { weight: usize, coord: Point2d },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Dfs {
    pub graph: AdjacencyList,
    pub stack: Vec<usize>,
    pub discovered: Vec<bool>,
    pub current_iter: Option<usize>,
}

#[derive(Debug)]
pub struct PairsError {
    pub inner: PairsErrorType,
}

#[derive(Debug)]
pub enum PairsErrorType {
    IndexOutOfBounds(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Edge {
    NonExistent,
    Weighted(usize),
}

#[derive(Debug)]
pub struct AdjacencyMatrixError {
    inner: AdjacencyMatrixErrorType,
}

#[derive(Debug)]
pub enum AdjacencyMatrixErrorType {
    NonSquareMatrix(String),
    IncompleteGraph(String),
    DirectedGraph(String),
    SelfLoops(String),
    MultipleEqualPoints(String),
    UnequalSamePoints(String),
}

#[macro_export]
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

#[macro_export]
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
                    "edge weights are forced to be `usize` in the `Ordering::Greater` branch so \
                    this cannot happen",
                )
            }
        }),+]),+])
    }};
}

#[macro_export]
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
    - `MultipleEqualPoints`
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
    (MultipleEqualPoints) => {{
        AdjacencyMatrixErrorType::MultipleEqualPoints(String::from(
            "matrix contains multple vertices with the same coordinates; that's unsupported",
        ))
    }};
    (UnequalSamePoints) => {{
        AdjacencyMatrixErrorType::UnequalSamePoints(String::from(
            "matrix contains points in the same column that are not equal; points in the same \
            column indicate an edge from any row (index) vertex to that point",
        ))
    }};
    (IndexOutOfBounds) => {{ PairsErrorType::IndexOutOfBounds(String::from("ufds doesn't contain such index element")) }};
}

#[macro_export]
macro_rules! ensure_or {
    ($check:expr, $error:tt$(,)?) => {{ $check.then_some(()).ok_or_else(|| build_error!($error)) }};
}

pub fn seglen(Point2d { x: x1, y: y1 }: Point2d, Point2d { x: x2, y: y2 }: Point2d) -> f64 {
    ((x1 - x2).abs().powi(2) + (y1 - y2).abs().powi(2)).sqrt()
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
    pub fn new(input: &[Vec<Edge>]) -> Result<Self, AdjacencyMatrixError> {
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
    pub fn new(src: &'a AdjacencyMatrix) -> Self {
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
    pub fn unite(&mut self, this: usize, other: usize) -> Result<(), PairsError> {
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
    pub fn find(&self, this: usize) -> Result<usize, PairsError> {
        ensure_or!(this < self.forest.len(), IndexOutOfBounds)?;
        match self.forest[this] {
            val if val == this => Ok(this),
            other => self.find(other),
        }
    }

    pub fn same(&self, this: usize, other: usize) -> Result<bool, PairsError> {
        let (this, other) = (self.find(this)?, self.find(other)?);

        Ok(this == other)
    }
}

impl Pairs<'_> {
    pub fn ancestors(&self, this: usize) -> Result<Vec<usize>, PairsError> {
        let (this_root, mut parent, mut ancestors) = (self.find(this)?, this, vec![this]);

        while parent != this_root {
            parent = self.forest[parent];
            ancestors.push(parent);
        }

        ancestors.reverse();

        Ok(ancestors)
    }

    pub fn build_tree_from(&mut self, this: usize) -> Result<(), PairsError> {
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

    pub fn cartesian_product(&mut self) -> Result<(), PairsError> {
        static ERROR_MSG: LazyLock<&str> =
            LazyLock::new(|| "this method should not be called outside iterator chains");

        let current_tree = self.current_tree.as_ref().expect(&ERROR_MSG);
        let others: Vec<_> = (0..self.forest.len())
            .filter(|&node| current_tree.iter().all(|&tree_node| tree_node != node))
            .collect();

        self.current_product = iter::repeat_n(self.current_node.expect(&ERROR_MSG), others.len())
            .zip(others)
            .collect();

        Ok(())
    }

    // NOTE: this exists as a replacement for the `min()` override of
    //       `Iterator`, as that doesn't seem to resolve to the overridden
    //       implementation when used in `tsp()` of `TspClosestPair`.
    pub fn min_fix(&mut self) -> Option<<Self as Iterator>::Item> {
        self.min_by_key(|&(node1, node2)| {
            let Edge::Weighted(weight) = self.src.inner[node1][node2] else {
                unreachable!(
                    "no node considered in the `Pairs` iterator should be `Edge::NonExistent`",
                )
            };

            weight
        })
    }

    pub fn dfs(&self) -> Dfs {
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
    // TODO: ask in Rust community forums about this behavior, and if it is a
    //       bug then report it with the write up you have in the notes under
    //       `~/algorist`.
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
    pub fn new(inner: &[Vec<GeoEdge>]) -> Result<Self, AdjacencyMatrixError> {
        ensure_or!(inner.len() > 1, NonSquareMatrix)?;
        for (vertex, edges) in inner.iter().enumerate() {
            ensure_or!(edges.len() == inner.len(), NonSquareMatrix)?;

            let row_vec: Vec<_> = edges
                .iter()
                .enumerate()
                .filter_map(|(vertex, edge)| {
                    if let GeoEdge::Weighted { weight, coord } = edge {
                        Some((vertex, (weight, coord)))
                    } else {
                        None
                    }
                })
                .collect();

            ensure_or!(
                row_vec.iter().all(|&(inner_idx, _)| inner_idx != vertex),
                SelfLoops,
            )?;
            ensure_or!(row_vec.len() == edges.len() - 1, IncompleteGraph)?;

            ensure_or!(
                row_vec.iter().all(|&(inner_idx, (&weight, _))| {
                    let GeoEdge::Weighted {
                        weight: symmetric_weight,
                        ..
                    } = inner[inner_idx][vertex]
                    else {
                        unimplemented!(
                            "This should be caught when traversing the next row as the symmetric \
                            node is always forward in the input array, but the graph checking \
                            logic relies on traversing each row serially so at this point it is \
                            not yet knonw that the next row would've thrown an \
                            `AdjacencyErrorType::IncompleteGraph`.",
                        );
                    };

                    weight == symmetric_weight
                }),
                DirectedGraph,
            )?;

            ensure_or!(
                row_vec
                    .iter()
                    .fold(Vec::new(), |accum, &(_, (_, point))| {
                        if !accum.contains(&point) {
                            let mut accum = accum;
                            accum.push(point);

                            return accum;
                        }

                        accum
                    })
                    .len()
                    == row_vec.len(),
                MultipleEqualPoints
            )?;

            // Square matrices with dimensionality 2 don't have any other
            // elements in the same column that are not `GeoEdge::NonExistent`.
            if vertex == 0 && inner.len() > 2 {
                ensure_or!(
                    row_vec.iter().all(|&(vertex, (_, point))| {
                        let (mut idx_state, mut checks) = (1, Vec::with_capacity(inner.len()));

                        while let Some(inner) = inner.get(vertex + idx_state) {
                            idx_state += 1;
                            if let GeoEdge::Weighted { coord, .. } = &inner[vertex] {
                                checks.push(coord == point);
                            }
                        }

                        checks.iter().fold(
                            true,
                            |accum, check| {
                                if accum && *check { accum } else { false }
                            },
                        )
                    }),
                    UnequalSamePoints
                )?;
            }
        }

        Ok(Self(inner.to_owned()))
    }

    pub fn from_point_set(points: Vec<Point2d>) -> (Vec<Point2d>, Self) {
        #[derive(Clone, Copy)]
        enum IRGeoEdge {
            NonExistent,
            Weighted(Point2d),
        }

        let mut output = Vec::with_capacity(points.len());
        output.resize_with(points.len(), || {
            let mut output = Vec::with_capacity(points.len());
            output.resize(points.len(), IRGeoEdge::NonExistent);

            output
        });

        output.iter_mut().enumerate().for_each(|(row, row_vec)| {
            row_vec.iter_mut().enumerate().for_each(|(col, edge)| {
                if col == row {
                    *edge = IRGeoEdge::NonExistent;
                } else {
                    *edge = IRGeoEdge::Weighted(points[col]);
                }
            });
        });

        // This finds the largest distance between any one point and any other,
        // different point, in the input set.
        let largest_distance = points
            .iter()
            .enumerate()
            .filter_map(|(idx, point)| {
                let target = points
                    .iter()
                    .skip(idx + 1)
                    .map(|other_point| seglen(*point, *other_point))
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or_default(); // We've either hit the end or not.

                (target != 0.).then_some(target)
            })
            .max_by(|a, b| a.total_cmp(b))
            .expect(
                "The point set should have at least four points to make things interesting, though \
                two is the bare minimum to make for a non-singleton set.",
            );
        let output = Self(
            output
                .iter()
                .enumerate()
                .map(|(row, row_vector)| {
                    row_vector
                        .iter()
                        .enumerate()
                        .map(|(col, vertex)| match vertex {
                            IRGeoEdge::NonExistent => GeoEdge::NonExistent,
                            IRGeoEdge::Weighted(coord) => GeoEdge::Weighted {
                                // The weight is computed as a ratio of the
                                // largest distance found above.
                                weight: ((seglen(points[row], points[col]) * 100.)
                                    / largest_distance)
                                    .floor() as usize,
                                coord: *coord,
                            },
                        })
                        .collect()
                })
                .collect(),
        );

        (points, output)
    }
}

impl PartialEq for GeoAdjacencyMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .enumerate()
            .all(|(idx, row)| row.iter().eq(other.0[idx].iter()))
    }
}

pub trait TspNearestNeighbor {
    fn tsp(&self) -> Vec<usize>;
}

pub trait TspClosestPair {
    fn pairs(&self) -> Pairs<'_>;

    fn tsp(&self) -> Vec<usize>;
}

pub trait TspTriMstDfs {
    fn triangulate(&mut self, points: Vec<Point2d>);
    fn mst(input: &Self) -> Vec<usize>;
    fn dfs(input: &Self) -> Vec<usize>;

    fn tsp(&self) -> Vec<usize>;
}

impl TspNearestNeighbor for AdjacencyMatrix {
    fn tsp(&self) -> Vec<usize> {
        let mut visited = vec![false; self.inner.len()];
        let mut output = Vec::new();
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
                    static ERROR_MSG: LazyLock<&str> =
                        LazyLock::new(|| "matrix elements yielded here should have a weight");

                    let Edge::Weighted(weight1) = elem1 else {
                        unreachable!("{:#?}", &ERROR_MSG)
                    };
                    let Edge::Weighted(weight2) = elem2 else {
                        unreachable!("{:#?}", &ERROR_MSG)
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

impl TspTriMstDfs for GeoAdjacencyMatrix {
    fn triangulate(&mut self, points: Vec<Point2d>) {
        #![expect(unused, reason = "Testing is taking place in separate steps.")]

        let mut points: Vec<_> = points.into_iter().enumerate().collect();
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

        // Follows Andrew's algorithm.
        fn build_hull(
            mut triangulation: Option<&mut [Vec<GeoEdge>]>,
            hull: &mut Vec<(usize, Point2d)>,
            compare: impl Fn(f64, f64) -> bool,
            points: &[(usize, Point2d)],
            edge_src: &[Vec<GeoEdge>],
        ) {
            for (niter, &(vertex, point)) in points.iter().enumerate() {
                while niter > 2
                    && hull.len() > 1
                    && let Some((rm, _)) = {
                        let (_, prev_last) = hull[hull.len() - 2];

                        hull.pop_if(|(_, last)| {
                            compare(
                                if point.y - last.y < *EPS {
                                    prev_last.y
                                } else {
                                    last.y
                                },
                                point.y,
                            )
                        })
                    }
                {
                    if let Some(ref mut triangulation) = triangulation {
                        let (&(prev, _), post) = (
                            hull.last()
                                .expect("The hull should have at least two points here."),
                            vertex,
                        );

                        (
                            triangulation[prev][rm],
                            triangulation[post][rm],
                            triangulation[rm][prev],
                            triangulation[rm][post],
                        ) = (
                            edge_src[prev][rm].clone(),
                            edge_src[post][rm].clone(),
                            edge_src[rm][prev].clone(),
                            edge_src[rm][post].clone(),
                        );
                    }
                }

                hull.push((vertex, point));
            }
        }

        points.sort_unstable_by(
            |(_, Point2d { x: x1, y: y1 }), (_, Point2d { x: x2, y: y2 })| match x1.total_cmp(x2) {
                Ordering::Equal => y1.total_cmp(y2),
                other => other,
            },
        );
        build_hull(
            Some(&mut triangulation),
            &mut upper_hull,
            |last, point| last <= point,
            &points,
            &self.0,
        );

        points.sort_unstable_by(
            |(_, Point2d { x: x1, y: y1 }), (_, Point2d { x: x2, y: y2 })| match x1.total_cmp(x2) {
                Ordering::Equal => match y1.total_cmp(y2) {
                    Ordering::Less => Ordering::Greater,
                    Ordering::Greater => Ordering::Less,
                    equal => equal,
                },
                other => other,
            },
        );

        let first_same_x_n = points
            .iter()
            .filter(|&(_, Point2d { x, .. })| *x == points[0].1.x)
            .count();
        let first_same_x = &mut points[0..first_same_x_n];
        first_same_x.reverse();

        build_hull(
            None,
            &mut lower_hull,
            |last, point| last >= point,
            &points,
            &self.0,
        );

        panic!(
            "Reached end of `upper_hull` construction:\
            \n{upper_hull:#?}\
            \nReached end of `lower_hull` construction:\
            \n{lower_hull:#?}\
            \nFinal state of the point set:\
            \n{points:#?}",
        );

        {
            points.sort_unstable_by(
                |(_, Point2d { x: x1, y: y1 }), (_, Point2d { x: x2, y: y2 })| match x1
                    .total_cmp(x2)
                {
                    Ordering::Equal => y1.total_cmp(y2),
                    other => other,
                },
            );

            let first_samex_num = points
                .iter()
                .take_while(|&&(_, Point2d { x, .. })| x == points[0].1.x)
                .count();
            let first_samex = &mut points[0..first_samex_num];
            first_samex.reverse();

            let mut dummy_lower_hull = Vec::with_capacity(points.len().div_ceil(2));
            build_hull(
                Some(&mut triangulation),
                &mut dummy_lower_hull,
                |last, point| last >= point,
                &points,
                &self.0,
            );
        }

        {
            let mut consume_and_triangulate = |collection: Vec<(usize, Point2d)>| {
                collection
                    .windows(2)
                    .map(|inner| (inner[0].0, inner[1].0))
                    .for_each(|(src, dst)| {
                        triangulation[src][dst] = self.0[src][dst].clone();
                        triangulation[dst][src] = self.0[dst][src].clone();
                    });
            };

            consume_and_triangulate(upper_hull);
            consume_and_triangulate(lower_hull);
        }

        static EPS: LazyLock<f64> = LazyLock::new(|| 1e-9);

        // See Lemma 1.3.1 in O'Rourke, 2001.
        const fn compute_triangle_area((a, b, c): (Point2d, Point2d, Point2d)) -> f64 {
            ((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)).abs() / 2.
        }

        fn check_point_ownership(
            (a, b, c): (Point2d, Point2d, Point2d),
            p_to_check: Point2d,
        ) -> bool {
            let container_area = compute_triangle_area((a, b, c));
            let (area_0, area_1, area_2) = {
                let (t_0, t_1, t_2) = ((a, b, p_to_check), (a, c, p_to_check), (c, b, p_to_check));

                (
                    compute_triangle_area(t_0),
                    compute_triangle_area(t_1),
                    compute_triangle_area(t_2),
                )
            };

            container_area - (area_0 + area_1 + area_2) <= *EPS
        }

        // This follows from the fact that for some three points to lie in the
        // same ring, the same segment (radius of the ring) must join those
        // points to the unknown. Thus, this becomes a problem of finding the
        // endpoint of the segment making up the edge incident to the two
        // triangles formed from joining the known points as the bases and
        // having the unknown as the remaining vertex in each triangle. Such
        // edge is a segment whose length will be the same for all three
        // segments going from each of the known points to the ring's center
        // point (the unknown.)
        fn find_ring((a, b, c): (Point2d, Point2d, Point2d)) -> Option<Point2d> {
            // This should be good enough for the purposes of robot tour
            // optimization.
            static EPS: LazyLock<f64> = LazyLock::new(|| 1e-3);

            ((-b.x + a.x).abs() > *EPS
                && (-b.y + c.y).abs() > *EPS
                && ((b.y - a.y) / (-b.x + a.x)) * ((b.x - c.x) / (-b.y + c.y)) != 1. + *EPS)
                .then(|| {
                    let (c0, c1, c2, c3) = (
                        (a.x.powi(2) + a.y.powi(2) - b.x.powi(2) - b.y.powi(2))
                            / (2. * (-b.x + a.x)),
                        (b.x - c.x) / (-b.y + c.y),
                        (c.x.powi(2) + c.y.powi(2) - b.x.powi(2) - b.y.powi(2))
                            / (2. * (-b.y + c.y)),
                        (b.y - a.y) / (-b.x + a.x),
                    );
                    let y = (c0 * c1 + c2) / (1. - c3 * c1);
                    let x = y * c3 + c0;

                    Point2d { x, y }
                })
        }

        let mut tracking_list = Vec::with_capacity(triangulation.len().pow(2));
        for (src, edges) in triangulation.iter().enumerate().map(|(src, edges)| {
            (
                src,
                edges
                    .iter()
                    .enumerate()
                    .filter_map(|(dst, edge)| {
                        if let GeoEdge::Weighted { coord, .. } = edge {
                            (!tracking_list.contains(&(dst, src))).then(|| {
                                tracking_list.push((src, dst));

                                (dst, coord)
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        }) {
            'edge_loop: for &(dst, &dst_point) in &edges {
                let (p1, p2) = {
                    let mut d1_adjacent_points = triangulation[src]
                        .iter()
                        .enumerate()
                        .filter_map(|(src_adjacent, src_adjacent_edge)| {
                            if let GeoEdge::Weighted { coord, .. } = src_adjacent_edge
                                && src_adjacent != dst
                            {
                                Some((src_adjacent, coord))
                            } else {
                                None
                            }
                        })
                        .filter(|&(src_adjacent, &src_adjacent_point)| {
                            triangulation[src_adjacent].iter().enumerate().any(
                                |(potential_dst, potential_dst_edge)| {
                                    potential_dst == dst
                                        && matches!(potential_dst_edge, GeoEdge::Weighted { .. })
                                },
                            ) && {
                                let (a, &GeoEdge::Weighted { coord: b, .. }, c) =
                                    (dst_point, &triangulation[dst][src], src_adjacent_point)
                                else {
                                    unreachable!(
                                        "This destructuring pattern should not be refutable, \
                                        because edge `src`->`dst` exists, and thus edge \
                                        `dst`->`src` must exist; The triangulation is processed in \
                                        terms of a directed graph."
                                    );
                                };

                                !triangulation[src]
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(other_src_adjacent, other_src_adjacent_edge)| {
                                        if let GeoEdge::Weighted { coord, .. } =
                                            other_src_adjacent_edge
                                            && other_src_adjacent != dst
                                            && other_src_adjacent != src_adjacent
                                        {
                                            Some(coord)
                                        } else {
                                            None
                                        }
                                    })
                                    .any(|&potential_miss| {
                                        check_point_ownership((a, b, c), potential_miss)
                                    })
                            }
                        });

                    assert_eq!(d1_adjacent_points.clone().count(), 2);

                    (d1_adjacent_points.next(), d1_adjacent_points.next())
                };

                if let Some((ex1, &p1)) = p1
                    && let Some((ex2, &p2)) = p2
                    && let p_dst = dst_point
                    && let GeoEdge::Weighted { coord: p_src, .. } = triangulation[dst][src]
                {
                    // Some vertex in the quadrilateral proved to be a reflex
                    // vertex (i.e. angle > PI, see Sec. 1.1.2, Subsec.
                    // Empirical Exploration in O'Rourke, 2001.)
                    if check_point_ownership((p1, p2, p_src), p_dst)
                        || check_point_ownership((p1, p2, p_dst), p_src)
                    {
                        continue 'edge_loop;
                    }

                    // Checking for the local edge yielding an optimal
                    // triangulation is done by means of computing for a ring
                    // that crosses three of the quadrilateral's vertices, and
                    // evaluating whether the remaining vertex lies within the
                    // inner area of that ring. The correctness of this argument
                    // follows from Thales' theorem. See Section 9.1.2 in de
                    // Berg et. al., 2008.
                    if let Some(ring_center) = find_ring((p_src, p1, p_dst)) {
                        let (center_to_p2, ring_radius) =
                            (seglen(ring_center, p2), seglen(ring_center, p1));

                        if ring_radius - center_to_p2 < (ring_radius - *EPS) {
                            (
                                *RefCell::new(&triangulation[src][dst]).borrow_mut(),
                                *RefCell::new(&triangulation[dst][src]).borrow_mut(),
                                *RefCell::new(&triangulation[ex1][ex2]).borrow_mut(),
                                *RefCell::new(&triangulation[ex2][ex1]).borrow_mut(),
                            ) = (
                                &GeoEdge::NonExistent,
                                &GeoEdge::NonExistent,
                                &self.0[ex1][ex2],
                                &self.0[ex2][ex1],
                            );
                        }
                    }
                }
            }
        }
    }
    fn mst(input: &Self) -> Vec<usize> {
        todo!();
    }
    fn dfs(input: &Self) -> Vec<usize> {
        todo!();
    }

    fn tsp(&self) -> Vec<usize> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use macros::points;

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
                (0., 0., 2), (0., 0., 0);
            }
            .is_ok(),
            "should've been an ok graph with 2 nodes layed out like the defining vertices of a \
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
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::DirectedGraph(_)
            )),
            "should've thrown an error about the graph not being undirected",
        );
    }

    #[test]
    fn basic_geometric_directed_graph() {
        assert!(
            geomatrix! {
                (0., 0., 0), (1., 1., 2);
                (0., 0., 3), (0., 0., 0);
            }
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::DirectedGraph(_)
            )),
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
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::NonSquareMatrix(_)
            )),
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
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::NonSquareMatrix(_)
            )),
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
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::SelfLoops(_)
            )),
            "should've thrown an error about the graph having self-loops (i.e. the main diagonal \
            is not made out of zeroes)",
        );
    }

    #[test]
    fn basic_geometric_nonsimple_graph() {
        assert!(
            geomatrix! {
                (0., 0., 1), (1., 0., 2);
                (0., 0., 2), (1., 0., 1);
            }
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::SelfLoops(_)
            )),
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
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::IncompleteGraph(_)
            )),
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
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::IncompleteGraph(_)
            )),
            "should've thrown an error about the graph not having as many edges as a complete, \
            simple graph is expected to have (i.e. the matrix has zeroes outside the main \
            diagonal)",
        );
    }

    #[test]
    fn basic_geometric_same_row_points_graph() {
        assert!(
            geomatrix! {
                (0., 0., 0), (1., 0., 1), (1., 0., 1);
                (0., 1., 1), (0., 0., 0), (2., 0., 1);
                (0., 1., 1), (1., 0., 1), (0., 0., 0);
            }
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::MultipleEqualPoints(_)
            )),
            "should've thrown an error about the graph having multiple vertices in the same row \
            with the same coordinates in euclidean space",
        )
    }

    #[test]
    fn basic_geometric_same_col_points_graph() {
        assert!(
            geomatrix! {
                (0., 0., 0), (1., 0., 1), (0., 1., 1);
                (0., 0., 1), (0., 0., 0), (0., 2., 1);
                (1., 0., 1), (0., 1., 1), (0., 0., 0);
            }
            .is_err_and(|AdjacencyMatrixError { inner: err }| matches!(
                err,
                AdjacencyMatrixErrorType::UnequalSamePoints(_)
            )),
            "should've thrown an error about the graph having multiple vertices in the same column \
            that are not equal",
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
    fn points_macro1() -> Result<(), AdjacencyMatrixError> {
        assert_eq!(
            points! {
                (x: 1.25, y: 2),
                (x: 1.3, y: 5),
                (x: 1.5, y: 3.5),
            }
            .1,
            geomatrix! {
                (0.,   0., 0),   (1.3, 5., 100), (1.5, 3.5, 50);
                (1.25, 2., 100), (0.,  0., 0),   (1.5, 3.5, 50);
                (1.25, 2., 50),  (1.3, 5., 50),  (0.,  0., 0);
            }?
        );

        Ok(())
    }

    #[test]
    fn points_macro2() -> Result<(), AdjacencyMatrixError> {
        assert_eq!(
            points! {
                (x: 0, y: 0),
                (x: 1.3, y: 5),
                (x: 1.5, y: 3.5),
            }
            .1,
            geomatrix! {
                (0., 0., 0),   (1.3, 5., 100), (1.5, 3.5, 73);
                (0., 0., 100), (0.,  0., 0),   (1.5, 3.5, 29);
                (0., 0., 73),  (1.3, 5., 29),  (0.,  0., 0);
            }?
        );

        Ok(())
    }

    #[test]
    fn points_macro3() -> Result<(), AdjacencyMatrixError> {
        assert_eq!(
            points! {
                (x: 1.25, y: 2),
                (x: 1.3, y: 5),
                (x: 1.5, y: 3.5),
                (x: 2, y: 3.6),
                (x: 3, y: 0.75),
                (x: 3.75, y: 3.7),
            }
            .1,
            geomatrix! {
                (0.,   0., 0),  (1.3, 5., 65),  (1.5, 3.5, 33), (2., 3.6, 38), (3., 0.75, 46),  (3.75, 3.7, 66);
                (1.25, 2., 65), (0.,  0., 0),   (1.5, 3.5, 33), (2., 3.6, 34), (3., 0.75, 100), (3.75, 3.7, 60);
                (1.25, 2., 33), (1.3, 5., 33),  (0.,  0.,  0),  (2., 3.6, 11), (3., 0.75, 68),  (3.75, 3.7, 49);
                (1.25, 2., 38), (1.3, 5., 34),  (1.5, 3.5, 11), (0., 0.,  0),  (3., 0.75, 65),  (3.75, 3.7, 38);
                (1.25, 2., 46), (1.3, 5., 100), (1.5, 3.5, 68), (2., 3.6, 65), (0., 0.,   0),   (3.75, 3.7, 66);
                (1.25, 2., 66), (1.3, 5., 60),  (1.5, 3.5, 49), (2., 3.6, 38), (3., 0.75, 66),  (0.,   0.,  0);
            }?
        );

        Ok(())
    }

    #[test]
    #[should_panic = "Reached end"]
    fn triangulation1() {
        let (points, mut matrix) = points! {
            (x: 1.25, y: 2),
            (x: 1.3, y: 5),
            (x: 1.5, y: 3.5),
            (x: 2, y: 3.6),
            (x: 3, y: 0.75),
            (x: 3.75, y: 3.7),
        };

        matrix.triangulate(points);
    }

    #[test]
    #[should_panic = "Reached end"]
    fn triangulation2() {
        let (points, mut matrix) = points! {
            (x: 0, y: 1),
            (x: 0, y: 2.5),
            (x: 1, y: 2),
            (x: 2, y: 2.5),
            (x: 2, y: 5),
            (x: 3, y: 2.5),
            (x: 4, y: 0),
            (x: 4, y: 1),
            (x: 4, y: 3.25),
            (x: 5, y: 2.5),
            (x: 6, y: 2),
            (x: 6, y: 3.25),
            (x: 7, y: 2),
        };

        matrix.triangulate(points);
    }

    #[test]
    #[ignore = "I need to sample the workings of the `triangulate()` method with a smaller input \
               set."]
    fn triangulation3() {
        let (points, mut matrix) = points! {
            (x: 1.25, y: 2),
            (x: 1.3, y: 5),
            (x: 1.5, y: 3.5),
            (x: 2, y: 3.6),
            (x: 3, y: 0.75),
            (x: 3.75, y: 3.7),
            (x: 4.25, y: 3),
            (x: 4.3, y: 1.7),
            (x: 4.5, y: 5),
            (x: 5.8, y: 3.45),
            (x: 6, y: 1),
            (x: 6.2, y: 4.7),
            (x: 7, y: 3.45),
        };

        matrix.triangulate(points);
    }

    #[test]
    #[ignore = "The algorithm is a WIP, and the test sample case is not ready yet."]
    #[allow(unreachable_code, reason = "Ibid.")]
    fn tsp_mst_dfs1() -> Result<(), AdjacencyMatrixError> {
        todo!();

        Ok(())
    }

    #[test]
    #[ignore = "The algorithm is a WIP, and the test sample case is not ready yet."]
    #[allow(unreachable_code, reason = "Ibid.")]
    fn tsp_mst_dfs2() -> Result<(), AdjacencyMatrixError> {
        todo!();

        Ok(())
    }
}
