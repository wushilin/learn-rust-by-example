use std;

pub trait Factory<T> {
    fn Create(&mut self)->T;
    fn Destroy(&mut self, what:T);
}

pub struct Buffer {
    ptr: Vec<u8>,
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        return &self.ptr;
    }
}
pub struct ByteArrayFactory {
    buffer: Vec<Buffer>,
    size: usize,
    limit: usize
}

impl Factory<Buffer> for ByteArrayFactory {
    fn Create(&mut self) -> Buffer {
        if self.buffer.len() > 0 {
            return self.buffer.pop().unwrap();
        }

        let buffer = Vec::with_capacity(self.size);
        return Buffer{ptr:buffer};
    }

    fn Destroy (&mut self, what: Buffer) {
        if self.buffer.len() > self.limit {

        }
        self.buffer.push(what);
    }
}
pub fn new_byte_array_factory(buffer_size: usize, limit: usize)-> ByteArrayFactory {
    let mut buf:Vec<Buffer> = Vec::with_capacity(limit);
    return ByteArrayFactory{buffer:buf, size:buffer_size, limit};
}