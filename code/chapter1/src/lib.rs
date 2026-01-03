struct AdjacencyMatrix(Vec<Vec<Edge>>);

#[derive(Clone)]
enum Edge {
    Weighted(usize),
    NonExistent,
}

struct AdjacencyMatrixError(AdjacencyMatrixErrorType);

enum AdjacencyMatrixErrorType {
    NonSquareMatrix(String),
    IncompleteGraph(String),
    DirectedGraph(String),
    SelfLoops(String),
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
        Self(value)
    }
}

impl AdjacencyMatrix {
    fn new(input: &[Vec<Edge>]) -> Result<Self, AdjacencyMatrixError> {
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
                        unimplemented!()
                    };
                    let Edge::Weighted(symmetric_weight) = input[inner_idx][idx] else {
                        unimplemented!()
                    };

                    *weight == symmetric_weight
                })
                .then_some(())
                .ok_or(build_error!(DirectedGraph))?;
        }
        Ok(Self(input.into()))
    }
}

trait TSPNearestNeighbor {
    fn nearest_neighbor(&self) -> Vec<&Edge>;
}

impl TSPNearestNeighbor for AdjacencyMatrix {
    fn nearest_neighbor(&self) -> Vec<&Edge> {
        let mut visited = vec![];
        let mut output = vec![];

        visited.reserve_exact(self.0.len());
        visited.resize(self.0.len(), false);

        let mut current = self
            .0
            .first()
            .expect("there's no way this happened")
            .first()
            .expect("there's no way this happened");
        let mut current_idx = 0;
        while visited.iter().any(|visited| !visited) {
            output.push(current);
            *visited
                .get_mut(current_idx)
                .expect("there's no way this happened") = true;
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
