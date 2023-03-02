type Anchor<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Anchor<T>,
}

pub struct List<T> {
    root: Anchor<T>,
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
}
