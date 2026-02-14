use std::{collections::HashMap, result};

#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

type Result<T> = result::Result<T, String>;

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn triangulate(points: &[u8]) -> Result<Vec<u8>> {
    let mut proc_points: Vec<HashMap<&str, f64>> = Vec::new();
    ciborium::into_writer(points, &mut proc_points).map_err(|e| e.to_string())?;

    Ok(proc_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
