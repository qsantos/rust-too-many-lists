use std::marker::PhantomData;
use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct List<T> {
    first: Link<T>,
    last: Link<T>,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            first: None,
            last: None,
            len: 0,
            _phantom: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, value: T) {
        unsafe {
            let node = Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value,
                next: self.first,
                prev: None,
            }))));
            match self.first.as_mut() {
                None => self.last = node,
                Some(first) => first.as_mut().prev = node,
            }
            self.first = node;
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, value: T) {
        unsafe {
            let node = Some(NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                value,
                next: None,
                prev: self.last,
            }))));
            match self.last.as_mut() {
                None => self.first = node,
                Some(last) => last.as_mut().next = node,
            }
            self.last = node;
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.first.map(|node| {
                let node = Box::from_raw(node.as_ptr());
                self.first = node.next;
                match self.first.as_mut() {
                    None => self.last = None,
                    Some(first) => first.as_mut().prev = None,
                }
                self.len -= 1;
                node.value
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.last.map(|node| {
                let node = Box::from_raw(node.as_ptr());
                self.last = node.prev;
                match self.last.as_mut() {
                    None => self.first = None,
                    Some(last) => last.as_mut().next = None,
                }
                self.len -= 1;
                node.value
            })
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        unsafe { self.first.map(|node| &node.as_ref().value) }
    }

    pub fn peek_back(&self) -> Option<&T> {
        unsafe { self.last.map(|node| &node.as_ref().value) }
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.first.as_mut().map(|node| &mut node.as_mut().value) }
    }
    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.last.as_mut().map(|node| &mut node.as_mut().value) }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct Iter<T>(List<T>);

impl<T> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> IntoIterator for List<T> {
    type IntoIter = Iter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
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

        assert_eq!(list.peek_front(), Some(&3));
        assert_eq!(list.peek_front_mut(), Some(&mut 3));
        assert_eq!(list.peek_back(), Some(&1));
        assert_eq!(list.peek_back_mut(), Some(&mut 1));
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
