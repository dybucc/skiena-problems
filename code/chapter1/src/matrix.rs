use std::{
  borrow::Borrow,
  ops::{Index, IndexMut},
};

use itertools::Itertools;
use num_traits::Num;

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
  pub fn transpose(&self) -> Self
  where
    T: Clone,
  {
    let Self(reference) = self;
    let mut out = Vec::with_capacity(reference.len());
    out.resize(self.0.len(), Vec::new());
    for (i, (t_row, row)) in out.iter_mut().zip(reference).enumerate() {
      t_row.reserve_exact(row.len());
      (0..row.len()).for_each(|j| t_row.push(reference[j][i].clone()));
    }

    Self(out)
  }

  pub fn transpose_in_place(&mut self) { let Self(matrix) = self; }
}

#[cfg(test)]
mod tests;
