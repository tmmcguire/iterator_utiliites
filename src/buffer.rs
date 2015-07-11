//! Iterator buffer: temporarily store and allow access to several elements of an iterator.

use std::ops::{Index,IndexMut};

/// A variable-size buffer reading from an iterator and providing
/// access to future elements of the stream.
pub struct IteratorBuffer<I:Iterator> {
    iterator: I,
    opening:  bool,
    closing:  bool,
    size:     usize,
    buffer:   Vec<I::Item>,
}

impl<I> IteratorBuffer<I> where I: Iterator, I::Item: Clone {

    /// Create a buffer for Iterator it, of size elements.
    pub fn new(it: I, size: usize) -> IteratorBuffer<I> {
        let mut ib = IteratorBuffer {
            iterator: it,
            opening:  true,
            closing:  false,
            size:     size + 1,
            buffer:   Vec::with_capacity(size + 1),
        };
        ib.fill();
        ib
    }

}

impl<I> IteratorBuffer<I> where I: Iterator, I::Item: Clone {

    /// Return true if the buffer has not yielded any elements from
    /// the contained iterator.
    pub fn is_opening(&self) -> bool { self.opening }

    /// Return false if the contained iterator has yielded None; the
    /// only remaining elements are in the buffer.
    pub fn is_closing(&self) -> bool { self.closing }

    /// The current length of the buffer. Pending elements from the
    /// iterator, if any, are not counted.
    pub fn len(&self) -> usize { self.buffer.len() }

    /// Provide read-only access to the buffer itself.
    pub fn buffer<'a>(&'a self) -> &'a [I::Item] {
        &self.buffer
    }

    /// Yield the next element from the buffer, or None if the buffer
    /// is empty and the iterator has terminated.
    pub fn pop(&mut self) -> Option<I::Item> {
        self.opening = false;
        self.fill();
        if self.buffer.is_empty() {
            None
        } else {
            let res = self.buffer.remove(0);
            self.fill();
            Some(res)
        }
    }

    /// Replace `len` elements from the buffer with copies of the
    /// contents of `replacement`.
    pub fn replace(&mut self, len: usize, replacement: &[I::Item]) {
        for _ in 0..len {
            self.buffer.remove(0);
        }
        for i in 0..replacement.len() {
            self.buffer.insert(i, replacement[i].clone());
        }
    }

    /// Fill the buffer from the iterator, setting closing if needed.
    fn fill(&mut self) {
        while !self.closing && self.buffer.len() < self.size {
            match self.iterator.next() {
                Some(item) => { self.buffer.push(item) }
                None       => { self.closing = true; }
            }
        }
    }
}

/// Functions for testing the contents of the buffer.
impl<I> IteratorBuffer<I> where I: Iterator, I::Item: Clone + PartialEq {

    /// Return true if the iterator stream starts with the
    /// prefix. This is true if the buffer has not yielded any
    /// elements, and if the buffer starts with the prefix. For this
    /// to be meaningful, the buffer must be larger than the prefix.
    pub fn starts_with(&self, prefix: &[I::Item]) -> bool {
        if self.opening && prefix.len() <= self.buffer.len() {
            self.buffer.starts_with(prefix)
        } else {
            false
        }
    }

    /// Return true if the iterator stream ends with the suffix. This
    /// is true if the buffer is closing (i.e. the iterator has
    /// returned None) and if the buffer ends with the suffix. For
    /// this to be meaningful, the buffer must be larger than the
    /// suffix.
    pub fn ends_with(&self, suffix: &[I::Item]) -> bool {
        if self.closing && suffix.len() == self.buffer.len() {
            self.buffer.ends_with(suffix)
        } else {
            false
        }
    }
}

impl<I> Index<usize> for IteratorBuffer<I> where I: Iterator, I::Item: Clone {
    type Output = I::Item;

    /// Provide access to an element in the buffer.
    fn index<'a>(&'a self, index: usize) -> &'a I::Item {
        &self.buffer[index]
    }
}

impl<I> IndexMut<usize> for IteratorBuffer<I> where I: Iterator, I::Item: Clone {

    /// Provide mutable access to an element in the buffer.
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut I::Item {
        self.fill();
        &mut self.buffer[index]
    }
}

#[test]
fn test1() {
    let mut ib = IteratorBuffer::new((0..4),2);
    assert_eq!(ib[0], 0);
    assert!(ib.starts_with(&[0,1]));
    assert_eq!(ib.pop(), Some(0));
    assert_eq!(ib[0], 1);
    assert!(!ib.starts_with(&[1,2]));
    ib[1] = 5;
    assert_eq!(ib.pop(), Some(1));
    assert!(ib.ends_with(&[5,3]));
    assert_eq!(ib.pop(), Some(5));
    assert_eq!(ib.pop(), Some(3));
    assert_eq!(ib.pop(), None);
}
