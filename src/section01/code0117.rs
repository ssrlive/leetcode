#![allow(dead_code)]

// 117. Populating Next Right Pointers in Each Node II
// https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/
// https://leetcode.cn/problems/populating-next-right-pointers-in-each-node-ii/
//

use crate::treenode::TreeNode;

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn connect(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn get_next(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if root.is_none() {
                return None;
            }
            let mut root = root.clone();
            while root.is_some() && root.as_ref()?.borrow().left.is_none() && root.as_ref()?.borrow().right.is_none() {
                let r = root.as_ref()?.borrow().next.clone();
                root = r;
            }
            root.map(|v| {
                v.borrow()
                    .left
                    .clone()
                    .unwrap_or_else(|| v.borrow().right.clone().unwrap_or_else(|| unreachable!()))
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
            root.as_ref()?.borrow_mut().right.as_ref()?.borrow_mut().next = v;
        }
        Self::connect(root.as_ref()?.borrow().right.clone());
        Self::connect(root.as_ref()?.borrow().left.clone());
        root
    }
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let root = TreeNode::from_vec(&[
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
    let root = root.ok_or("")?;
    let root = root.borrow();
    assert_eq!(root.val, 1);
    assert_eq!(root.left.as_ref().ok_or("")?.borrow().val, 2);
    assert_eq!(root.right.as_ref().ok_or("")?.borrow().val, 3);
    assert_eq!(root.left.as_ref().ok_or("")?.borrow().left.as_ref().ok_or("")?.borrow().val, 4);
    assert_eq!(root.left.as_ref().ok_or("")?.borrow().right.as_ref().ok_or("")?.borrow().val, 5);
    assert_eq!(root.right.as_ref().ok_or("")?.borrow().left.as_ref(), None);
    assert_eq!(root.right.as_ref().ok_or("")?.borrow().right.as_ref().ok_or("")?.borrow().val, 7);
    assert_eq!(
        root.left
            .as_ref()
            .ok_or("")?
            .borrow()
            .left
            .as_ref()
            .ok_or("")?
            .borrow()
            .next
            .as_ref()
            .ok_or("")?
            .borrow()
            .val,
        5
    );
    assert_eq!(
        root.left
            .as_ref()
            .ok_or("")?
            .borrow()
            .right
            .as_ref()
            .ok_or("")?
            .borrow()
            .next
            .as_ref()
            .ok_or("")?
            .borrow()
            .val,
        7
    );
    assert_eq!(
        root.right.as_ref().ok_or("")?.borrow().right.as_ref().ok_or("")?.borrow().next,
        None
    );
    assert_eq!(
        root.left
            .as_ref()
            .ok_or("")?
            .borrow()
            .right
            .as_ref()
            .ok_or("")?
            .borrow()
            .right
            .as_ref()
            .ok_or("")?
            .borrow()
            .next
            .as_ref()
            .ok_or("")?
            .borrow()
            .val,
        13
    );
    Ok(())
}
