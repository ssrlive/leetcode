#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Leaf(bool),
    Node {
        val: bool,
        top_left: Option<Rc<RefCell<Node>>>,
        top_right: Option<Rc<RefCell<Node>>>,
        bottom_left: Option<Rc<RefCell<Node>>>,
        bottom_right: Option<Rc<RefCell<Node>>>,
    },
}

impl Node {
    pub fn new(
        is_leaf: bool,
        val: bool,
        top_left: Option<Rc<RefCell<Node>>>,
        top_right: Option<Rc<RefCell<Node>>>,
        bottom_left: Option<Rc<RefCell<Node>>>,
        bottom_right: Option<Rc<RefCell<Node>>>,
    ) -> Self {
        if is_leaf {
            assert!(top_left.is_none());
            assert!(top_right.is_none());
            assert!(bottom_left.is_none());
            assert!(bottom_right.is_none());
            Node::Leaf(val)
        } else {
            Node::Node {
                val,
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            }
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf(_) => true,
            Node::Node { .. } => false,
        }
    }

    pub fn val(&self) -> bool {
        match self {
            Node::Leaf(val) => *val,
            Node::Node { val, .. } => *val,
        }
    }

    pub fn top_left(&self) -> Option<Rc<RefCell<Node>>> {
        match self {
            Node::Leaf(_) => None,
            Node::Node {
                val: _,
                top_left,
                top_right: _,
                bottom_left: _,
                bottom_right: _,
            } => top_left.clone(),
        }
    }

    pub fn top_right(&self) -> Option<Rc<RefCell<Node>>> {
        match self {
            Node::Leaf(_) => None,
            Node::Node {
                val: _,
                top_left: _,
                top_right,
                bottom_left: _,
                bottom_right: _,
            } => top_right.clone(),
        }
    }

    pub fn bottom_left(&self) -> Option<Rc<RefCell<Node>>> {
        match self {
            Node::Leaf(_) => None,
            Node::Node {
                val: _,
                top_left: _,
                top_right: _,
                bottom_left,
                bottom_right: _,
            } => bottom_left.clone(),
        }
    }

    pub fn bottom_right(&self) -> Option<Rc<RefCell<Node>>> {
        match self {
            Node::Leaf(_) => None,
            Node::Node {
                val: _,
                top_left: _,
                top_right: _,
                bottom_left: _,
                bottom_right,
            } => bottom_right.clone(),
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Leaf(val) => write!(f, "Leaf(val = {val})"),
            Node::Node {
                val,
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            } => write!(
                f,
                "Node(val = {}, {}, {}, {}, {})",
                val,
                top_left
                    .as_ref()
                    .map(|node| node.borrow().to_string())
                    .unwrap_or_else(|| "null".to_string()),
                top_right
                    .as_ref()
                    .map(|node| node.borrow().to_string())
                    .unwrap_or_else(|| "null".to_string()),
                bottom_left
                    .as_ref()
                    .map(|node| node.borrow().to_string())
                    .unwrap_or_else(|| "null".to_string()),
                bottom_right
                    .as_ref()
                    .map(|node| node.borrow().to_string())
                    .unwrap_or_else(|| "null".to_string()),
            ),
        }
    }
}
