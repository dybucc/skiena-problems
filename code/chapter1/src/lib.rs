//! Problems in _The Algorithm Design Manual_, by S. Skiena, 3rd ed., chapter 1.
//!
//! The use of traits in this crate is not idiomatic; In a real library, the
//! associated functions would've probably been free functions taking in some
//! type implementing a trait providing information on any graph DS.
//!
//! The goal is to simply group under a single umbrella the methods required to
//! implement a certain algorithm for a specific instance of a specific problem.
//!
//! The use of errors is also very much unidiomatic and overall not what you
//! would go for in a real library.

#![feature(stmt_expr_attributes, float_algebraic)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    iter,
    ops::ControlFlow,
    sync::LazyLock,
};

#[derive(Debug)]
pub struct AdjacencyMatrix(Vec<Vec<Edge>>);

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
    /// # Panics
    ///
    /// Can't really panic because all elements of the forest are guaranteed to
    /// at least have themselves as their ancestors.
    #[must_use]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeoEdge {
    NonExistent,
    Weighted { weight: usize, coord: Point2d },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Hash for Point2d {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Eq for Point2d {}

#[derive(Debug)]
pub struct Dfs {
    pub graph: AdjacencyList,
    pub stack: Vec<usize>,
    pub discovered: Vec<bool>,
    pub current_iter: Option<usize>,
}

#[derive(Debug)]
pub struct PairsError(pub PairsErrorType);

#[derive(Debug)]
pub enum PairsErrorType {
    IndexOutOfBounds(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Edge {
    NonExistent,
    Weighted(usize),
}

#[expect(
    dead_code,
    reason = "This works alongside the `build_error!` macro, so it's actually used."
)]
#[derive(Debug)]
pub struct AdjacencyMatrixError(AdjacencyMatrixErrorType);

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
            "Matrix is not square; Adjacency matrices must be square.",
        ))
    }};
    (IncompleteGraph) => {{
        AdjacencyMatrixErrorType::IncompleteGraph(String::from(
            "Matrix contains more nonexistent edges than it should; this is not a complete graph.",
        ))
    }};
    (DirectedGraph) => {{
        AdjacencyMatrixErrorType::DirectedGraph(String::from(
            "Matrix contains different values above and below the main diagonal; This is not an \
            undirected graph.",
        ))
    }};
    (SelfLoops) => {{
        AdjacencyMatrixErrorType::SelfLoops(String::from(
            "Matrix contains self-loops; This is not a simple graph.",
        ))
    }};
    (MultipleEqualPoints) => {{
        AdjacencyMatrixErrorType::MultipleEqualPoints(String::from(
            "Matrix contains multple vertices with the same coordinates; That's unsupported.",
        ))
    }};
    (UnequalSamePoints) => {{
        AdjacencyMatrixErrorType::UnequalSamePoints(String::from(
            "Matrix contains points in the same column that are not equal; Points in the same \
            column indicate an edge from any row (index) vertex to that point.",
        ))
    }};
    (IndexOutOfBounds) => {{
        PairsErrorType::IndexOutOfBounds(String::from(
            "UFDS doesn't contain \
            such index element.",
        ))
    }};
}

#[macro_export]
macro_rules! ensure_or {
    ($check:expr, $error:tt$(,)?) => {{ $check.then_some(()).ok_or_else(|| build_error!($error)) }};
}

impl From<AdjacencyMatrixErrorType> for AdjacencyMatrixError {
    fn from(value: AdjacencyMatrixErrorType) -> Self {
        Self(value)
    }
}

impl From<PairsErrorType> for PairsError {
    fn from(value: PairsErrorType) -> Self {
        Self(value)
    }
}

/// Computes the segment length of two [`Point2d`]s.
///
/// The implementation follows that for some segment denoted by two endpoints
/// `a` and `b`, the length of such a segment is bound to be equivalent to the
/// length of the hypotenuse denoted by that same segement of a right triangle.
/// This then exploits the R^2 elements (ordered pairs resulting from the
/// cartesian product of R x R) of each point to compute the sides of such a
/// triangle and solves through Pythagoras' Theorem.
#[must_use]
pub fn seglen(Point2d { x: x1, y: y1 }: Point2d, Point2d { x: x2, y: y2 }: Point2d) -> f64 {
    let (x_res, y_res) = ((x1.algebraic_sub(x2)).abs(), (y1.algebraic_sub(y2)).abs());

    x_res
        .algebraic_mul(x_res)
        .algebraic_add(y_res.algebraic_mul(y_res))
        .sqrt()
}

