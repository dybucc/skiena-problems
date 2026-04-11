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

    pub fn into_type_array(self) -> Matrix<T> {
        let buf = self.buf;
        let len = buf.cap;
        let buf = buf.buf;
        todo!()
    }
}
