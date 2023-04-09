/// Byte pools a specialized structure that allows you to reuse byte slices
/// instead of allocating new ones.
///
/// When an byteslice is returned to the pool, it is immediately reused.
/// Do not use this if you are using a `BinaryStream` in multiple threads.
/// This will cause latency issues.
pub struct BytePool {}