impl AdjacencyMatrix {
    /// # Errors
    ///
    /// May fail if:
    /// 1. the input matrix is not square, or
    /// 2. the input matrix denotes a non-simple graph (specifically, containing
    ///    self loops,) or
    /// 3. the input matrix denotes an incomplete graph, or
    /// 4. the input matrix denotes a directed graph (with elements below the
    ///    main diagonal that do not map 1:1 to the elements above the main
    ///    diagonal.)
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

        Ok(Self(input.into()))
    }
}

impl<'a> Pairs<'a> {
    #[must_use]
    pub fn new(src: &'a AdjacencyMatrix) -> Self {
        Self {
            forest: (0..src.0.len()).collect(),
            current_node: None,
            current_tree: None,
            current_product: vec![],
            current_iter: None,
            src,
        }
    }
}

impl Pairs<'_> {
    /// # Errors
    ///
    /// Fails if any one of `this` or `other` are not indices pointing to valid
    /// nodes in the underlying UFDS.
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
    ///
    /// # Errors
    ///
    /// Fails if `this` denotes an index that does not point to a valid element
    /// of the underlying UFDS.
    pub fn find(&self, this: usize) -> Result<usize, PairsError> {
        ensure_or!(this < self.forest.len(), IndexOutOfBounds)?;
        match self.forest[this] {
            val if val == this => Ok(this),
            other => self.find(other),
        }
    }

    /// # Errors
    ///
    /// Fails if any one of `this` or `other` are not indices pointing to valid
    /// nodes in the underlying UFDS.
    pub fn same(&self, this: usize, other: usize) -> Result<bool, PairsError> {
        let (this, other) = (self.find(this)?, self.find(other)?);

        Ok(this == other)
    }
}

