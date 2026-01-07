use std::{cmp::Ordering, iter::repeat_n, sync::LazyLock};

#[allow(unused)]
#[derive(Debug)]
struct AdjacencyMatrix {
    inner: Vec<Vec<Edge>>,
}

#[allow(unused)]
#[derive(Debug)]
struct Pairs<'a> {
    /// Holds the parent of each node (where the node itself is the index).
    vertices: Vec<usize>,
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

#[allow(unused)]
#[derive(Debug)]
struct PairsError {
    inner: PairsErrorType,
}

#[allow(unused)]
#[derive(Debug)]
enum PairsErrorType {
    IndexOutOfBounds(String),
}

#[allow(unused)]
#[derive(Clone, PartialEq, Debug)]
enum Edge {
    NonExistent,
    Weighted(usize),
}

#[allow(unused)]
#[derive(Debug)]
struct AdjacencyMatrixError {
    inner: AdjacencyMatrixErrorType,
}

#[allow(unused)]
#[derive(Debug)]
enum AdjacencyMatrixErrorType {
    NonSquareMatrix(String),
    IncompleteGraph(String),
    DirectedGraph(String),
    SelfLoops(String),
}

#[allow(unused)]
macro_rules! matrix {
    ($($($row:literal),+);+ $(;)?) => {
        AdjacencyMatrix::new(&[$(vec![$({
            let row: isize = $row;
            if row == -1 { Edge::NonExistent } else { Edge::Weighted(row as usize) }
        }),+]),+])
    };
}

macro_rules! build_error {
    (NonSquareMatrix) => {
        AdjacencyMatrixErrorType::NonSquareMatrix(String::from(
            "matrix is not square; adjacency matrices must be square",
        ))
    };
    (IncompleteGraph) => {
        AdjacencyMatrixErrorType::IncompleteGraph(String::from(
            "matrix contains more nonexistent edges than it should; this is not a complete graph",
        ))
    };
    (DirectedGraph) => {
        AdjacencyMatrixErrorType::DirectedGraph(String::from(
            "matrix contains different values on the values above and below the main diagonal; \
            this is not an undirected graph",
        ))
    };
    (SelfLoops) => {
        AdjacencyMatrixErrorType::SelfLoops(String::from(
            "matrix contains self-loops; this is not a simple graph",
        ))
    };
    (IndexOutOfBounds) => {
        PairsErrorType::IndexOutOfBounds(String::from("ufds doesn't contain such index element"))
    };
}

macro_rules! assure_or {
    ($check:expr, $error:tt) => {
        $check.then_some(()).ok_or_else(|| build_error!($error))
    };
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
    #[allow(unused)]
    fn new(input: &[Vec<Edge>]) -> Result<Self, AdjacencyMatrixError> {
        assure_or!(input.len() > 1, NonSquareMatrix)?;

        for (idx, vertex) in input.iter().enumerate() {
            assure_or!(vertex.len() == input.len(), NonSquareMatrix)?;

            let row_vec: Vec<_> = vertex
                .iter()
                .enumerate()
                .filter(|(_, edge)| matches!(edge, Edge::Weighted(_)))
                .collect();

            assure_or!(
                row_vec.iter().all(|&(inner_idx, _)| inner_idx != idx),
                SelfLoops
            )?;
            assure_or!(row_vec.len() == vertex.len() - 1, IncompleteGraph)?;

            assure_or!(
                row_vec.iter().all(|&(inner_idx, edge)| {
                    let Edge::Weighted(weight) = edge else {
                        unreachable!("there should at least be one weighted edge")
                    };
                    let Edge::Weighted(symmetric_weight) = input[inner_idx][idx] else {
                        unreachable!("there should at least be one symmetric weighted edge")
                    };

                    *weight == symmetric_weight
                }),
                DirectedGraph
            )?;
        }

        Ok(Self {
            inner: input.into(),
        })
    }
}

#[allow(unused)]
impl<'a> Pairs<'a> {
    fn new(matrix: &'a AdjacencyMatrix) -> Self {
        Self {
            vertices: (0..matrix.inner.len()).collect(),
            current_node: None,
            current_tree: None,
            current_product: vec![],
            current_iter: None,
            src: matrix,
        }
    }

    fn unite(&mut self, this: usize, other: usize) -> Result<(), PairsError> {
        if !self.same(this, other)? {
            self.vertices[other] = this;
        }

        Ok(())
    }

    fn find(&self, this: usize) -> Result<usize, PairsError> {
        assure_or!(this < self.vertices.len() - 1, IndexOutOfBounds)?;
        match self.vertices[this] {
            val if val == this => Ok(this),
            other => self.find(other),
        }
    }

    fn same(&self, this: usize, other: usize) -> Result<bool, PairsError> {
        let (this, other) = (self.find(this)?, self.find(other)?);

        Ok(this == other)
    }

    fn ancestors(&mut self, this: usize) -> Result<(), PairsError> {
        let this_root = Self::find(self, this)?;
        let mut parent = this;
        let mut ancestors = vec![parent];

        while parent != this_root {
            parent = self.vertices[parent];
            ancestors.push(parent);
        }

        ancestors.reverse();
        self.current_tree = Some(ancestors);

        Ok(())
    }

