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

impl<T: PartialEq, const N: usize, const M: usize> PartialEq<[[T; M]; N]> for Matrix<T> {
    fn eq(&self, other: &[[T; M]; N]) -> bool {
        self.inner
            .iter()
            .eq(other.iter().flat_map(IntoIterator::into_iter))
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

        #[cfg(debug_assertions)]
        let mut stderr = ::std::io::stderr().lock();

        for i in 0..*cols {
            for j in 0..*rows {
                #[cfg(debug_assertions)]
                {
                    use std::io::Write;

                    writeln!(stderr, "row: {j}, col: {i}").unwrap();
                    writeln!(stderr, "rows: {rows}, cols: {cols}").unwrap();
                }

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

    pub fn offset(&self, row: usize, col: usize) -> usize {
        let Self { cols, .. } = self;

        row * cols + col
    }

    pub fn transpose_in_place(&mut self) {
        let Self { rows, cols, .. } = *self;
        let Self { inner, .. } = self;

        // mem layout: 1 2 | 3 4 | 5 6
        // transposed mem layout: 1 3 5 | 2 4 6
        // 1a 2a 3b 4b 5c 6c
        // 1a 3b 5c 2a 4b 6c
        //
        // mem layout: 1 3 5 | 2 4 6
        // transposed mem layout: 1 2 | 3 4 | 5 6
        // 1a 3a 5a 2b 4b 6b
        // 1a 5a 3a 2b 4b 6b
        // 1a 2b 3a 4b 5a 6b
        let mut current = (0, 1);
        let mut iter_counter = inner.len() / rows;
        while iter_counter != 0 {
            // TODO: obtain the buffer offset from the abstract ordered pair.
            let (elem, idx) = {
                let (row, col) = current;
                let next_idx = col * rows + row;
                let next_elem = {
                    let nrow = rows - (rows * cols - next_idx) / cols - 1;
                    (nrow, next_idx - cols * nrow)
                };

                (next_elem, next_idx)
            };
            // inner.swap(a, b);
            current = elem;
            iter_counter -= 1;
        }
        #[allow(clippy::never_loop)]
        loop {
            break;
        }

        self.rows = cols;
        self.cols = rows;
    }
}

pub fn solver(rows: usize, cols: usize, elem: (usize, usize)) -> (usize, usize) {
    todo!()
}

#[cfg(test)]
mod tests;
