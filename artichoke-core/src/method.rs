/// Method specs.

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Type {
    Class,
    Global,
    Instance,
    Module,
}

pub trait Spec<I, A, M> {
    fn method_type(&self) -> &Type;

    fn method(&self) -> M;

    fn name(&self) -> &str;

    fn argspec(&self) -> A;
}
