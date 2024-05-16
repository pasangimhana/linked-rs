use std::cell::RefCell;
use std::rc::Rc;
use crate::node::Node;

#[derive(Debug)]
pub struct NodePool<T> {
    pool: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> NodePool<T> {
    pub fn new() -> Self {
        NodePool { pool: Vec::new() }
    }

    pub fn populate(&mut self, size: usize, default_value: T) {
        for _ in 0..size {
            self.pool
                .push(Rc::new(RefCell::new(Node::new(default_value.clone()))));
        }
    }

    pub fn get_node(&mut self, data: T) -> Rc<RefCell<Node<T>>> {
        if let Some(node) = self.pool.pop() {
            node.borrow_mut().data = data;
            node.borrow_mut().next = None;
            node.borrow_mut().prev = None;
            node
        } else {
            Rc::new(RefCell::new(Node::new(data)))
        }
    }

    pub fn return_node(&mut self, node: Rc<RefCell<Node<T>>>) {
        node.borrow_mut().next = None;
        node.borrow_mut().prev = None;
        self.pool.push(node);
    }
}

