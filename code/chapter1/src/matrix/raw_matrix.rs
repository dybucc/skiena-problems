use std::{
    alloc::{AllocError, Layout},
    marker::PhantomData,
};

use crate::matrix::{Buffer, Matrix};

#[derive(Debug)]
pub struct RawMatrix<T> {
    pub buf: Buffer,
    pub rows: usize,
    pub cols: usize,
    pub _marker: PhantomData<T>,
}

impl<T> RawMatrix<T> {
    pub fn new(rows: usize, cols: usize) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();
        let dims = rows * cols;
        let buf = {
            let res = Buffer::new(layout, dims);
            res?
        };
        let marker = PhantomData;
        let out = Self {
            buf,
            rows,
            cols,
            _marker: marker,
        };
        Ok(out)
    }

    pub fn into_matrix(self) -> Matrix<T> {
        let Buffer { mut buf, elem_size } = self.buf;
        let type_size = size_of::<T>();
        let padding = elem_size - type_size;
        let mut idx = 0;
        loop {
            let elem = {
                let offset = type_size * idx;
                let next_ptr = unsafe { buf.byte_add(offset) };
                let elem = next_ptr.cast::<T>();
            };
            buf = unsafe { buf.byte_add(type_size * idx + padding * idx) };
        }
        todo!()
    }
}
