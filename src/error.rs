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

impl BinaryError {
    pub fn get_message(&self) -> String {
        match self {
            Self::OutOfBounds(offset, length, append) => {
                format!("Offset {} out of range for a buffer size with: {}. {}", offset, length, append)
            },
            Self::EOF(length) => format!("Buffer reached End Of File at offset: {}", length),
            Self::RecoverableKnown(msg) => msg.clone(),
            Self::RecoverableUnknown => "An interruption occurred when performing a binary operation, however this error was recovered safely.".to_string()
        }
    }
}

impl From<std::io::Error> for BinaryError {
    fn from(_error: std::io::Error) -> Self {
        Self::RecoverableUnknown
    }
}

impl std::fmt::Display for BinaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_message())
    }
}