impl Pairs<'_> {
    /// # Errors
    ///
    /// Fails if `this` denotes an index that does not point to a valid element
    /// of the underlying UFDS.
    pub fn ancestors(&self, this: usize) -> Result<Vec<usize>, PairsError> {
        let (this_root, mut parent, mut ancestors) = (self.find(this)?, this, vec![this]);

        while parent != this_root {
            parent = self.forest[parent];
            ancestors.push(parent);
        }

        ancestors.reverse();

        Ok(ancestors)
    }

    /// # Panics
    ///
    /// Can't really panic. See [Errors].
    ///
    /// # Errors
    ///
    /// Fails if `this` denotes an index that does not point to a valid element
    /// of the underlying UFDS.
    ///
    /// [Errors]: #errors-4
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

    /// # Panics
    ///
    /// Panics if `self.current_tree` is not `Some(_)`. This happens most
    /// naturaly as a consequence of calling this outside iterator chains.
    pub fn cartesian_product(&mut self) {
        static ERROR_MSG: LazyLock<&str> =
            LazyLock::new(|| "this method should not be called outside iterator chains");

        let current_tree = self.current_tree.as_ref().expect(&ERROR_MSG);
        let others: Vec<_> = (0..self.forest.len())
            .filter(|&node| current_tree.iter().all(|&tree_node| tree_node != node))
            .collect();

        self.current_product = iter::repeat_n(self.current_node.expect(&ERROR_MSG), others.len())
            .zip(others)
            .collect();
    }

    // NOTE: this exists as a replacement for the `min()` override of
    //       `Iterator`, as that doesn't seem to resolve to the overridden
    //       implementation when used in `tsp()` of `TspClosestPair`.
    pub fn min_fix(&mut self) -> Option<<Self as Iterator>::Item> {
        self.min_by_key(|&(node1, node2)| {
            let Edge::Weighted(weight) = self.src.0[node1][node2] else {
                unreachable!(
                    "no node considered in the `Pairs` iterator should be `Edge::NonExistent`",
                )
            };

            weight
        })
    }

    /// # Panics
    ///
    /// Panics if the method is called before the underlying UFDS isn't made up
    /// of a single tree (contrary to multiple trees in the initial forest.)
    #[must_use]
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
                self.cartesian_product();
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
                            self.cartesian_product();
                            self.current_iter = Some(0);
                        }
                        Ordering::Greater => unreachable!(
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
        let matrix = &self.src.0;
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
    /// # Errors
    ///
    /// May fail if:
    /// 1. the input matrix is not square, or
    /// 2. the input matrix denotes a non-simple graph (specifically, containing
    ///    self loops,) or
    /// 3. the input matrix denotes an incomplete graph, or
    /// 4. the input matrix denotes a directed graph (with elements below the
    ///    main diagonal that do not map 1:1 to the elements above the main
    ///    diagonal,) or
    /// 5. the input matrix contains some row (denoting the edges of the vertex
    ///    represented by that row's index) with the same points, which is not
    ///    possible as all columns must represent the same point and thus every
    ///    row must contain a different sequence of points, or
    /// 6. if the input matrix has dimensionality greater than 2 (there's more
    ///    than 2 vertices in the graph,) then it contains some element in some
    ///    column that is not equal to the same element in the same column, as
    ///    commented on the prior failure condition.
    pub fn new(inner: &[Vec<GeoEdge>]) -> Result<Self, AdjacencyMatrixError> {
        ensure_or!(inner.len() > 1, NonSquareMatrix)?;

        inner.iter().enumerate().try_for_each(|(vertex, edges)| {
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
                    .fold(HashSet::new(), |mut accum, &(_, (_, point))| {
                        if !accum.contains(&point) {
                            accum.insert(point);
                            return accum;
                        }

                        accum
                    })
                    .len()
                    == row_vec.len(),
                MultipleEqualPoints,
            )?;

            // Square matrices with dimensionality 2 don't have any other
            // elements in the same column that are not `GeoEdge::NonExistent`.
            (vertex == 0 && inner.len() > 2)
                .then_some(())
                .into_iter()
                .try_for_each(|()| {
                    ensure_or!(
                        row_vec.iter().all(|&(vertex, (_, &point))| {
                            inner
                                .iter()
                                .skip(1)
                                .filter_map(|elem| {
                                    elem.iter().enumerate().find_map(|(idx, elem)| {
                                        if let GeoEdge::Weighted { coord, .. } = elem
                                            && idx == vertex
                                        {
                                            Some(*coord)
                                        } else {
                                            None
                                        }
                                    })
                                })
                                .eq(iter::repeat_n(point, inner.len() - 2))
                        }),
                        UnequalSamePoints,
                    )?;

                    Ok::<_, AdjacencyMatrixError>(())
                })?;

            Ok::<_, AdjacencyMatrixError>(())
        })?;

        Ok(Self(inner.into()))
    }

    /// # Panics
    ///
    /// Panics if the input point set is empty.
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
            .filter_map(|(idx, &point)| {
                points
                    .iter()
                    .skip(idx + 1)
                    .map(|&other_point| seglen(point, other_point))
                    .max_by(f64::total_cmp)
            })
            .max_by(f64::total_cmp)
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
                                #[expect(
                                    clippy::cast_possible_truncation,
                                    clippy::cast_sign_loss,
                                    reason = "`seglen()` always produces positive numbers, and the \
                                             problem space doesn't allow for arbitrary `f64` \
                                             values."
                                )]
                                weight: seglen(points[row], points[col])
                                    .algebraic_mul(100.)
                                    .algebraic_div(largest_distance)
                                    as usize,
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
    #[must_use]
    fn compute_triangle_area(t: (Point2d, Point2d, Point2d)) -> f64 {
        Self::compute_raw_triangle_area(t)
            .abs()
            .algebraic_div(2.0_f64)
    }
    #[must_use]
    fn compute_raw_triangle_area((a, b, c): (Point2d, Point2d, Point2d)) -> f64 {
        b.x.algebraic_sub(a.x)
            .algebraic_mul(c.y.algebraic_sub(a.y))
            .algebraic_sub(c.x.algebraic_sub(a.x).algebraic_mul(b.y.algebraic_sub(a.y)))
    }
    #[must_use]
    fn find_ring((a, b, c): (Point2d, Point2d, Point2d)) -> Option<Point2d> {
        #![expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "The absolute value is always taken and `floor()` gets me a \"floating point \
                     integer\"; The problem space doesn't allow for arbitrarily large values for \
                     the elements of the point set and I assume all point are given with respect \
                     to an axes-aligned box that considers only positive values."
        )]

        (a.x.algebraic_sub(b.x).abs().floor() as usize == 0_usize
            && c.y.algebraic_sub(b.y).abs().floor() as usize == 0_usize
            && b.y
                .algebraic_sub(a.y)
                .algebraic_div(a.x.algebraic_sub(b.x))
                .algebraic_mul(b.x.algebraic_sub(c.x).algebraic_div(c.y.algebraic_sub(b.y)))
                .floor() as usize
                != 1_usize)
            .then(|| {
                let (c0, c1, c2, c3) = (
                    a.x.algebraic_mul(a.x)
                        .algebraic_add(a.y.algebraic_mul(a.y))
                        .algebraic_sub(b.x.algebraic_mul(b.x).algebraic_sub(b.y.algebraic_mul(b.y)))
                        .algebraic_div(2.0_f64.algebraic_mul(a.x.algebraic_sub(b.x))),
                    b.x.algebraic_sub(c.x).algebraic_div(c.y.algebraic_sub(b.y)),
                    c.x.algebraic_mul(c.x)
                        .algebraic_add(c.y.algebraic_mul(c.y))
                        .algebraic_sub(b.x.algebraic_mul(b.x).algebraic_sub(b.y.algebraic_mul(b.y)))
                        .algebraic_div(2.0_f64.algebraic_mul(c.y.algebraic_sub(b.y))),
                    b.y.algebraic_sub(a.y).algebraic_div(a.x.algebraic_sub(b.x)),
                );
                let y_component = c0
                    .algebraic_mul(c1)
                    .algebraic_add(c2)
                    .algebraic_div(1.0_f64.algebraic_sub(c3.algebraic_mul(c1)));
                let x_component = y_component.algebraic_mul(c3).algebraic_add(c0);

                Point2d {
                    x: x_component,
                    y: y_component,
                }
            })
    }
    #[must_use]
    fn check_point_ownership((a, b, c): (Point2d, Point2d, Point2d), p_to_check: Point2d) -> bool {
        #![expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "The value won't be truncted because it's already a \"floating point \
                     integer\" thanks to `floor()`, and the known positive values (thanks to \
                     `abs()`) are well within bounds of `usize`."
        )]

        let container_area = Self::compute_triangle_area((a, b, c));
        let (area_0, area_1, area_2) = {
            let (t_0, t_1, t_2) = ((a, b, p_to_check), (a, c, p_to_check), (b, c, p_to_check));

            (
                Self::compute_triangle_area(t_0),
                Self::compute_triangle_area(t_1),
                Self::compute_triangle_area(t_2),
            )
        };

        container_area
            .algebraic_sub(area_0.algebraic_add(area_1).algebraic_add(area_2))
            .abs()
            .floor() as usize
            == 0_usize
    }
    fn build_hull(
        &self,
        triangulation: &mut [Vec<GeoEdge>],
        hull: &mut Vec<(usize, Point2d)>,
        compare: impl Fn(Point2d, Point2d, Point2d) -> bool,
        points: &[(usize, Point2d)],
    );
    fn optimize_triangulation(&self, triangulation: Vec<Vec<GeoEdge>>);
    fn triangulate(&mut self, points: Vec<Point2d>);

    fn mst(&self) -> Vec<usize>;
    fn dfs(&self) -> Vec<usize>;

    fn tsp(&self) -> Vec<usize>;
}

