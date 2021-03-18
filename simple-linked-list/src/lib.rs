use std::iter::FromIterator;
use std::mem;
use std::rc::Rc;

pub struct SimpleLinkedList<T> {
    head: Option<Rc<Node<T>>>,
}
pub struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut counter: usize = 0;
        let mut current_node = self.head.as_ref();

        loop {
            match current_node {
                None => break counter,
                Some(node) => current_node = node.next.as_ref(),
            }
            counter += 1
        }
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Node {
            data: _element,
            next: self.head.clone(),
        };
        self.head = Some(Rc::new(new_node))
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,

            Some(rc_node) => {
                self.head = rc_node.next.clone();
                Some(rc_node.data.clone())
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        let mut old = self;
        loop {
            match old.pop() {
                None => break reversed,
                Some(item) => reversed.push(item),
            }
        }
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut simplelist = SimpleLinkedList::new();
        let mut iterator = _iter.into_iter();
        loop {
            match iterator.next() {
                None => break simplelist,
                Some(dat) => simplelist.push(dat),
            }
        }
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut old = self.rev();
        let mut vec: Vec<T> = Vec::new();
        loop {
            match old.pop() {
                None => return vec,
                Some(elem) => vec.push(elem),
            }
        }
    }
}
