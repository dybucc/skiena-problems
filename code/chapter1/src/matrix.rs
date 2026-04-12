use num_traits::Num;

pub mod buffer;
pub mod errors;
pub mod raw_matrix;

pub use self::{buffer::Buffer, errors::BuildError, raw_matrix::RawMatrix};

#[derive(Debug)]
pub struct Matrix<T = f64>(pub RawMatrix<T>);

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
        todo!()
    }

    /// Returns another matrix allocation with `self`'s elements borrowed and
    /// transposed.
    #[expect(
        clippy::must_use_candidate,
        reason = "It's not a bug not to use the output of this routine."
    )]
    pub fn transpose_ref(&self) -> Matrix<&T> {
        todo!()
    }

    /// Returns another matrix allocation with `self`'s elements exclusively
    /// borrowed and transposed.
    pub fn transpose_mut(&mut self) -> Matrix<&mut T> {
        todo!()
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
        todo!()
    }
}

#[cfg(test)]
mod tests;
