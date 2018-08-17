use failure::{Backtrace, Context, Fail};

use std::fmt;

/// An Error which can occur when accessing the Unsplash API.
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

/// Types of errors which can be raised by this crate.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Fail)]
pub enum ErrorKind {
    /// Raised when there is an issue with the request.
    #[fail(display = "Failed to send request.")]
    Request,

    /// Raised when the caller's supplied access_token or bearer doesn't have permission to access
    /// an endpoint.
    #[fail(display = "Not authorized to access endpoint.")]
    Forbidden,

    /// Raised when the response from Unsplash cannot be understood.
    #[fail(display = "Failed to parse response from Unsplash.")]
    MalformedResponse,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> { self.inner.cause() }

    fn backtrace(&self) -> Option<&Backtrace> { self.inner.backtrace() }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(inner: ErrorKind) -> Self { Error { inner: Context::new(inner) } }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self { Error { inner } }
}

impl Error {
    /// Returns the context of this error
    pub fn kind(&self) -> ErrorKind { *self.inner.get_context() }
}
