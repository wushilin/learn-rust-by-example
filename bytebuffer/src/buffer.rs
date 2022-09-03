trait ByteBuffer {
    fn put_i8(num:i8);
    fn put_u8(num:u8);
    fn put_i16(num:i16);
    fn put_u16(num:u16);
    fn put_i32(num:i32);
    fn put_u32(num:u32);
    fn put_i64(num:i64);
    fn put_u64(num:u64);
    fn put_f32(num:f32);
    fn put_f64(num:f64);
    fn put_bytes(buf:dyn AsRef<[u8]>);
    fn put_bytes_ex(buf:dyn AsRef<[u8]>, pos:usize, offset: usize);
    fn limit() -> usize;
    fn expand_to(size:usize);
    fn shrink_to(size:usize);
    fn flip();
    fn get_u8() -> u8;
    fn get_i8() -> i8;
    fn get_u16() -> u16;
    fn get_i16() -> i16;
    fn get_u32() -> u32;
    fn get_i32() -> i32;
    fn get_u64() -> u64;
    fn get_i64() -> i64;
    fn get_f32() -> f32;
    fn get_f64() -> f64;
}