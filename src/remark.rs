use std::borrow::Cow;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

/// An error with a comment giving human-readable context about what was going on when it happened.
pub struct Remark<E> {
    /// The error being remarked upon.
    pub error: E,

    /// A human-readable comment giving context about the error.
    pub msg: Cow<'static, str>,
}

impl<E> Remark<E> {
    /// Attach a fixed message to an error.
    pub(crate) fn new_str(error: E, s: &'static str) -> Self {
        Self {
            msg: Cow::Borrowed(s),
            error,
        }
    }

    /// Attach an owned string message to an error.
    pub(crate) fn new_string(error: E, s: String) -> Self {
        Self {
            msg: Cow::Owned(s),
            error,
        }
    }
}

impl<E: Debug> Debug for Remark<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Pretend we're a map, because it's easier to read.
        // Writes a result like:
        //   {"remark message": ErrorDebugRepr { ...fields }}
        f.debug_map()
            .entry(&self.msg, &self.error)
            .finish()
    }
}

impl<E: Display> Display for Remark<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.msg, self.error)
    }
}

impl<E: Debug + Display> Error for Remark<E> {}

impl<E> std::ops::Deref for Remark<E> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.error
    }
}
