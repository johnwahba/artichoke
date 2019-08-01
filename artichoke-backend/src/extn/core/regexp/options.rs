//! [`Regexp#options`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-options)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::regexp::Regexp;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Error> {
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let opts = borrow.literal_options.flags() | borrow.encoding.flags();
    Ok(Value::convert(interp, opts))
}
