use std::{
  borrow::Borrow,
  ops::{Index, IndexMut},
};

use itertools::Itertools;
use num_traits::Num;

pub mod buffer;
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
  #[expect(
    clippy::must_use_candidate,
    reason = "It's not a bug not to use the output of this routine."
  )]
  pub fn new() -> Self { Self(Vec::new()) }
}

/// Basic matrix operations on matrices of `R^(m times n)`.
impl<T: Num> Matrix<T> {
  /// Returns another matrix allocation that has `self`'s elements transposed.
  #[expect(
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    reason = "It's not a bug not to use the output of this routine."
  )]
  pub fn transpose(&self) -> Self
  where
    T: Clone,
  {
    let (Self(reference), mut out) = (self, transpose_buf::<_, T>(self));
    out.iter_mut().enumerate().for_each(|(i, t_row)| {
      (0..reference.len()).for_each(|j| t_row.push(reference[j][i].clone()));
    });

    Self(out)
  }

  /// Returns another matrix allocation with `self`'s elements borrowed and
  /// transposed.
  #[expect(
    clippy::must_use_candidate,
    reason = "It's not a bug not to use the output of this routine."
  )]
  pub fn transpose_ref(&self) -> Matrix<&T> {
    let (Self(reference), mut out) = (self, transpose_buf::<_, &T>(self));
    out.iter_mut().enumerate().for_each(|(i, t_row)| {
      (0..reference.len()).for_each(|j| t_row.push(&reference[j][i]));
    });

    Matrix(out)
  }

  /// Returns another matrix allocation with `self`'s elements exclusively
  /// borrowed and transposed.
  pub fn transpose_mut(&mut self) -> Matrix<&mut T> {
    let (mut out, Self(reference)) = (transpose_buf::<_, &mut T>(self), self);
    // SAFETY: the borrow checker considers that `self` is already borrowed
    // after calling `transpose_buf`, so taking another mutable reference to it
    // is invalid. `out` is actually empty and only allocates a buffer with the
    // right bit-width for the elements.
    out.iter_mut().enumerate().for_each(|(i, t_row)| {
      (0..reference.len()).for_each(|j| {
        t_row.push(unsafe { (&raw mut reference[j][i]).as_mut_unchecked() });
      });
    });

    Matrix(out)
  }

  /// Tranposes `self` without any extra memory allocations.
  pub fn transpose_in_place(&mut self) {
    // 3 4 4
    // 3 4 2
    // r: 2, c: 3
    // -----
    // 3 3
    // 4 4
    // 4 2
    // r: 3, c: 2
    //             |       Row 1       | |       Row 2       |
    // mem layout: | 3 4 4 | len | cap | | 3 4 2 | len | cap |
    //                 ^      ^     ^   ^
    //                 8      8     8   0                      byte(s)
    //                                  ^
    //                                  |
    // ----------------------------------
    // |
    // v
    // padding bytes shouldn't be required, as all elements are 8 bytes, and
    // thus the alignment requirement of `Vec` is 8 bytes, so any contiguous
    // `Vec` can be stored immediately after the previous `Vec`.
    // transpose mem layout:
    // |      Row 1      | |      Row 2      | |      Row 3      |
    // | 3 3 | len | cap | | 4 4 | len | cap | | 4 2 | len | cap |
    //    ^     ^     ^   ^
    //    8     8     8   0                                        byte(s)
    let (rows, cols) =
      (self.0.len(), self.0.first().map(Vec::len).unwrap_or_default());
    let Self(matrix) = self;
    todo!();
  }
}

pub fn transpose_buf<T, R>(matrix: &Matrix<T>) -> Vec<Vec<R>> {
  let (rows, cols) =
    (matrix.0.len(), matrix.0.first().map(Vec::len).unwrap_or_default());
  let mut out = Vec::with_capacity(cols);
  out.resize_with(cols, || {
    let mut out = Vec::new();

    (out.reserve(rows), out).1
  });

  out
}

#[cfg(test)]
mod tests;
