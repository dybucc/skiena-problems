use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    ptr::NonNull,
};

#[derive(Debug)]
pub struct Buffer {
    pub buf: NonNull<[u8]>,
    pub cap: usize,
}

impl Buffer {
    pub fn new(layout: Layout, len: usize) -> Result<Self, AllocError> {
        let layout = {
            let res = layout.repeat(len);
            let container_layout = res.unwrap();
            container_layout.0
        };
        let res = Global.allocate(layout);
        let map_res = |buf: NonNull<[u8]>| {
            let cap = buf.len();
            Self { buf, cap }
        };
        res.map(map_res)
    }
}
