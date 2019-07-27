use std::error;
use std::fmt;

use crate::value::{types, Value};

pub trait Convert<I, T> {
    type From;
    type To;

    fn convert(interp: &I, value: T) -> Self;
}

pub trait TryConvert<I, T>
where
    Self: Sized,
{
    type From;
    type To;

    unsafe fn try_convert(interp: &I, value: T) -> Result<Self, Error<Self::From, Self::To>>;
}

/// Provide a falible converter for types that implement an infallible
/// conversion.
impl<I, F, T> TryConvert<I, F> for T
where
    T: Convert<I, F>,
{
    type From = <Self as Convert<I, F>>::From;
    type To = <Self as Convert<I, F>>::To;

    unsafe fn try_convert(interp: &I, value: F) -> Result<Self, Error<Self::From, Self::To>> {
        Ok(T::convert(interp, value))
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Error<F, T> {
    from: F,
    to: T,
}

impl<F, T> fmt::Display for Error<F, T>
where
    F: fmt::Display,
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to convert from {} to {}", self.from, self.to)
    }
}

impl<F, T> fmt::Debug for Error<F, T>
where
    F: fmt::Display,
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<F, T> error::Error for Error<F, T>
where
    F: fmt::Display,
    T: fmt::Display,
{
    fn description(&self) -> &str {
        "Failed to convert types between ruby and rust"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl<I, V> Convert<I, V> for ()
where
    V: Value<I>,
{
    type From = types::Ruby;
    type To = types::Rust;

    fn convert(_interp: &I, _value: V) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::convert::Error;
    use crate::value::types::*;

    #[test]
    fn ruby_to_rust_error_display() {
        let err = Error {
            from: Ruby::Fixnum,
            to: Rust::Vec,
        };
        assert_eq!(
            format!("{}", err),
            "failed to convert from ruby Fixnum to rust Vec"
        );
    }

    #[test]
    fn ruby_to_rust_error_debug() {
        let err = Error {
            from: Ruby::Fixnum,
            to: Rust::Vec,
        };
        assert_eq!(
            format!("{:?}", err),
            "mruby conversion error: failed to convert from ruby Fixnum to rust Vec"
        );
    }

    #[test]
    fn rust_to_ruby_error_display() {
        let err = Error {
            from: Rust::Bool,
            to: Ruby::String,
        };
        assert_eq!(
            format!("{}", err),
            "failed to convert from rust bool to ruby String"
        );
    }

    #[test]
    fn rust_to_ruby_error_debug() {
        let err = Error {
            from: Rust::Bool,
            to: Ruby::String,
        };
        assert_eq!(
            format!("{:?}", err),
            "mruby conversion error: failed to convert from rust bool to ruby String"
        );
    }
}
