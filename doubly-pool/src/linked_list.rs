use std::cell::RefCell;
use std::rc::Rc;
use crate::enums::Side;
use crate::node::Node;
use crate::node_pool::NodePool;

#[derive(Debug)]
pub struct DoublyLinkedList<T: Clone> {
    pub(crate) head: Option<Rc<RefCell<Node<T>>>>,
    pub(crate) tail: Option<Rc<RefCell<Node<T>>>>,
    pool: NodePool<T>,
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            pool: NodePool::new(),
        }
    }

    pub fn init(&mut self, size: usize, default_value: T) {
        self.pool.populate(size, default_value);
    }

    pub fn insert(&mut self, side: Side, data: T, anchor: Rc<RefCell<Node<T>>>) {
        let new_node = self.pool.get_node(data);

        match side {
            Side::Before => {
                let mut anchor_borrow = anchor.borrow_mut();
                let prev_node = anchor_borrow.prev.clone();

                new_node.borrow_mut().next = Some(anchor.clone());
                new_node.borrow_mut().prev = prev_node.clone();

                if let Some(prev) = prev_node {
                    prev.borrow_mut().next = Some(new_node.clone());
                } else {
                    self.head = Some(new_node.clone());
                }

                anchor_borrow.prev = Some(new_node);
            }
            Side::After => {
                let mut anchor_borrow = anchor.borrow_mut();
                let next_node = anchor_borrow.next.clone();

                new_node.borrow_mut().prev = Some(anchor.clone());
                new_node.borrow_mut().next = next_node.clone();

                if let Some(next) = next_node {
                    next.borrow_mut().prev = Some(new_node.clone());
                } else {
                    self.tail = Some(new_node.clone());
                }

                anchor_borrow.next = Some(new_node);
            }
        }
    }

    pub fn push_first(&mut self, data: T) {
        if let Some(head) = self.head.clone() {
            self.insert(Side::Before, data, head);
        } else {
            let new_node = self.pool.get_node(data);
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }

    pub fn push_back(&mut self, data: T) {
        if let Some(tail) = self.tail.clone() {
            self.insert(Side::After, data, tail);
        } else {
            let new_node = self.pool.get_node(data);
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }

    pub fn remove(&mut self, node: Rc<RefCell<Node<T>>>) -> Option<T> {
        let node_borrow = node.borrow();
        let (_prev, next) = (node_borrow.prev.clone(), node_borrow.next.clone());

        if let Some(prev) = _prev.clone() {
            prev.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }
        if let Some(next) = next {
            next.borrow_mut().prev = _prev.clone();
        } else {
            self.tail = _prev.clone();
        }

        let data: T = node_borrow.data.clone();
        drop(node_borrow);
        self.pool.return_node(node);
        return Some(data);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        return self.tail.clone().map(|tail| self.remove(tail)).flatten();
    }

    pub fn pop_first(&mut self) -> Option<T> {
        return self.head.clone().map(|head| self.remove(head)).flatten();
    }
}

impl<T: Clone> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.tail.is_some() {
            self.pop_back();
        }
    }
}
