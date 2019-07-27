use crate::value::Value;

/// Return a [`Value`]-wrapped reference to "top self".
///
/// Top self is the root object that evaled code is executed within. Global
/// methods, classes, and modules are defined in top self.
pub trait TopSelf {
    /// Return a [`Value`]-wrapped reference to "top self".
    ///
    /// Top self is the root object that evaled code is executed within. Global
    /// methods, classes, and modules are defined in top self.
    fn top_self(&self) -> Box<dyn Value<Self>>;
}
