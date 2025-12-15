use core::{error, fmt};

use crate::{thin_box::ThinBox, Source, Trace};

#[ptr_meta::pointee]
trait ErrorTrace: fmt::Debug + fmt::Display + Send + Sync + 'static {}

impl<T> ErrorTrace for T where
    T: fmt::Debug + fmt::Display + Send + Sync + 'static + ?Sized
{
}

#[derive(Debug)]
struct ErrorWithTrace {
    error: BoxedError,
    trace: ThinBox<dyn ErrorTrace>,
}

impl fmt::Display for ErrorWithTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;
        write!(f, "trace: {}", self.trace)?;

        Ok(())
    }
}

impl error::Error for ErrorWithTrace {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.error.inner.source()
    }
}

/// An error type that preserves all detailed error messages. It is optimized
/// to fit in a single pointer.
#[derive(Debug)]
pub struct BoxedError {
    inner: ThinBox<dyn error::Error + Send + Sync + 'static>,
}

impl fmt::Display for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl error::Error for BoxedError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.inner.source()
    }
}

impl Trace for BoxedError {
    fn trace<R>(self, trace: R) -> Self
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
    {
        Self::new(ErrorWithTrace {
            error: self,
            // SAFETY: The provided closure returns the same pointer unsized to
            // a `dyn ErrorTrace`.
            trace: unsafe {
                ThinBox::new_unchecked(trace, |ptr| ptr as *mut _)
            },
        })
    }
}

impl Source for BoxedError {
    fn new<T: error::Error + Send + Sync + 'static>(source: T) -> Self {
        Self {
            // SAFETY: The provided closure returns the same pointer unsized to
            // a `dyn Error`.
            inner: unsafe {
                ThinBox::new_unchecked(source, |ptr| ptr as *mut _)
            },
        }
    }
}
