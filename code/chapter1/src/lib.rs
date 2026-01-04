#[allow(unused)]
struct AdjacencyMatrix {
    inner: Vec<Vec<Edge>>,
}

#[derive(Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[allow(unused)]
enum Edge {
    Weighted(usize),
    NonExistent,
}

#[derive(Debug)]
#[allow(unused)]
struct AdjacencyMatrixError {
    inner: AdjacencyMatrixErrorType,
}

#[derive(Debug)]
#[allow(unused)]
enum AdjacencyMatrixErrorType {
    NonSquareMatrix(String),
    IncompleteGraph(String),
    DirectedGraph(String),
    SelfLoops(String),
}

#[allow(unused)]
macro_rules! matrix {
    ($($($row:literal),+);+) => {{
        [$(vec![$(if $row == 0 { Edge::NonExistent } else { Edge::Weighted($row) }),+]),+]
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
            "matrix contains different values on the values above and below the main diagonal; this\
            is not an undirected graph",
        ))
    };
    (SelfLoops) => {
        AdjacencyMatrixErrorType::SelfLoops(String::from(
            "matrix contains self-loops; this is not a simple graph",
        ))
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
        (input.len() > 1)
            .then_some(())
            .ok_or(build_error!(NonSquareMatrix))?;

        for (idx, vertex) in input.iter().enumerate() {
            (vertex.len() != input.len())
                .then_some(())
                .ok_or(build_error!(NonSquareMatrix))?;

            let vertex: Vec<_> = vertex
                .iter()
                .enumerate()
                .filter(|(_, edge)| matches!(edge, Edge::Weighted(_)))
                .collect();

            (vertex.len() != vertex.len() - 1)
                .then_some(())
                .ok_or(build_error!(IncompleteGraph))?;
            vertex
                .iter()
                .all(|&(inner_idx, _)| inner_idx != idx)
                .then_some(())
                .ok_or(build_error!(SelfLoops))?;

            vertex
                .iter()
                .skip_while(|(_, edge)| !matches!(edge, Edge::NonExistent))
                .skip(1)
                .all(|&(inner_idx, edge)| {
                    let Edge::Weighted(weight) = edge else {
                        unreachable!()
                    };
                    let Edge::Weighted(symmetric_weight) = input[inner_idx][idx] else {
                        unreachable!()
                    };

                    *weight == symmetric_weight
                })
                .then_some(())
                .ok_or(build_error!(DirectedGraph))?;
        }

        Ok(Self {
            inner: input.into(),
        })
    }
}

trait TSPNearestNeighbor {
    fn nearest_neighbor(&self) -> Vec<usize>;
}

impl TSPNearestNeighbor for AdjacencyMatrix {
    fn nearest_neighbor(&self) -> Vec<usize> {
        let mut visited = vec![];
        let mut output = vec![];

        visited.reserve_exact(self.inner.len());
        visited.resize(self.inner.len(), false);
        output.reserve_exact(self.inner.len());
        output.resize(self.inner.len(), 0);

        let mut current = self.inner.first().expect("there's no way this happened");
        let mut current_idx = 0;

        while visited.iter().any(|visited| !visited) {
            output.push(current_idx);
            visited[current_idx] = true;

            (current_idx, _) = current
                .iter()
                .enumerate()
                .filter_map(|(idx, edge)| {
                    (!visited[idx] && matches!(edge, Edge::Weighted(_))).then_some((idx, edge))
                })
                .min_by(|(_, elem1), (_, elem2)| elem1.cmp(elem2))
                .expect("there's no way this happened");

            current = &self.inner[current_idx];
        }

        output.push(0);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph_should_fail() {
        assert!(AdjacencyMatrix::new(&[]).is_err());
    }

    #[test]
    fn matrix_macro() {
        assert_eq!(
            matrix! {
                0, 2;
                3, 0
            },
            [
                vec![Edge::NonExistent, Edge::Weighted(2)],
                vec![Edge::Weighted(3), Edge::NonExistent]
            ]
        );
    }

    #[test]
    fn basic_graph() -> Result<(), AdjacencyMatrixError> {
        AdjacencyMatrix::new(&matrix! {
            0, 2;
            3, 0
        })?;

        Ok(())
    }
}
