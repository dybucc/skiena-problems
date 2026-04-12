use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    iter,
    ptr::NonNull,
};

#[derive(Debug)]
pub struct Buffer(pub NonNull<[u8]>);

impl Buffer {
    pub fn new(layout: Layout, len: usize) -> Result<Self, AllocError> {
        unsafe {
            iter::once(layout.repeat(len).unwrap())
                .map(|(layout, _)| Global.allocate(layout).map(|buf| Self(buf)))
                .next()
                .unwrap_unchecked()
        }
    }
}
