use crate::value::Value;

/// Return a [`Value`]-wrapped reference to `nil`.
pub trait Nil {
    /// Return a [`Value`]-wrapped reference to `nil`.
    fn nil(&self) -> Box<dyn Value<Self>>;
}
