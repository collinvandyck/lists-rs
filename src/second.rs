use std::mem;

#[derive(Clone)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur: Option<Box<Node<T>>> = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|x| &mut x.elem)
    }

    pub fn push(&mut self, val: T) {
        self.head = Some(Box::new(Node {
            elem: val,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node.elem
        })
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter(&self.head)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(self.head.as_deref_mut())
    }
}

pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next<'b>(&'b mut self) -> Option<&'a mut T> {
        let head: Option<&mut Node<T>> = self.0.take();
        head.map(|head| {
            self.0 = head.next.as_deref_mut();
            &mut head.elem
        })
    }
}

pub struct Iter<'a, T>(&'a Link<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_ref().map(|head| {
            self.0 = &head.next;
            &head.elem
        })
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(3);
        list.push(2);
        list.push(1);
        for x in list.iter_mut() {
            *x *= 10;
        }
        let xs = list.into_iter().collect_vec();
        assert_eq!(xs, vec![10, 20, 30]);
    }

    #[test]
    fn test_basics() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        list.peek_mut().map(|x| *x *= 10);
        list.push(4);
        assert_eq!(list.clone().into_iter().collect_vec(), vec![4, 30, 1]);
        {
            let mut list = list.clone();
            assert_eq!(list.pop(), Some(4));
            assert_eq!(list.pop(), Some(30));
            assert_eq!(list.pop(), Some(1));
            assert_eq!(list.pop(), None);
        }
        let xs = list.iter().collect_vec();
        assert_eq!(xs, vec![&4, &30, &1]);
    }
}
