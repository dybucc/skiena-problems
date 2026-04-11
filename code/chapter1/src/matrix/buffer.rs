use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    ptr::NonNull,
};

#[derive(Debug)]
pub struct Buffer {
    pub buf: NonNull<[u8]>,
    pub elem_size: usize,
}

impl Buffer {
    pub fn new(layout: Layout, len: usize) -> Result<Self, AllocError> {
        let (layout, elem_size) = {
            let res = layout.repeat(len);
            res.unwrap()
        };
        let res = Global.allocate(layout);
        let map_res = |buf: NonNull<[u8]>| Self { buf, elem_size };
        res.map(map_res)
    }
}
