use std::ptr::NonNull;

#[derive(Debug)]
struct Buffer {
  buf: NonNull<[u8]>,
  cap: usize,
}

impl Buffer {
    fn new() -> Self {
        Self { buf: , len: (), cap: () }
    }
}
