use std::ptr::null_mut;

type Link<T> = *mut Node<T>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Box<Node<T>> {
        Box::new(Node {
            value,
            next: null_mut(),
        })
    }
}

pub struct List<T> {
    first: Link<T>,
    last: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            first: null_mut(),
            last: null_mut(),
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::into_raw(Node::new(value));
        let last = self.last;
        self.last = new_node;
        if last.is_null() {
            self.first = new_node;
        } else {
            unsafe {
                (*last).next = new_node;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let first = self.first;
        if first.is_null() {
            None
        } else {
            let node = unsafe { Box::from_raw(first) };
            self.first = node.next;
            if self.first.is_null() {
                self.last = null_mut();
            }
            Some(node.value)
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
