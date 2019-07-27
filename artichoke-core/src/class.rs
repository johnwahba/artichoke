use crate::value::Value;

pub trait Spec<I> {
    fn new_instance(
        &self,
        interp: &I,
        args: &[Box<dyn Value<I>>],
        block: Option<Box<dyn Value<I>>>,
    ) -> Option<Box<dyn Value<I>>>;
}
