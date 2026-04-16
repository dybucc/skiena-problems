use std::{
    alloc::{AllocError, Layout},
    marker::PhantomData,
    mem::MaybeUninit,
};

use crate::matrix::buffer::Buffer;

#[derive(Debug)]
pub struct RawMatrix<T> {
    pub buf: Buffer,
    pub rows: usize,
    pub cols: usize,
    pub _marker: PhantomData<T>,
}

/// # Safety
///
/// Dropping a `RawMatrix` is only safe if each of the elements is already
/// initialized. Otherwise, it's UB.
impl<T> Drop for RawMatrix<T> {
    fn drop(&mut self) {
        let Buffer(buf) = self.buf;

        for i in 0..self.rows {
            for j in 0..self.cols {
                let ptr = unsafe { buf.byte_add(self.offset(i, j)).cast::<T>() };

                todo!(
                    "drop each element that `ptr` takes on, and then let `Buffer`'s `drop` run to \
                     deallocate the slice of bytes (which implies another TODO for `Buffer`'s \
                     `drop`)"
                );
            }
        }
    }
}

impl<T> RawMatrix<T> {
    const TYPE_SIZE: usize = size_of::<T>();

    pub fn new(rows: usize, cols: usize) -> Result<Self, AllocError> {
        Buffer::new(Layout::new::<T>(), rows * cols).map(|buf| Self {
            buf,
            rows,
            cols,
            _marker: PhantomData,
        })
    }

    #[expect(
        clippy::must_use_candidate,
        reason = "It's not a bug not to use the result of this routine."
    )]
    #[inline]
    pub fn offset(&self, row: usize, col: usize) -> usize {
        row * self.cols * Self::TYPE_SIZE + col * Self::TYPE_SIZE
    }

    pub fn grow(&mut self, additional_rows: usize, additional_cols: usize) {
        let buf = &mut self.buf;
        buf.realloc(additional_rows * additional_cols);
    }

    /// # Safety
    ///
    /// The provided indices into the matrix should be valid in that they should
    /// denote a valid position in memory (corresponding to the same provenance
    /// as the current matrix buffer allocation.)
    ///
    /// Note this method is meant to initialize some byte(s) in memory with the
    /// expected bit pattern of the generic type `T` over which this matrix
    /// abstracts.
    pub unsafe fn init(&mut self, row: usize, col: usize, val: T) {
        unsafe {
            self.buf
                .0
                .byte_add(self.offset(row, col))
                .cast::<MaybeUninit<T>>()
                .as_mut()
                .write(val)
        };
    }

    /// # Safety
    ///
    /// The provided indices into the matrix should be valid in that they should
    /// both denote a valid position in memory (corresponding to the current
    /// matrix allocation,) and should already be initialized.
    #[expect(
        clippy::must_use_candidate,
        reason = "It's not a bug not to use the result of this routine."
    )]
    pub unsafe fn get(&self, row: usize, col: usize) -> &T {
        unsafe {
            self.buf
                .0
                .byte_add(self.offset(row, col))
                .cast::<T>()
                .as_ref()
        }
    }

    /// # Safety
    ///
    /// The provided indices into the matrix should be valid in that they should
    /// both denote a valid position in memory (corresponding to the current
    /// matrix allocation,) and should already be initialized.
    pub unsafe fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        unsafe {
            self.buf
                .0
                .byte_add(self.offset(row, col))
                .cast::<T>()
                .as_mut()
        }
    }
}
