use std::{
    error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct BitBitError {
    /// Error message
    message: String,
    /// Extra error context
    hint: Option<String>,
    /// Error exit code
    exit_code: i32,
    /// Write message to console directly or print with BitBit identifier
    direct: bool,
}

impl BitBitError {
    #[inline]
    pub const fn _new(
        message: String,
        hint: Option<String>,
        exit_code: i32,
        direct: bool,
    ) -> BitBitError {
        BitBitError {
            message,
            hint,
            exit_code,
            direct,
        }
    }

    /// Generic version of `BitBitError::new()`
    pub fn _with(message: impl Display, hint: Option<impl Display>, direct: bool) -> BitBitError {
        BitBitError {
            message: message.to_string(),
            hint: if let Some(hint) = hint {
                Some(hint.to_string())
            } else {
                None
            },
            exit_code: 1,
            direct,
        }
    }

    /// Exit immediately with a BITBIT error
    pub fn err(message: impl Display) -> BitBitError {
        BitBitError {
            message: message.to_string(),
            hint: None,
            exit_code: 1,
            direct: false,
        }
    }

    /// Exit immediately with a BITBIT error and hint
    pub fn hint(message: impl Display, hint: impl Display) -> BitBitError {
        BitBitError {
            message: message.to_string(),
            hint: Some(hint.to_string()),
            exit_code: 1,
            direct: false,
        }
    }

    /// Exit immediately with a message
    pub fn _direct(message: impl Display) -> BitBitError {
        BitBitError {
            message: message.to_string(),
            hint: None,
            exit_code: 1,
            direct: true,
        }
    }

    #[inline]
    pub const fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

impl Display for BitBitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.direct {
            write!(f, "{}", self.message)
        } else {
            if let Some(hint) = &self.hint {
                write!(f, "BITBIT: {}! ({})", self.message, hint)
            } else {
                write!(f, "BITBIT: {}!", self.message)
            }
        }
    }
}

impl error::Error for BitBitError {}
