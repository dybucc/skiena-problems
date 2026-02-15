use std::{collections::HashMap, result};

use chapter1::{GeoAdjacencyMatrix, GeoEdge, Point2d, TspTriMstDfs};
use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

type Result<T> = result::Result<T, String>;

#[derive(Serialize)]
struct SerializedPoint2d {
    x: f64,
    y: f64,
}

impl From<Point2d> for SerializedPoint2d {
    fn from(value: Point2d) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Serialize)]
struct WeightedEdge {
    weight: usize,
    #[serde(flatten)]
    coord: SerializedPoint2d,
}

#[derive(Serialize)]
#[serde(transparent)]
struct SerializedGeoEdge(Option<WeightedEdge>);

impl From<GeoEdge> for SerializedGeoEdge {
    fn from(value: GeoEdge) -> Self {
        match value {
            GeoEdge::NonExistent => Self(None),
            GeoEdge::Weighted { weight, coord } => Self(Some(WeightedEdge {
                weight,
                coord: SerializedPoint2d::from(coord),
            })),
        }
    }
}

#[derive(Serialize)]
#[serde(transparent)]
struct SeralizedGeoAdjacencyMatrix(Vec<Vec<SerializedGeoEdge>>);

impl From<GeoAdjacencyMatrix> for SeralizedGeoAdjacencyMatrix {
    fn from(value: GeoAdjacencyMatrix) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|row_vec| row_vec.into_iter().map(SerializedGeoEdge::from).collect())
                .collect(),
        )
    }
}

/// # Errors
///
/// May fail if the input from the Typst side is not an array of key-value pairs
/// (`x`: <value>, `y`: <value>).
#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn triangulate(points: &[u8]) -> Result<Vec<u8>> {
    let (points, adjacency_matrix) =
        init(&ciborium::from_reader::<Vec<_>, _>(points).map_err(|e| e.to_string())?).ok_or(
            String::from("some point is not denoted by a key-value pair of x and y coordinates"),
        )?;
    let adjacency_matrix = SeralizedGeoAdjacencyMatrix::from(adjacency_matrix.triangulate(points));
    let mut buf = Vec::new();

    ciborium::into_writer(&adjacency_matrix, &mut buf).map_err(|e| e.to_string())?;

    Ok(buf)
}

fn init(raw_points: &[HashMap<String, f64>]) -> Option<(Vec<Point2d>, GeoAdjacencyMatrix)> {
    Some(GeoAdjacencyMatrix::from_point_set(
        raw_points.iter().try_fold(
            Vec::with_capacity(raw_points.len()),
            |mut point_vec, coord| {
                let (x, y) = (*coord.get("x")?, *coord.get("y")?);
                point_vec.push(Point2d { x, y });

                Some(point_vec)
            },
        )?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
