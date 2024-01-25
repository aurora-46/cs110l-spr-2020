use std::clone;
use std::fmt::{self, Display};
use std::option::Option;

#[derive(PartialEq, Clone)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}


#[derive(PartialEq, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_Node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_Node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let Node: Box<Node<T>> = self.head.take()?;
        self.head = Node.next;
        self.size -= 1;
        Some(Node.value)
    }
}


impl<T:Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(Node) => {
                    result = format!("{} {}", result, Node.value);
                    current = &Node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut Node) = current {
            current = Node.next.take();
        }
    }
}
pub struct LinkedListIter<'a,T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a,T> Iterator for LinkedListIter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                // YOU FILL THIS IN!
                self.current=&node.next;
                Some(&node.value)
         },
            None => None//
        }
    }
}

impl<'a,T> IntoIterator for &'a LinkedList<T> {
    type Item =&'a T;
    type IntoIter = LinkedListIter<'a,T>;
    fn into_iter(self) -> LinkedListIter<'a,T> {
        LinkedListIter {current: &self.head}
    }
}
