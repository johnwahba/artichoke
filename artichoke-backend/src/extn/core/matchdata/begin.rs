//! [`MatchData#begin`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-begin)

use std::convert::TryFrom;
use std::mem;

use crate::convert::{Convert, RustBackedValue, TryConvert};
use crate::extn::core::matchdata::MatchData;
use crate::sys;
use crate::types::Int;
use crate::value::{Value, ValueLike};
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    IndexType,
    NoGroup,
    NoMatch,
}

#[derive(Debug, Clone)]
pub enum Args {
    Index(Int),
    Name(String),
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o\0";

    pub unsafe fn extract(interp: &Artichoke) -> Result<Self, Error> {
        let mut first = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            first.as_mut_ptr(),
        );
        let first = first.assume_init();
        if let Ok(index) = Int::try_convert(interp, Value::new(interp, first)) {
            Ok(Args::Index(index))
        } else if let Ok(name) = String::try_convert(interp, Value::new(interp, first)) {
            Ok(Args::Name(name))
        } else if let Ok(index) = Value::new(interp, first).funcall::<Int, _, _>("to_int", &[]) {
            Ok(Args::Index(index))
        } else {
            Err(Error::IndexType)
        }
    }
}

pub fn method(interp: &Artichoke, args: Args, value: &Value) -> Result<Value, Error> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let regex = (*borrow.regexp.regex).as_ref().ok_or(Error::Fatal)?;
    let match_against = &borrow.string[borrow.region.start..borrow.region.end];
    let captures = regex.captures(match_against).ok_or(Error::NoMatch)?;
    let index = match args {
        Args::Index(index) => {
            if index < 0 {
                // Positive Int must be usize
                let index = usize::try_from(-index).map_err(|_| Error::Fatal)?;
                captures.len().checked_sub(index).ok_or(Error::Fatal)?
            } else {
                // Positive Int must be usize
                usize::try_from(index).map_err(|_| Error::Fatal)?
            }
        }
        Args::Name(name) => {
            let index = regex
                .capture_names()
                .enumerate()
                .find(|(_idx, capture)| capture == &Some(name.as_str()))
                .ok_or(Error::NoGroup)?
                .0;
            let capture = regex.capture_locations().get(index).ok_or(Error::Fatal)?;
            capture.0
        }
    };
    let match_ = captures.get(index).ok_or(Error::NoMatch)?;
    let begin = match_against[0..match_.start()].chars().count();
    let begin = begin + borrow.region.start;
    let begin = Int::try_from(begin).map_err(|_| Error::Fatal)?;
    Ok(Value::convert(&interp, begin))
}
