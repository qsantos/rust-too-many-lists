use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

pub struct List<T> {
    first: Link<T>,
    last: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.first.take() {
            None => {
                self.first = Some(new_node.clone());
                self.last = Some(new_node);
            }
            Some(next) => {
                next.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(next);
                self.first = Some(new_node);
            }
        }
        assert_eq!(Rc::strong_count(self.first.as_ref().unwrap()), 2);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.first.take() {
            None => None,
            Some(node) => {
                assert!(node.borrow().prev.is_none());
                // detach node
                match node.borrow_mut().next.take() {
                    None => {
                        self.last = None;
                    }
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.first = Some(next);
                    }
                }
                // unwrap the value
                Some(Rc::try_unwrap(node).ok().unwrap().into_inner().value)
            }
        }
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.first
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.first
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.value))
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.last.take() {
            None => {
                self.last = Some(new_node.clone());
                self.first = Some(new_node);
            }
            Some(prev) => {
                prev.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(prev);
                self.last = Some(new_node);
            }
        }
        assert_eq!(Rc::strong_count(self.first.as_ref().unwrap()), 2);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.last.take() {
            None => None,
            Some(node) => {
                assert!(node.borrow().next.is_none());
                // detach node
                match node.borrow_mut().prev.take() {
                    None => {
                        self.first = None;
                    }
                    Some(prev) => {
                        prev.borrow_mut().next = None;
                        self.last = Some(prev);
                    }
                }
                // unwrap the value
                Some(Rc::try_unwrap(node).ok().unwrap().into_inner().value)
            }
        }
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.last
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.last
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.value))
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for List<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

/*
pub struct Iter<'a, T> {
    current: Option<Ref<'a, Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node_ref| {
            let (value, next) = Ref::map_split(node_ref, |node| (&node.value, &node.next));
            self.current = next.as_ref().map(|next_ref| next_ref.borrow());
            value
        })
    }
}
*/

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
