//! The delta list was originally introduced by [Operating System Design, The Xinu Approach](https://xinu.cs.purdue.edu/)'s
//! Chapter 13.
//!
//! We specify the delta list by using TLA+.
//! See [the specification](https://github.com/tier4/safe_drive/tree/main/specifications/callback).

use std::{cell::UnsafeCell, time::Duration};

/// Timers are represented by a linked list.
/// Each element represents the difference of time from parent node.
///
/// For example, if `timer = 10 -> 20 -> 5`,
/// timers will be invoked after 10, 10 + 20 = 30, and 10 + 20 + 5 = 35 seconds later, respectively.
///
/// At that time, if `t` is 13, the callback will be invoked 13 seconds later.
/// To accomplish this, 13 should be inserted between 10 and 20 like
/// `10 -> 3 (inserted) -> 17 (updated) -> 5`.
pub enum DeltaList<T> {
    Cons(Box<UnsafeCell<(Duration, T, DeltaList<T>)>>),
    Nil,
}

impl<T> DeltaList<T> {
    pub fn insert(&mut self, delta: Duration, data: T) {
        insert_delta(self, delta, data);
    }

    pub fn front(&self) -> Option<(&Duration, &T)> {
        if let DeltaList::Cons(e) = self {
            let elm = unsafe { &(*e.get()) };
            Some((&elm.0, &elm.1))
        } else {
            None
        }
    }

    pub fn front_mut(&mut self) -> Option<(&mut Duration, &mut T)> {
        if let DeltaList::Cons(e) = self {
            let elm = e.get_mut();
            Some((&mut elm.0, &mut elm.1))
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<Self> {
        if let DeltaList::Cons(e) = self {
            let next = std::mem::replace(&mut e.get_mut().2, DeltaList::Nil);
            let head = std::mem::replace(self, next);
            Some(head)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, DeltaList::Nil)
    }
}

fn insert_delta<T>(mut list: &mut DeltaList<T>, delta: Duration, data: T) {
    loop {
        match list {
            DeltaList::Nil => {
                *list = DeltaList::Cons(Box::new(UnsafeCell::new((delta, data, DeltaList::Nil))));
                return;
            }
            DeltaList::Cons(e) => {
                let front = e.get();
                let d_mut = unsafe { &mut (*front).0 };
                if delta < *d_mut {
                    *d_mut -= delta;
                    let next = std::mem::replace(list, DeltaList::Nil);
                    *list = DeltaList::Cons(Box::new(UnsafeCell::new((delta, data, next))));
                    return;
                } else {
                    list = unsafe { &mut (*front).2 };
                }
            }
        }
    }
}
