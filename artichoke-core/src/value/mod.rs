use crate::ArtichokeError;

pub mod types;

/// Max argument count for function calls including initialize.
///
/// Defined in `vm.c`.
pub const MRB_FUNCALL_ARGC_MAX: usize = 16;

#[allow(clippy::module_name_repetitions)]
pub trait Value<I> {
    fn ruby_type(&self) -> types::Ruby;

    /// Some type tags like [`MRB_TT_UNDEF`](sys::mrb_vtype::MRB_TT_UNDEF) are
    /// internal to the mruby VM and manipulating them with the [`sys`] API is
    /// unspecified and may result in a segfault.
    ///
    /// After extracting a [`sys::mrb_value`] from the interpreter, check to see
    /// if the value is [unreachable](types::Ruby::Unreachable) and propagate an
    /// [`ArtichokeError::UnreachableValue`](crate::ArtichokeError::UnreachableValue) error.
    ///
    /// See: <https://github.com/mruby/mruby/issues/4460>
    fn is_unreachable(&self) -> bool {
        self.ruby_type() == types::Ruby::Unreachable
    }

    /// Call `#to_s` on this [`Value`].
    ///
    /// This function can never fail.
    fn to_s(&self) -> String;

    /// Generate a debug representation of self.
    ///
    /// Format:
    ///
    /// ```ruby
    /// "#{self.class.name}<#{self.inspect}>"
    /// ```
    ///
    /// This function can never fail.
    fn to_s_debug(&self) -> String;

    /// Call `#inspect` on this [`Value`].
    ///
    /// This function can never fail.
    fn inspect(&self) -> String;

    /// Call `#freeze` on this [`Value`] and consume `self`.
    fn freeze(&mut self) -> Result<(), ArtichokeError>;

    /// Whether this `Value` response to the given method.
    fn respond_to(&self, method: &str) -> Result<bool, ArtichokeError>;
}
