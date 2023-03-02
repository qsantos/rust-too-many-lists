type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct List<T> {
    root: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { root: None }
    }

    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.root.as_mut().map(|node| &mut node.value)
    }

    pub fn push_front(&mut self, value: T) {
        self.root = Some(Box::new(Node {
            value,
            next: self.root.take(),
        }));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.root.take().map(|node| {
            self.root = node.next;
            node.value
        })
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.root.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for List<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IterRef<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for IterRef<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(node) => {
                self.current = node.next.as_deref();
                Some(&node.value)
            }
        }
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> IterRef<'_, T> {
        IterRef {
            current: self.root.as_deref(),
        }
    }
}

pub struct IterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            None => None,
            Some(node) => {
                self.current = node.next.as_deref_mut();
                Some(&mut node.value)
            }
        }
    }
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            current: self.root.as_deref_mut(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        assert_eq!(list.pop_front(), None);
        list.push_front(10);
        list.push_front(5);
        assert_eq!(list.peek(), Some(&5));
        let head = list.peek_mut().unwrap();
        *head = 7;
        list.push_front(0);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), Some(7));
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);
        let values: Vec<_> = list.into_iter().collect();
        assert_eq!(values, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);
        let values: Vec<_> = list.iter().copied().collect();
        assert_eq!(values, vec![5, 4, 3, 2, 1]);
        let values: Vec<_> = list.into_iter().collect();
        assert_eq!(values, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);
        for v in list.iter_mut() {
            *v *= 2;
        }
        let values: Vec<_> = list.into_iter().collect();
        assert_eq!(values, vec![10, 8, 6, 4, 2]);
    }
}
