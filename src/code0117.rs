#![allow(dead_code)]

// 117. Populating Next Right Pointers in Each Node II
// https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/

// Definition for a binary tree node.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub val: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
            next: None,
        }
    }

    pub fn from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<Node>>> {
        if v.is_empty() {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(Node::new(v[0]?))));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < v.len() {
            let node = queue.pop_front()?;
            if let Some(val) = v.get(i)? {
                node.as_ref()?.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(*val))));
                queue.push_back(node.as_ref()?.borrow().left.clone());
            }
            i += 1;
            if i >= v.len() {
                break;
            }
            if let Some(val) = v.get(i)? {
                node.as_ref()?.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(*val))));
                queue.push_back(node.as_ref()?.borrow().right.clone());
            }
            i += 1;
        }
        root
    }

    pub fn preorder_traversal(root: Option<Rc<RefCell<Self>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<Node>>>, ret: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();
                ret.push(v.val);
                helper(&v.left, ret);
                helper(&v.right, ret);
            }
        }
        let mut ret = Vec::new();
        helper(&root, &mut ret);
        ret
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<Self>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<Node>>>, ret: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();
                helper(&v.left, ret);
                ret.push(v.val);
                helper(&v.right, ret);
            }
        }
        let mut ret = Vec::new();
        helper(&root, &mut ret);
        ret
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<Self>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<Node>>>, ret: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();
                helper(&v.left, ret);
                helper(&v.right, ret);
                ret.push(v.val);
            }
        }
        let mut ret = Vec::new();
        helper(&root, &mut ret);
        ret
    }
}

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn connect(root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        fn get_next(root: &Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
            if root.is_none() {
                return None;
            }
            let mut root = root.clone();
            while root.is_some()
                && root.as_ref()?.borrow().left.is_none()
                && root.as_ref()?.borrow().right.is_none()
            {
                let r = root.as_ref()?.borrow().next.clone();
                root = r;
            }
            root.map(|v| {
                v.borrow()
                    .left
                    .clone()
                    .unwrap_or_else(|| v.borrow().right.clone().unwrap())
            })
        }

        root.as_ref()?;
        if root.as_ref()?.borrow().left.is_some() && root.as_ref()?.borrow().right.is_some() {
            let v = root.as_ref()?.borrow().right.clone();
            root.as_ref()?.borrow_mut().left.as_ref()?.borrow_mut().next = v;
        } else if root.as_ref()?.borrow().left.is_some() {
            let v = get_next(&root.as_ref()?.borrow().next);
            root.as_ref()?.borrow_mut().left.as_ref()?.borrow_mut().next = v;
        }
        if root.as_ref()?.borrow().right.is_some() {
            let v = get_next(&root.as_ref()?.borrow().next);
            root.as_ref()?
                .borrow_mut()
                .right
                .as_ref()?
                .borrow_mut()
                .next = v;
        }
        Self::connect(root.as_ref()?.borrow().right.clone());
        Self::connect(root.as_ref()?.borrow().left.clone());
        root
    }
}

#[test]
fn test() {
    let root = Node::from_vec(vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        Some(7),
        None,
        None,
        None,
        Some(11),
        None,
        Some(13),
    ]);
    let root = Solution::connect(root);
    let root = root.unwrap();
    let root = root.borrow();
    assert_eq!(root.val, 1);
    assert_eq!(root.left.as_ref().unwrap().borrow().val, 2);
    assert_eq!(root.right.as_ref().unwrap().borrow().val, 3);
    assert_eq!(
        root.left
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        4
    );
    assert_eq!(
        root.left
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        5
    );
    assert_eq!(root.right.as_ref().unwrap().borrow().left.as_ref(), None);
    assert_eq!(
        root.right
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        7
    );
    assert_eq!(
        root.left
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        5
    );
    assert_eq!(
        root.left
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        7
    );
    assert_eq!(
        root.right
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .next,
        None
    );
    assert_eq!(
        root.left
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        13
    );
}