impl TspNearestNeighbor for AdjacencyMatrix {
    fn tsp(&self) -> Vec<usize> {
        let mut visited = vec![false; self.0.len()];
        let mut output = Vec::new();
        let mut current_idx = 0;

        while visited.iter().any(|visited| !visited) {
            let current = &self.0[current_idx];

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
        for _ in 1..self.0.len() {
            static ERROR_MSG: LazyLock<&str> = LazyLock::new(|| {
                "`node2` was just sourced through `min_fix()` so the operation should be \
                infallible"
            });

            let (node1, node2) = pairs_iter.min_fix().expect(
                "there should always be a minimum value given the loop runs for n - 1 \
                iterations, where n is the number of nodes in the graph, and the underlying ufds \
                decreases its number of disjoint trees by a factor of 1 on each iteration (i.e. on \
                each call to `unite()` with the nodes yielded by `min_fix()`)",
            );

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
    /// Builds any one of the upper or lower convex hulls of a point set
    /// provided a corresponding comparison function, and builds an accompanying
    /// triangulation from the edges that got removed from the hull.
    ///
    /// The algorithm follows [Andrew, 1979]'s approach, with [Skiena, 2020]'s
    /// algorithm for building a triangulation from the points that are removed
    /// from the convex hull during construction.
    ///
    /// To determine which of the upper or lower hull to construct at a time,
    /// the function accepts a custom comparison function that should determine
    /// whether the last three points in the hull's boundary are "turning right"
    /// or "turning left." The use of trivial terminology is due to the
    /// possibility for such a comparison function to be determined in one of
    /// multiple ways. For the one used in [`triangulate()`], see
    /// Sec. 1.2.1 in [O'Rourke, 2001].
    ///
    /// [Andrew, 1979]: https://doi.org/10.1016/0020-0190(79)90072-3
    /// [Skiena, 2020]: https://doi.org/10.1007/978-3-030-54256-6
    /// [`triangulate()`]: Self::triangulate()
    /// [O'Rourke, 2001]: https://doi.org/10.1017/CBO9780511804120
    fn build_hull(
        &self,
        triangulation: &mut [Vec<GeoEdge>],
        hull: &mut Vec<(usize, Point2d)>,
        compare: impl Fn(Point2d, Point2d, Point2d) -> bool,
        points: &[(usize, Point2d)],
    ) {
        for &(vertex, point) in points {
            while hull.len() > 1
                && let Some((rm, _)) = {
                    let (_, prev_last) = hull[hull.len() - 2];

                    hull.pop_if(|(_, last)| compare(prev_last, *last, point))
                }
            {
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
                    self.0[prev][rm],
                    self.0[post][rm],
                    self.0[rm][prev],
                    self.0[rm][post],
                );
            }

            hull.push((vertex, point));
        }
    }
    /// Finds the best angle-optimal triangulation for a point set `self` given
    /// a starting triangulation `triangulation`.
    ///
    /// This follows the method of local maxima outlined in Sec. 9.1,
    /// [de Berg et. al., 2008].
    ///
    /// The algorithm is inefficient but I wanted to try out building a Delauney
    /// triangulation from a regular triangulation instead of going straight for
    /// an angle-optimal triangulation.
    ///
    /// Provided there are a finite number of possible triangulations in a fixed
    /// point set, we define an angle-optimal triangulation as one whose angle
    /// vector is lexicographically larger than some other triangulation for the
    /// same point set. The angle vector of some triangulation is denoted by the
    /// multiset of angles for each of the vertices in any one of the two
    /// triangles that a quadrilateral can be broken down into. When a
    /// quadrilateral is convex, two such possible combinations of triangles
    /// exist; It is in the optimality of one of these that a better
    /// triangulation can be found.
    ///
    /// To determine the optimality of an angle, we seek for non-bounding edges
    /// in the triangulation that can be flipped. We define edge-flipping as an
    /// operation whereby the quadrilateral formed from the two triangles
    /// incident to some such edge has the original edge removed, and a new edge
    /// added between the other two non-adjacent points in the quadrilateral.
    ///
    /// [de Berg et. al., 2008]: https://doi.org/10.1007/978-3-540-77974-2
    fn optimize_triangulation(&self, mut triangulation: Vec<Vec<GeoEdge>>) {
        while let Some(((src, dst), (p1, p2))) = triangulation
            .iter()
            .enumerate()
            // Only takes edges above the main diagonal; The triangulation is
            // stored as an adjacency matrix for an undirected, simple graph so
            // all other edges (below the main diagonal) are only flipped, and
            // the main diagonal is empty.
            .flat_map(|(src, row)| (0..row.len()).skip(src + 1).map(move |dst| (src, dst)))
            // Finds an edge in the triangulation that is determined to be
            // illegal by de Berg et. al.'s terminology.
            // It's failry straightforward; You try to find the two other points
            // that would make up a quadrilateral alongside the edge at hand
            // (denoted by (`src`, `dst`),) and then make sure the quadrilateral
            // is convex. We could fail at the start if the edge is not an inner
            // edge but rather a boundary edge of the convex hull, or we could
            // fail in finding a convex quadrilateral because one of the points
            // in the triangulation is a reflex vertex (i.e. you can find it
            // lying within the area of the triangle formed by the other three
            // vertices.) Then you check if there's a possibly better,
            // angle-wise, triangulation by checking for a consequence of
            // Thales' theorem (`find_ring()` at the end is part of that) and
            // perform edge flipping if that's the case.
            .find_map(|(src, dst)| {
                let GeoEdge::Weighted { coord: p_dst, .. } = &triangulation[src][dst] else {
                    return None;
                };
                let GeoEdge::Weighted { coord: p_src, .. } = &triangulation[dst][src] else {
                    return None;
                };

                // If we broke early, then we found (`p1`, `p2`); Otherwise, we
                // may have found them at the end or not found them at all.
                if let ControlFlow::Continue((Some((p1, p1_idx)), Some((p2, p2_idx))))
                | ControlFlow::Break((Some((p1, p1_idx)), Some((p2, p2_idx)))) = triangulation
                    [src]
                    .iter()
                    .enumerate()
                    .try_fold((None, None), |(p1, p2), (idx, edge)| {
                        let GeoEdge::Weighted { coord, .. } = edge else {
                            return ControlFlow::Continue((p1, p2));
                        };

                        // Checks if there's any neighbor to the current
                        // neighbor of `src` that is equivalent to `dst`, while
                        // also making sure we are not choosing a point that
                        // stems from `src` but can contain an actually valid
                        // point (because there's a different, non-convex
                        // quadrilateral nearby.)
                        let find_p = || {
                            triangulation[idx].iter().enumerate().find_map(
                                |(inner_idx, inner_edge)| {
                                    (inner_idx == dst
                                        && matches!(inner_edge, GeoEdge::Weighted { .. })
                                        && triangulation[src]
                                            .iter()
                                            .filter_map(|elem| {
                                                if let GeoEdge::Weighted {
                                                    coord: p_to_check, ..
                                                } = elem
                                                    && p_to_check != coord
                                                    && p_to_check != p_dst
                                                {
                                                    Some(p_to_check)
                                                } else {
                                                    None
                                                }
                                            })
                                            .all(|p_to_check| {
                                                !Self::check_point_ownership(
                                                    (*p_src, *p_dst, *coord),
                                                    *p_to_check,
                                                )
                                            }))
                                    .then_some((coord, idx))
                                },
                            )
                        };

                        match (p1, p2) {
                            (None, p2) => ControlFlow::Continue((find_p(), p2)),
                            (p1, None) => ControlFlow::Continue((p1, find_p())),
                            // None of the points are `None` so we've found all
                            // we needed.
                            other => ControlFlow::Break(other),
                        }
                    })
                    && !(Self::check_point_ownership((*p1, *p2, *p_src), *p_dst)
                        || Self::check_point_ownership((*p1, *p2, *p_dst), *p_src))
                    && let Some(ring_center) = Self::find_ring((*p_src, *p_dst, *p1))
                    && #[expect(
                        clippy::cast_possible_truncation,
                        reason = "`signum()` always returns -1., 1. or NaN; I am sure it will \
                                 never be NaN. Truncation won't happen as the problem space \
                                 doesn't allow for arbitrary `f64` values and both `ceil()` and \
                                 `floor()` yield \"floating point integers\"."
                    )]
                    ((seglen(ring_center, *p1) - seglen(ring_center, *p2)).ceil() as isize > 0)
                {
                    Some(((src, dst), (p1_idx, p2_idx)))
                } else {
                    None
                }
            })
        {
            // Recall `self` is outfit with edges between *any* pair of vertices
            // that don't form a self-loop; It is `triangulation` that only
            // considers a *proper subset* of those edges.
            (
                triangulation[src][dst],
                triangulation[dst][src],
                triangulation[p1][p2],
                triangulation[p2][p1],
            ) = (
                GeoEdge::NonExistent,
                GeoEdge::NonExistent,
                self.0[p1][p2],
                self.0[p2][p1],
            );
        }
    }
    /// Computes the Delauney trianguluation of a given point set and stores it
    /// in the adjacency matrix `self`.
    ///
    /// The method follows that for some point set _already_ embedded into the
    /// receiver, and a separate vector comprising only the point set (not
    /// stored as a graph,) it computes the convex hull of the point set and
    /// builds up the triangulation by adding to it all edges from the hull that
    /// end up discarded. Then, post-hull construction (lower and upper hull,)
    /// it uses the resulting hulls to add whichever boundary edges are not yet
    /// part of the triangulation because they never got the chance to be
    /// discarded in the first place. See [`build_hull()`] for more information
    /// on convex hull construction.
    ///
    /// Then it computes the best angle-optimal triangulation from the above
    /// triangulation. See [`optimize_triangulation()`] for more information on
    /// the method to obtain an angle optimal triangulation.
    ///
    /// [`build_hull()`]: Self::build_hull()
    /// [`optimize_triangulation()`]: Self::optimize_triangulation()
    fn triangulate(&mut self, points: Vec<Point2d>) {
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

        points.sort_unstable_by(
            |(_, Point2d { x: x1, y: y1 }), (_, Point2d { x: x2, y: y2 })| match x1.total_cmp(x2) {
                Ordering::Equal => y1.total_cmp(y2),
                other => other,
            },
        );
        {
            #![expect(
                clippy::float_cmp,
                reason = "`signum()` always returns -1., 1. or NaN; I am sure it will never be NaN."
            )]

