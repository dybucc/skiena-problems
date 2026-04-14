use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    ptr::NonNull,
};

#[derive(Debug)]
pub struct Buffer(pub NonNull<[u8]>);

impl Buffer {
    #[inline]
    pub fn new(layout: Layout, len: usize) -> Result<Self, AllocError> {
        Global.allocate(layout.repeat(len).unwrap().0).map(Self)
    }

    pub fn realloc(&mut self, more: usize) {
        todo!()
    }
}
