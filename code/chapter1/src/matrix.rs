#![expect(clippy::missing_panics_doc, reason = "WIP.")]

use std::{
  borrow::Borrow,
  iter,
  ops::{Index, IndexMut},
};

use itertools::Itertools;
use num_traits::Num;

pub mod buffer;
pub mod errors;

pub use self::{buffer::Buffer, errors::BuildError};

// FIXME: replace the inner storage strategy to whatever type ends up being
// exposed through the `buffer` module.
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

macro_rules! transpose {
  ($spec:tt $self:expr => $self_ty:expr) => {{
    // SAFETY: the borrow checker considers `self` to already be borrowed after
    // calling `transpose_buf`, so taking another exclusive reference to it is
    // "invalid". `out` is actually empty and only allocates a buffer with the
    // right (pointer) bit-width for the elements. The solution to avoid using
    // `unsafe` is made obvious from this, but I choose not to implement it that
    // way.
    macro_rules! spec {
      (own; $reference:expr,($i:expr, $j:expr)) => {
        $reference[$j][$i].clone()
      };
      (ref; $reference:expr,($i:expr, $j:expr)) => {
        &$reference[$j][$i]
      };
      (mut; $reference:expr,($i:expr, $j:expr)) => {
        unsafe { (&raw mut $reference[$j][$i]).as_mut_unchecked() }
      };
    }

    macro_rules! spec_ty {
      (own) => {
        T
      };
      (ref) => {
        &T
      };
      (mut) => {
        &mut T
      };
    }

    $self_ty(
      iter::once((transpose_buf::<_, spec_ty!($spec)>($self), $self))
        .map(|(mut out, Matrix(reference))| {
          (
            out.iter_mut().enumerate().for_each(|(i, t_row)| {
              (0..reference.len()).for_each(|j| t_row.push(
                spec!($spec; reference, (i, j))
              ));
            }),
            out,
          )
            .1
        })
        .next()
        .unwrap(),
    )
  }};
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
    transpose!(own self => Self)
  }

  /// Returns another matrix allocation with `self`'s elements borrowed and
  /// transposed.
  #[expect(
    clippy::must_use_candidate,
    reason = "It's not a bug not to use the output of this routine."
  )]
  pub fn transpose_ref(&self) -> Matrix<&T> { transpose!(ref self => Matrix) }

  /// Returns another matrix allocation with `self`'s elements exclusively
  /// borrowed and transposed.
  pub fn transpose_mut(&mut self) -> Matrix<&mut T> {
    transpose!(mut self => Matrix)
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

pub fn transpose_buf<T, R>(Matrix(matrix): &Matrix<T>) -> Vec<Vec<R>> {
  iter::once((matrix.len(), matrix.first().map(Vec::len).unwrap_or_default()))
    .map(|(rows, cols)| {
      iter::once(Vec::with_capacity(cols))
        .map(|mut out| {
          (out.resize_with(cols, || Vec::with_capacity(rows)), out).1
        })
        .next()
        .unwrap()
    })
    .next()
    .unwrap()
}

#[cfg(test)]
mod tests;
