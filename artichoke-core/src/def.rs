use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::method;
use crate::value::Value;
use crate::ArtichokeError;

/// Typesafe wrapper for the Ruby [`ClassLike`] representing the enclosing scope
/// for an Ruby `Module` or `Class`.
///
/// In Ruby, classes and modules can be defined inside of another class or
/// module.
#[derive(Clone, Debug)]
pub enum EnclosingRubyScope<I, M, A> {
    /// Reference to a Ruby `Class` enclosing scope.
    Class {
        /// Shared copy of the underlying class definition.
        spec: Rc<RefCell<dyn ClassLike<I, M, A>>>,
    },
    /// Reference to a Ruby `Module` enclosing scope.
    Module {
        /// Shared copy of the underlying module definition.
        spec: Rc<RefCell<dyn ClassLike<I, M, A>>>,
    },
}

impl<I, M, A> EnclosingRubyScope<I, M, A> {
    /// Factory for [`EnclosingRubyScope::Class`] that clones an `Rc` smart
    /// pointer wrapped class spec.
    #[allow(clippy::needless_pass_by_value)]
    pub fn class(spec: Rc<RefCell<dyn ClassLike<I, M, A>>>) -> Self {
        EnclosingRubyScope::Class {
            spec: Rc::clone(&spec),
        }
    }

    /// Factory for [`EnclosingRubyScope::Module`] that clones an `Rc` smart
    /// pointer wrapped module spec.
    #[allow(clippy::needless_pass_by_value)]
    pub fn module(spec: Rc<RefCell<dyn ClassLike<I, M, A>>>) -> Self {
        EnclosingRubyScope::Module {
            spec: Rc::clone(&spec),
        }
    }

    /// Get the fully qualified name of the wrapped [`ClassLike`].
    ///
    /// For example, in the following Ruby code, `C` has an fqname of `A::B::C`.
    ///
    /// ```ruby
    /// module A
    ///   class B
    ///     module C
    ///       CONST = 1
    ///     end
    ///   end
    /// end
    /// ```
    ///
    /// The current implemention results in recursive calls to this function
    /// for each enclosing scope.
    pub fn fqname(&self) -> String {
        match self {
            EnclosingRubyScope::Class { spec } | EnclosingRubyScope::Module { spec } => {
                spec.borrow().fqname()
            }
        }
    }
}

impl<I, M, A> Eq for EnclosingRubyScope<I, M, A> {}

impl<I, M, A> PartialEq for EnclosingRubyScope<I, M, A> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EnclosingRubyScope::Class { .. }, EnclosingRubyScope::Class { .. })
            | (EnclosingRubyScope::Module { .. }, EnclosingRubyScope::Module { .. }) => {
                self.fqname() == other.fqname()
            }
            _ => false,
        }
    }
}

impl<I, M, A> Hash for EnclosingRubyScope<I, M, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fqname().hash(state)
    }
}

/// `Define` trait allows a type to install classes, modules, and
/// methods into an mruby interpreter.
pub trait Define<I, C> {
    /// Define the class or module and all of its methods into the interpreter.
    fn define(&self, interp: &I) -> Result<C, ArtichokeError>;
}

/// `ClassLike` trait unifies `class::Spec` and `module::Spec`.
pub trait ClassLike<I, M, A>
where
    Self: fmt::Debug + fmt::Display,
{
    fn as_value(&self) -> Box<dyn Value<I>>;

    fn add_method(&mut self, name: &str, method: M, args: A);

    fn add_self_method(&mut self, name: &str, method: M, args: A);

    fn methods(&self) -> &HashSet<Box<dyn method::Spec<I, A, M>>>;

    fn name(&self) -> &str;

    fn enclosing_scope(&self) -> Option<&EnclosingRubyScope<I, M, A>>;

    /// Compute the fully qualified name of a Class or module. See
    /// [`EnclosingRubyScope::fqname`].
    fn fqname(&self) -> String {
        if let Some(scope) = self.enclosing_scope() {
            format!("{}::{}", scope.fqname(), self.name())
        } else {
            self.name().to_owned()
        }
    }
}