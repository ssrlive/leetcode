#![allow(dead_code)]

// 116. Populating Next Right Pointers in Each Node
// https://leetcode.com/problems/populating-next-right-pointers-in-each-node/

use super::treenode::TreeNode;

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn connect<T>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
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
    let root = TreeNode::from_vec(&vec![
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
