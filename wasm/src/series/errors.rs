use core::fmt;

use bitvec::prelude::BitVec;

#[derive(Debug)]
pub struct NonHashable;

impl fmt::Display for NonHashable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot hash column")
    }
}

#[derive(Debug)]
pub struct WrongType;

impl fmt::Display for WrongType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected type")
    }
}

type View<'a, T> = &'a [Option<T>];
pub type ViewResult<'a, T> = Result<View<'a, T>, WrongType>;

pub type FilterResult<'a> = Result<BitVec, WrongType>;
