use std::cell::{Ref, RefCell};
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
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}

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
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
    }
}
