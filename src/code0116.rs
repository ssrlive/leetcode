#![allow(dead_code)]

// 116. Populating Next Right Pointers in Each Node
// https://leetcode.com/problems/populating-next-right-pointers-in-each-node/

// Definition for a binary tree node.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub next: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            next: None,
        }
    }

    pub fn from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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
                node.as_ref()?.borrow_mut().right =
                    Some(Rc::new(RefCell::new(TreeNode::new(*val))));
                queue.push_back(node.as_ref()?.borrow().right.clone());
            }
            i += 1;
        }
        root
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();
                helper(&v.left, ret);
                ret.push(v.val);
                helper(&v.right, ret);
            }
        }

        let mut ret = vec![];
        if let Some(v) = root {
            helper(&Some(v), &mut ret);
        }
        ret
    }
}

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn connect(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = root.clone();
        while let Some(cur_rc) = cur.clone() {
            let l = cur_rc.borrow().left.clone();
            while let Some(cur_rc) = cur {
                if let Some(left) = cur_rc.borrow().left.clone() {
                    left.borrow_mut().next = cur_rc.borrow().right.clone();
                    if let Some(next) = cur_rc.borrow().next.clone() {
                        cur_rc.borrow().right.as_ref()?.borrow_mut().next =
                            next.borrow().left.clone();
                    }
                }
                cur = cur_rc.borrow().next.clone();
            }
            cur = l;
        }
        root
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
    ]);
    let root = Solution::connect(root);

    println!("{:#?}", root.unwrap().borrow());
}
