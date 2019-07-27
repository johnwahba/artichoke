use crate::ArtichokeError;

pub trait File<I> {
    fn require(interp: &I) -> Result<(), ArtichokeError>;
}
