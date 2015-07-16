//! An iterator iterator over the equivalence classes in a slice.

use std;
use std::iter::Iterator;
use std::ops::Fn;

pub struct EqClIter<'t,T,F> where T:'t, F:Fn(&'t T,&'t T)->bool {
    vect: &'t [T],
    pred: F,
    last: usize,
}

impl<'t,T,F> Iterator for EqClIter<'t,T,F> where T:'t, F:Fn(&'t T,&'t T)->bool {
    type Item = std::slice::Iter<'t,T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last >= self.vect.len() {
            None
        } else {
            let mut i = self.last;
            while i < self.vect.len() && (self.pred)(&self.vect[self.last], &self.vect[i]) {
                i += 1;
            }
            let iter = self.vect[self.last..i].iter();
            self.last = i;
            Some(iter)
        }
    }
}

/// Iterate over sub-slices of the argument slice, where the elements
/// of each sub-slice are adjacent and equivalent under the predicate.
///
/// ```
/// use iterator_utilities::equivalence_class::equivalence_classes;
///
/// let ns = vec!{0usize,2,1,3,4,5};
/// let mut eq : Vec<_> = equivalence_classes(&ns, |l,r| l%2 == r%2 ).collect();
///
/// assert_eq!(eq.len(), 4);
///
/// let even: Vec<&usize> = eq[0].clone().collect();
/// assert_eq!(even, vec!{&0,&2});
///
/// let odd: Vec<&usize> = eq[1].clone().collect();
/// assert_eq!(odd, vec!{&1,&3});
///
/// let odd2: Vec<&usize> = eq[3].clone().collect();
/// assert_eq!(odd2, vec!{&5});
/// ```
pub fn equivalence_classes<'t,T,F>(slice: &'t Vec<T>, predicate: F) -> EqClIter<'t,T,F>
    where F: Fn(&T,&T)->bool {
        EqClIter {
            vect: slice,
            pred: predicate,
            last: 0,
        }
    }

#[test]
fn test1() {
    let ns = vec!{0,2,4,6,8,1,3,5,7,9};
    let mut eq = equivalence_classes(&ns, |l,r| l%2 == r%2 );

    if let Some(mut even) = eq.next() {
        for i in 0..5 {
            assert_eq!(even.next(), Some(&(i*2)));
        }
        assert_eq!(even.next(), None);
    } else { panic!("no even iterator"); }
    if let Some(mut odd) = eq.next() {
        for i in 0..5 {
            assert_eq!(odd.next(), Some(&(i*2 + 1)));
        }
        assert_eq!(odd.next(), None);
    } else { panic!("no even iterator"); }
    assert!(eq.next().is_none());
}
