#[allow(unused)]
#[derive(Debug)]
struct AdjacencyMatrix {
    inner: Vec<Vec<Edge>>,
}

#[allow(unused)]
#[derive(Clone, PartialEq, Debug)]
enum Edge {
    Weighted(usize),
    NonExistent,
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
    ($($($row:literal),+);+ $(;)?) => {{
        AdjacencyMatrix::new(&[
            $(
                vec![
                    $(
                        {
                            let row: isize = $row;
                            if row == -1 { Edge::NonExistent } else { Edge::Weighted(row as usize) }
                        }
                    ),+
                ]
            ),+
        ])
    }};
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
trait TSPNearestNeighbor {
    fn tsp(&self) -> Vec<usize>;
}

#[allow(unused)]
trait TSPClosestPair {
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
    fn tsp(&self) -> Vec<usize> {
        todo!()
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
