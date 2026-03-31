use std::{
  borrow::Borrow,
  mem,
  ops::{Index, IndexMut},
};

use itertools::Itertools;
use num_traits::Num;

pub mod errors;

#[derive(Debug, Default, Clone)]
pub struct Matrix<T = f64>(Vec<Vec<T>>);

impl<T> IndexMut<usize> for Matrix<T> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.0.index_mut(index)
  }
}

impl<T> Index<usize> for Matrix<T> {
  type Output = Vec<T>;

  fn index(&self, index: usize) -> &Self::Output { self.0.index(index) }
}

impl<T, A: Into<T>> From<Vec<Vec<A>>> for Matrix<T> {
  fn from(value: Vec<Vec<A>>) -> Self {
    Self(
      value
        .into_iter()
        .map(|row| row.into_iter().map_into().collect())
        .collect(),
    )
  }
}

impl<T: PartialEq, A: Borrow<T>> PartialEq<Vec<Vec<A>>> for Matrix<T> {
  fn eq(&self, other: &Vec<Vec<A>>) -> bool {
    self.0.iter().eq_by(other, |ours, theirs| {
      ours.iter().eq(theirs.iter().map(Borrow::borrow))
    })
  }
}

impl<T> Matrix<T> {
  pub fn new() -> Self { Self(Vec::new()) }
}

/// Basic matrix operations on matrices of `R^(m times n)`.
impl<T: Num> Matrix<T> {
  /// Returns another matrix allocation that has `self`'s elements transposed.
  pub fn transpose(&self) -> Self
  where
    T: Clone,
  {
    let (rows, cols) =
      (self.0.len(), self.0.first().map(Vec::len).unwrap_or_default());
    let (Self(reference), mut out) = (self, Vec::new());
    out.reserve_exact(cols);
    out.resize_with(cols, || {
      let mut out = Vec::new();

      (out.reserve_exact(rows), out).1
    });
    for (i, (t_row, row)) in out.iter_mut().zip(reference).enumerate() {
      (0..row.len()).for_each(|j| t_row.push(reference[j][i].clone()));
    }

    Self(out)
  }

  /// Tranposes `self` without any extra memory allocations.
  pub fn transpose_in_place(&mut self) {
    // 3 4 4
    // 3 4 2
    // -----
    // 3 3
    // 4 4
    // 4 2
    // r: 2, c: 3
    // mem layout: 3 4 4 3 4 2
    // transpose mem layout: 3 3 4 4 4 2
    let (rows, cols) =
      (self.0.len(), self.0.first().map(Vec::len).unwrap_or_default());
    let Self(matrix) = self;
    (0..cols).map(|i| (i, 0..rows)).for_each(|(col, row_range)| {
      row_range.for_each(|row| {
        mem::swap(&mut matrix[col][row], &mut matrix[row][col])
      });
    });
    todo!();
  }
}

#[cfg(test)]
mod tests;
