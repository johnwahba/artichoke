//! [`Regexp#=~`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-3D-7E)

use std::cmp;
use std::convert::TryFrom;
use std::mem;

use crate::convert::{Convert, RustBackedValue, TryConvert};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::Regexp;
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    NoImplicitConversionToString,
}

#[derive(Debug, Clone)]
pub struct Args {
    pub string: Option<String>,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o\0";

    pub unsafe fn extract(interp: &Artichoke) -> Result<Self, Error> {
        let mut string = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            string.as_mut_ptr(),
        );
        let string = string.assume_init();
        if let Ok(string) = <Option<String>>::try_convert(interp, Value::new(interp, string)) {
            Ok(Self { string })
        } else {
            Err(Error::NoImplicitConversionToString)
        }
    }
}

// TODO: extract named captures and assign to local variables, see GH-156.
//
// See: https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-3D-7E
pub fn method(interp: &Artichoke, args: Args, value: &Value) -> Result<Value, Error> {
    let mrb = interp.borrow().mrb;
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let regex = (*borrow.regex).as_ref().ok_or(Error::Fatal)?;
    let string = if let Some(string) = args.string {
        string
    } else {
        unsafe {
            let nil = Value::convert(interp, None::<Value>);
            sys::mrb_gv_set(mrb, interp.borrow_mut().sym_intern("$~"), nil.inner());
            return Ok(nil);
        }
    };
    let (matchdata, pos) = if let Some(captures) = regex.captures(string.as_str()) {
        let num_regexp_globals_to_set = {
            let num_previously_set_globals = interp.borrow().num_set_regexp_capture_globals;
            cmp::max(num_previously_set_globals, captures.len())
        };
        for group in 0..num_regexp_globals_to_set {
            let sym = if group == 0 {
                interp.borrow_mut().sym_intern("$&")
            } else {
                interp.borrow_mut().sym_intern(&format!("${}", group))
            };

            let value = Value::convert(&interp, captures.get(group).map(|m| m.as_str()));
            unsafe {
                sys::mrb_gv_set(mrb, sym, value.inner());
            }
        }
        interp.borrow_mut().num_set_regexp_capture_globals = captures.len();

        let matchdata = MatchData::new(string.as_str(), borrow.clone(), 0, string.len());
        let matchdata =
            unsafe { matchdata.try_into_ruby(&interp, None) }.map_err(|_| Error::Fatal)?;
        if let Some(match_) = captures.get(0) {
            let pre_match = &string[..match_.start()];
            let post_match = &string[match_.end()..];
            unsafe {
                let pre_match_sym = interp.borrow_mut().sym_intern("$`");
                sys::mrb_gv_set(
                    mrb,
                    pre_match_sym,
                    Value::convert(interp, pre_match).inner(),
                );
                let post_match_sym = interp.borrow_mut().sym_intern("$'");
                sys::mrb_gv_set(
                    mrb,
                    post_match_sym,
                    Value::convert(interp, post_match).inner(),
                );
            }
            (
                matchdata,
                Value::convert(interp, Int::try_from(match_.start()).ok()),
            )
        } else {
            (matchdata, Value::convert(interp, None::<Value>))
        }
    } else {
        unsafe {
            let last_match_sym = interp.borrow_mut().sym_intern("$~");
            sys::mrb_gv_set(
                mrb,
                last_match_sym,
                Value::convert(interp, None::<Value>).inner(),
            );
            let pre_match_sym = interp.borrow_mut().sym_intern("$`");
            sys::mrb_gv_set(
                mrb,
                pre_match_sym,
                Value::convert(interp, None::<Value>).inner(),
            );
            let post_match_sym = interp.borrow_mut().sym_intern("$'");
            sys::mrb_gv_set(
                mrb,
                post_match_sym,
                Value::convert(interp, None::<Value>).inner(),
            );
        }
        (
            Value::convert(interp, None::<Value>),
            Value::convert(interp, None::<Value>),
        )
    };
    unsafe {
        sys::mrb_gv_set(mrb, interp.borrow_mut().sym_intern("$~"), matchdata.inner());
    }
    Ok(pos)
}