            self.build_hull(
                &mut triangulation,
                &mut upper_hull,
                |prev_last, last, point| {
                    // If the area is negative, then `last` lies to the right of
                    // directed segment (`prev_last`, `point`), and it must be
                    // removed because it's a reflex vertex. See Sec. 1.2.1 in
                    // O'Rourke, 2001.
                    Self::compute_raw_triangle_area((prev_last, last, point)).signum() == -1.
                },
                &points,
            );
            self.build_hull(
                &mut triangulation,
                &mut lower_hull,
                |prev_last, last, point| {
                    Self::compute_raw_triangle_area((prev_last, last, point)).signum() == 1.
                },
                &points,
            );
        }

        let mut triangulate_bounds_of = |collection: Vec<(usize, Point2d)>| {
            collection
                .windows(2)
                .map(|inner| (inner[0].0, inner[1].0))
                .for_each(|(src, dst)| {
                    triangulation[src][dst] = self.0[src][dst];
                    triangulation[dst][src] = self.0[dst][src];
                });
        };

        triangulate_bounds_of(upper_hull);
        triangulate_bounds_of(lower_hull);

        self.optimize_triangulation(triangulation);
    }

    fn mst(&self) -> Vec<usize> {
        todo!();
    }
    fn dfs(&self) -> Vec<usize> {
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
            quadrilateral",
        )
    }

    #[test]
    fn basic_directed_graph() {
        assert!(
            matrix! {
                0, 2;
                3, 0;
            }
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
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
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
                err,
                AdjacencyMatrixErrorType::DirectedGraph(_)
            )),
            "should've thrown an error about the graph not being undirected",
        );
    }

    #[test]
    fn malformed_matrix() {
        assert!(
            matrix! {
                0, 2, 3;
                0, 2;
            }
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
                err,
                AdjacencyMatrixErrorType::NonSquareMatrix(_)
            )),
            "should've thrown an error about the matrix not being square, or a matrix for that \
            matter",
        );
    }

    #[test]
    fn malformed_geometric_matrix() {
        assert!(
            geomatrix! {
                (0., 0., 0), (0., 0., 2), (0., 0., 3);
                (0., 0., 0), (0., 0., 2);
            }
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
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
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
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
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
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
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
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
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
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
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
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
            .is_err_and(|AdjacencyMatrixError(err)| matches!(
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

    // TODO: update all the `points_macro*` tests and/or see into floating point
    //       imprecision in `GeoAdjacencyMatrix::from_point_set()`.

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
                (1.25, 2., 50),  (1.3, 5., 50),  (0.,  0.,  0);
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
                (0., 0., 73),  (1.3, 5., 29),  (0.,  0.,  0);
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