    fn cartesian_product(&mut self, this: usize) -> Result<(), PairsError> {
        static ERROR_MSG: LazyLock<&str> =
            LazyLock::new(|| "this method may not be called outside iterator chains");

        let current_tree = self.current_tree.as_ref().expect(&ERROR_MSG);
        let others: Vec<_> = self
            .vertices
            .iter()
            .filter(|&&node| current_tree.iter().all(|&tree_node| tree_node != node))
            .collect();

        self.current_product =
            repeat_n(self.current_node.as_ref().expect(&ERROR_MSG), others.len())
                .zip(others)
                .map(|(&node1, &node2)| (node1, node2))
                .collect();

        Ok(())
    }
}

impl Iterator for Pairs<'_> {
    type Item = (usize, usize);

    // TODO: get rid of the expects once testing proves design soundness.
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_node {
            None => {
                self.current_node = Some(0);
                self.current_tree = Some(vec![0]);
                self.cartesian_product(0).ok()?;
                self.current_iter = Some(0);
            }
            Some(mut current_node) => {
                if let Some(ref mut val) = self.current_iter {
                    match val.cmp(&&mut (self.current_product.len() - 1)) {
                        Ordering::Less => *val += 1,
                        Ordering::Equal => {
                            current_node = (current_node < self.vertices.len() - 1)
                                .then_some(current_node)?
                                + 1;

                            self.current_node = Some(current_node);
                            self.ancestors(current_node).expect(
                                "this operation should be \
                                infallible if the iteration indices (`self.current_node` and \
                                `self.current_iter`) have been correctly handled",
                            );
                            self.cartesian_product(current_node).expect(
                                "this operation should be \
                                infallible if the iteration indices (`self.current_node` and \
                                `self.current_iter`) have been correctly handled",
                            );
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

    fn min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let matrix = &self.src.inner;
        self.min_by_key(move |(node1, node2)| {
            if let Edge::Weighted(weight) = matrix[*node1][*node2] {
                weight
            } else {
                unreachable!(
                    "no edge in the iterator should be `Edge::NonExistent` after \
                    `self.cartesian_product()` has run"
                )
            }
        })
    }
}

#[allow(unused)]
trait TSPNearestNeighbor {
    fn tsp(&self) -> Vec<usize>;
}

#[allow(unused)]
trait TSPClosestPair {
    fn pairs(&'_ self) -> Pairs<'_>;
    fn tsp(&self) -> Vec<usize>;
}

impl TSPNearestNeighbor for AdjacencyMatrix {
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

impl TSPClosestPair for AdjacencyMatrix {
    fn pairs(&'_ self) -> Pairs<'_> {
        Pairs::new(self)
    }

    fn tsp(&self) -> Vec<usize> {
        todo!("this will be implemented once the `Pairs` implementation of `Iterator` is done")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph() {
        assert!(
            AdjacencyMatrix::new(&[]).is_err(),
            "should've thrown an error about the graph not being a square matrix, or really a \
            matrix"
        );
    }

    #[test]
    fn basic_graph() {
        assert!(
            matrix! {
                -1, 2;
                2, -1;
            }
            .is_ok(),
            "should've been an ok graph with two vertices and one weight 2 edge between them"
        );
    }

    #[test]
    fn basic_directed_graph() {
        assert!(
            matrix! {
                -1, 2;
                3, -1;
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::DirectedGraph(_))),
            "should've thrown an error about the graph not being undirected"
        );
    }

    #[test]
    fn malformed_matrix_graph() {
        assert!(
            matrix! {
                -1, 2, 3;
                -1, 2;
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::NonSquareMatrix(_))),
            "should've thrown an error about the matrix not being square, or a matrix for that \
            matter"
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
            "should've thrown an error about the graph having self-loops (i.e. the main diagonal is \
            not made out of -1)"
        );
    }

    #[test]
    fn basic_incomplete_graph() {
        assert!(
            matrix! {
                -1, -1;
                2, -1;
            }
            .is_err_and(|err| matches!(err.inner, AdjacencyMatrixErrorType::IncompleteGraph(_))),
            "should've thrown an error about the graph not having as many edges as a complete, \
            simple graph is expected to have (i.e. the matrix has -1 outside the main diagonal)"
        );
    }

    #[test]
    fn tsp_nearest_neighbor1() -> Result<(), AdjacencyMatrixError> {
        assert_eq!(
            TSPNearestNeighbor::tsp(&matrix! {
                -1, 1, 3;
                1, -1, 4;
                3, 4, -1;
            }?),
            vec![0, 1, 2, 0]
        );

        Ok(())
    }

    #[test]
    fn tsp_nearest_neighbor2() -> Result<(), AdjacencyMatrixError> {
        assert_eq!(
            TSPNearestNeighbor::tsp(&matrix! {
                -1, 3, 4, 4, 2;
                3, -1, 4, 2, 2;
                4, 4, -1, 3, 2;
                4, 2, 3, -1, 2;
                2, 2, 2, 2, -1;
            }?),
            vec![0, 4, 1, 3, 2, 0]
        );

        Ok(())
    }
}
