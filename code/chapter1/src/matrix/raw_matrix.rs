use std::{
    alloc::{AllocError, Layout},
    marker::PhantomData,
};

use crate::matrix::Buffer;

#[derive(Debug)]
pub struct RawMatrix<T> {
    pub buf: Buffer,
    pub dims: (usize, usize),
    pub _marker: PhantomData<T>,
}

impl<T> RawMatrix<T> {
    pub fn new(rows: usize, cols: usize) -> Result<Self, AllocError> {
        Buffer::new(Layout::new::<T>(), rows * cols).map(|buf| Self {
            buf,
            dims: (rows, cols),
            _marker: PhantomData,
        })
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
        // E.g. rows: 3, cols: 3
        // mem layout: 0 0 0 | 0 0 0 | 0 0 0 |
        //             ^
        //             |
        //             +-- buf (the data part of the slice pointer)
        let (_, cols) = self.dims;
        let Buffer(buf) = &self.buf;
        let type_size = size_of::<T>();
        let row_offset = row * cols * type_size;
        let col_offset = col * type_size;
        let elem_ptr = unsafe { buf.byte_add(row_offset + col_offset) };
        todo!()
    }
}
