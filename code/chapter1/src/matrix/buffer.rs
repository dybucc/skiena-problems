use std::{
  alloc::{AllocError, Allocator, Global, Layout},
  ptr::NonNull,
};

#[derive(Debug)]
pub struct Buffer {
  buf: NonNull<[u8]>,
  cap: usize,
}

impl Buffer {
  fn new(layout: Layout) -> Result<Self, AllocError> {
    Global.allocate(layout).map(|buf| Self { buf, cap: buf.len() })
  }
}

#[derive(Debug)]
pub struct RawMatrix {
  buf:  Buffer,
  dims: (usize, usize),
}

impl RawMatrix {
  fn new() -> Result<Self, AllocError> {}
}
