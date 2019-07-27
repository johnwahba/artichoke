use crate::value::Value;
use crate::ArtichokeError;

const TOP_FILENAME: &str = "(eval)";

/// `Context` is used to manipulate the currently executing file in the
/// interpreter as files are required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    /// Value of the `__FILE__` magic constant that also appears in stack
    /// frames.
    pub filename: String,
}

impl Context {
    /// Create a new [`Context`].
    pub fn new<T>(filename: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            filename: filename.as_ref().to_owned(),
        }
    }

    /// Create a root, or default, [`Context`]. The root context sets the
    /// `__FILE__` magic constant to "(eval)".
    pub fn root() -> Self {
        Self::default()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            filename: TOP_FILENAME.to_owned(),
        }
    }
}

/// Interpreters that implement [`Eval`] expose methods for injecting code into
/// an interpreter and extracting [`Value`]s from the interpereter.
///
/// Implementations are expected to maintain a stack of [`Context`] objects that
/// maintain filename context across nested invocations of [`Eval::eval`].
#[allow(clippy::module_name_repetitions)]
pub trait Eval {
    /// Eval code on the interpreter using the current [`Context`] or
    /// [`Context::root`] if none is present on the stack.
    fn eval<T>(&self, code: T) -> Result<Box<dyn Value<Self>>, ArtichokeError>
    where
        T: AsRef<[u8]>;

    /// Eval code on the interpreter using the current [`Context`] or
    /// [`Context::root`] if none is present on the stack.
    ///
    /// Exceptions and panics will unwind past this call.
    fn unchecked_eval<T>(&self, code: T) -> Box<dyn Value<Self>>
    where
        T: AsRef<[u8]>;

    /// Eval code on the interpreter using a custom [`Context`].
    ///
    /// `Context` allows manipulating interpreter state before eval, for
    /// example, setting the `__FILE__` magic constant.
    fn eval_with_context<T>(
        &self,
        code: T,
        context: Context,
    ) -> Result<Box<dyn Value<Self>>, ArtichokeError>
    where
        T: AsRef<[u8]>;

    /// Eval code on the interpreter using a custom [`Context`].
    ///
    /// `Context` allows manipulating interpreter state before eval, for
    /// example, setting the `__FILE__` magic constant.
    ///
    /// Exceptions and panics will unwind past this call.
    fn unchecked_eval_with_context<T>(&self, code: T, context: Context) -> Box<dyn Value<Self>>
    where
        T: AsRef<[u8]>;

    /// Peek at the top of the [`Context`] stack.
    fn peek_context(&self) -> Option<&Context>;

    /// Push an [`Context`] onto the stack.
    fn push_context(&self, context: Context);

    /// Pop an [`Context`] from the stack.
    fn pop_context(&self);
}
