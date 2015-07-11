//! Iterator buffer: temporarily store and allow access to several elements of an iterator.

use std::ops::{Index,IndexMut};

pub struct IteratorBuffer<I:Iterator> {
    iterator: I,
    opening:  bool,
    closing:  bool,
    size:     usize,
    buffer:   Vec<I::Item>,
}

impl<I> IteratorBuffer<I> where I: Iterator, I::Item: Clone {

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

    pub fn is_opening(&self) -> bool { self.opening }
    pub fn is_closing(&self) -> bool { self.closing }

    pub fn len(&self) -> usize { self.buffer.len() }

    pub fn buffer<'a>(&'a self) -> &'a [I::Item] {
        &self.buffer
    }

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

    pub fn replace(&mut self, len: usize, replacement: &[I::Item]) {
        for _ in 0..len {
            self.buffer.remove(0);
        }
        for i in 0..replacement.len() {
            self.buffer.insert(i, replacement[i].clone());
        }
    }

    fn fill(&mut self) {
        while !self.closing && self.buffer.len() < self.size {
            match self.iterator.next() {
                Some(item) => { self.buffer.push(item) }
                None       => { self.closing = true; }
            }
        }
    }

    // fn start(&mut self) -> Option<char> {
    //     self.started = true;
    //     match self.iter.next() {
    //         Some(&ch) => { self.prev = ch;   Some(ch) }
    //         None      => { self.done = true; None }
    //     }
    // }

    // fn cont(&mut self) -> Option<char> {
    //     self.fill();
    //     for &(trans,replacement) in TRANSLATIONS {
    //         if self.matches(trans) {
    //             self.replace(trans.len(), replacement);
    //             break;
    //         }
    //     }
    //     let next = self.buffer.remove(0);
    //     match next {
    //         'h' if is_vowel(&self.prev) || is_vowel(&self.buffer[0]) => Some(self.prev),
    //         'h' =>                                                      self.cont(),
    //         'w' if is_vowel(&self.prev) =>                              Some(self.prev),
    //         'w' =>                                                      self.cont(),
    //         ch if ch == self.prev =>                                    self.cont(),
    //         ch => {
    //             self.prev = next;
    //             Some(next)
    //         }
    //     }
    // }
}

impl<I> IteratorBuffer<I> where I: Iterator, I::Item: Clone + PartialEq {

    pub fn starts_with(&self, prefix: &[I::Item]) -> bool {
        if self.opening && prefix.len() <= self.buffer.len() {
            self.buffer.starts_with(prefix)
        } else {
            false
        }
    }

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

    fn index<'a>(&'a self, index: usize) -> &'a I::Item {
        &self.buffer[index]
    }
}

impl<I> IndexMut<usize> for IteratorBuffer<I> where I: Iterator, I::Item: Clone {

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
