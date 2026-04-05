use std::ptr::NonNull;

#[derive(Debug)]
pub struct Buffer {
  buf: NonNull<[u8]>,
  cap: usize,
}

impl Buffer {
  fn new() -> Self { todo!() }
}
