#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub next: Option<Rc<RefCell<TreeNode>>>,
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self._to_string())
    }
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            next: None,
        }
    }

    pub fn from_vec(v: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if v.is_empty() {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(v[0]?))));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < v.len() {
            let node = queue.pop_front()?;
            if let Some(val) = v.get(i)? {
                node.as_ref()?.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(*val))));
                queue.push_back(node.as_ref()?.borrow().left.clone());
            }
            i += 1;
            if i >= v.len() {
                break;
            }
            if let Some(val) = v.get(i)? {
                node.as_ref()?.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(*val))));
                queue.push_back(node.as_ref()?.borrow().right.clone());
            }
            i += 1;
        }
        root
    }

    pub fn to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut v = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        while let Some(node) = queue.pop_front() {
            if node.is_some() {
                let node = node.as_ref().unwrap().borrow();
                v.push(Some(node.val));
                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());
            } else {
                v.push(None);
            }
        }
        while let Some(None) = v.last() {
            v.pop();
        }
        v
    }

    fn _to_string(&self) -> String {
        let node = Some(Rc::new(RefCell::new(self.clone())));
        let v = TreeNode::to_vec(&node);
        let mut s = String::new();
        for (i, item) in v.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            if item.is_none() {
                s.push_str("null");
            } else {
                s.push_str(&item.unwrap_or_default().to_string());
            }
        }
        s
    }

    pub fn preorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();
                ret.push(v.val);
                helper(&v.left, ret);
                helper(&v.right, ret);
            }
        }

        let mut ret = vec![];
        helper(root, &mut ret);
        ret
    }

    pub fn inorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();
                helper(&v.left, ret);
                ret.push(v.val);
                helper(&v.right, ret);
            }
        }

        let mut ret = vec![];
        helper(root, &mut ret);
        ret
    }

    pub fn postorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();
                helper(&v.left, ret);
                helper(&v.right, ret);
                ret.push(v.val);
            }
        }

        let mut ret = vec![];
        helper(root, &mut ret);
        ret
    }

    pub fn find_node(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if node.is_some() {
                if node.as_ref()?.borrow().val == val {
                    return node.clone();
                }
                if let node @ Some(_) = helper(&node.as_ref()?.borrow().left, val) {
                    return node;
                }
                if let node @ Some(_) = helper(&node.as_ref()?.borrow().right, val) {
                    return node;
                }
            }
            None
        }

        helper(root, val)
    }
}

#[test]
fn test_tree_node() -> Result<(), Box<dyn std::error::Error>> {
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    let root2 = root.as_ref().ok_or("")?.borrow();
    let expected = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
    assert_eq!(TreeNode::to_vec(&root), expected);
    assert_eq!(root2.to_string(), "1, 2, 3, 4, 5, 6");
    assert_eq!(TreeNode::preorder_traversal(&root), vec![1, 2, 4, 5, 3, 6]);
    assert_eq!(TreeNode::inorder_traversal(&root), vec![4, 2, 5, 1, 6, 3]);
    assert_eq!(TreeNode::postorder_traversal(&root), vec![4, 5, 2, 6, 3, 1]);
    Ok(())
}
