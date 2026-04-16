use std::borrow::Borrow;

use num_traits::Num;

pub mod borrowed;
pub mod buffer;
pub mod errors;
pub mod raw_matrix;

pub use self::borrowed::{BorrowedMatrix, ExclusiveMatrix};

#[derive(Debug)]
pub struct Matrix<T = f64> {
    pub inner: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Default for Matrix<T> {
    fn default() -> Self {
        Self {
            inner: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }
}

impl<T, II: IntoIterator<Item = T>> FromIterator<II> for Matrix<T> {
    fn from_iter<I: IntoIterator<Item = II>>(iter: I) -> Self {
        let mut inner = Vec::new();

        let mut rows = 0;
        let mut cols = 0;

        for row in iter {
            rows += 1;

            for col in row {
                if rows == 1 {
                    cols += 1;
                }

                inner.push(col);
            }
        }

        Self { inner, rows, cols }
    }
}

impl<T: PartialEq, I: IntoIterator<Item: IntoIterator<Item = T>>> PartialEq<I> for Matrix<T> {
    fn eq(&self, other: &I) -> bool {
        self.inner.iter().eq(other
            .into_iter()
            .flat_map(|inner_iter| inner_iter.into_iter()))
    }
}

impl<T> Matrix<T> {
    #[expect(
        clippy::must_use_candidate,
        reason = "It's not a bug not to use the result of this routine."
    )]
    pub fn new() -> Self {
        Self::default()
    }

    #[expect(
        clippy::must_use_candidate,
        reason = "It's not a bug not to use the result of this routine."
    )]
    pub fn dims(rows: usize, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        let dims = rows * cols;

        let mut out = Vec::with_capacity(dims);
        out.resize(dims, T::default());

        Self {
            inner: out,
            rows,
            cols,
        }
    }

    pub fn with_dims(rows: usize, cols: usize, producer: impl Fn() -> T) -> Self {
        let dims = rows * cols;

        let mut out = Vec::with_capacity(dims);
        out.resize_with(dims, producer);

        Self {
            inner: out,
            rows,
            cols,
        }
    }
}

macro_rules! transpose {
    ($src:expr, $s:tt) => {{
        let Self {
            inner: src,
            rows,
            cols,
        } = $src;

        let mut inner = Vec::with_capacity(*cols * *rows);

        for i in 0..*cols {
            for j in 0..*rows {
                eprintln!("row: {j}, col: {i}");
                eprintln!("rows: {rows}, cols: {cols}");

                macro_rules! spec {
                    (own) => {
                        src[j * cols + i].clone()
                    };
                    (ref) => {
                        &src[j * cols + i]
                    };
                    (mut) => {
                        // SAFETY: in theory, because `self` is borrowed exclusively, and the return
                        // value contains a reference that is inferred to be that of `self`,
                        // modification of the matrix values should only happen within the returned
                        // transposed matrix.
                        unsafe { (&raw mut src[j * *cols + i]).as_mut_unchecked() }
                    };
                }

                inner.push(spec!($s));
            }
        }

        macro_rules! spec {
            (own) => {
                Matrix {
                    inner,
                    rows: *cols,
                    cols: *rows,
                }
            };
            (ref) => {
                BorrowedMatrix {
                    inn: Matrix {
                        inner,
                        rows: *cols,
                        cols: *rows,
                    },
                    _marker: ::std::marker::PhantomData,
                }
            };
            (mut) => {
                ExclusiveMatrix {
                    inn: Matrix {
                        inner,
                        rows: *cols,
                        cols: *rows,
                    },
                    _marker: ::std::marker::PhantomData,
                }
            };
        }

        spec!($s)
    }};
}

impl<T: Num> Matrix<T> {
    #[expect(
        clippy::must_use_candidate,
        clippy::return_self_not_must_use,
        reason = "It's not a bug not to use the output of this routine."
    )]
    pub fn transpose(&self) -> Self
    where
        T: Clone,
    {
        transpose!(self, own)
    }

    #[expect(
        clippy::must_use_candidate,
        reason = "It's not a bug not to use the output of this routine."
    )]
    pub fn transpose_ref(&self) -> BorrowedMatrix<'_, T> {
        transpose!(self, ref)
    }

    pub fn transpose_mut(&mut self) -> ExclusiveMatrix<'_, T> {
        transpose!(self, mut)
    }

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
        todo!()
    }
}

#[cfg(test)]
mod tests;
