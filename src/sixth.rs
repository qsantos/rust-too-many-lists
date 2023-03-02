use std::marker::PhantomData;
use std::ptr::null_mut;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

pub struct List<T> {
    first: *mut Node<T>,
    last: *mut Node<T>,
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            first: null_mut(),
            last: null_mut(),
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
        let first = self.first;
        let node = Box::into_raw(Box::new(Node {
            value,
            next: first,
            prev: null_mut(),
        }));
        if first.is_null() {
            self.last = node;
        } else {
            unsafe {
                (*first).prev = node;
            }
        }
        self.first = node;
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let last = self.last;
        let node = Box::into_raw(Box::new(Node {
            value,
            next: null_mut(),
            prev: last,
        }));
        if last.is_null() {
            self.first = node;
        } else {
            unsafe {
                (*last).next = node;
            }
        }
        self.last = node;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.first.is_null() {
            None
        } else {
            let node = unsafe { Box::from_raw(self.first) };
            self.first = node.next;
            if self.first.is_null() {
                self.last = null_mut();
            } else {
                unsafe {
                    (*self.first).prev = null_mut();
                }
            }
            self.len -= 1;
            Some(node.value)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.last.is_null() {
            None
        } else {
            let node = unsafe { Box::from_raw(self.last) };
            self.last = node.prev;
            if self.last.is_null() {
                self.first = null_mut();
            } else {
                unsafe {
                    (*self.last).next = null_mut();
                }
            }
            self.len -= 1;
            Some(node.value)
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        unsafe { self.first.as_ref() }.map(|node| &node.value)
    }

    pub fn peek_back(&self) -> Option<&T> {
        unsafe { self.last.as_ref() }.map(|node| &node.value)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.first.as_mut() }.map(|node| &mut node.value)
    }
    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.last.as_mut() }.map(|node| &mut node.value)
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
