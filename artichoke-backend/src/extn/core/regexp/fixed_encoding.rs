//! [`Regexp#fixed_encoding?`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-fixed_encoding-3F)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::regexp::enc::Encoding;
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
    match borrow.encoding {
        Encoding::No if borrow.literal_options.flags() & Regexp::NOENCODING == 0 => {
            Ok(Value::convert(interp, false))
        }
        Encoding::Fixed | Encoding::No => Ok(Value::convert(interp, true)),
        Encoding::None => Ok(Value::convert(interp, false)),
    }
}
