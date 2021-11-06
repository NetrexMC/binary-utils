/// An enum consisting of a Binary Error
/// (recoverable)
#[derive(Debug, PartialEq)]
pub enum BinaryError {
    /// Offset is out of bounds
    ///
    /// **Tuple Values:**
    /// - `usize` = Given Offset.
    /// - `usize` = Stream length.
    /// - `&'static str` = Message to add on to the error.
    OutOfBounds(usize, usize, &'static str),

    /// Similar to `OutOfBounds` except it means;
    /// the stream tried to read more than possible.
    ///
    /// **Tuple Values:**
    /// - `usize` = Stream length.
    EOF(usize),

    /// A known error that was recoverable to safely proceed the stack.
    RecoverableKnown(String),

    /// An unknown error occurred, but it wasn't critical,
    /// we can safely proceed on the stack.
    RecoverableUnknown,
}

impl From<std::io::Error> for BinaryError {
    fn from(_error: std::io::Error) -> Self {
        Self::RecoverableUnknown
    }
}
