//! An iterator that contains no elements.

use std::marker::PhantomData;

pub struct Empty<Elt> {
    phantom: PhantomData<Elt>
}

impl<Elt> Empty<Elt> {
    pub fn new() -> Empty<Elt> { Empty { phantom: PhantomData } }
}

impl<Elt> Iterator for Empty<Elt> {
    type Item = Elt;
    fn next(&mut self) -> Option<Self::Item> { None }
}
