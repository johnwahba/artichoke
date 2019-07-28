use crate::types;
use crate::ArtichokeError;

/// Max argument count for function calls including initialize.
pub const FUNCALL_ARGC_MAX: usize = 16;

#[allow(clippy::module_name_repetitions)]
pub trait Value<I> {
    fn ruby_type(&self) -> types::Ruby;

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
