use std::{fmt::Debug, mem};

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug)]
enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur: Link<T> = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur {
            cur = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn iter<'a>(&'a self) -> ListIter<'a, T> {
        ListIter { link: &self.head }
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = mem::replace(&mut self.head, Link::Empty);
        match head {
            Link::Empty => None,
            Link::More(mut node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn push(&mut self, val: T) {
        self.head = Link::More(Box::new(Node {
            elem: val,
            next: mem::replace(&mut self.head, Link::Empty),
        }));
    }
}

pub struct ListIter<'a, T> {
    link: &'a Link<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.link {
            Link::Empty => None,
            Link::More(node) => {
                self.link = &node.next;
                Some(&node.elem)
            }
        }
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let head = mem::replace(&mut self.head, Link::Empty);
        match head {
            Link::Empty => None,
            Link::More(mut node) => {
                self.head = mem::replace(&mut node.next, Link::Empty);
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        for x in [1, 2, 3] {
            list.push(x);
        }
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        for x in [4, 5] {
            list.push(x);
        }
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
    }
}
