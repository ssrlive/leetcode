#![allow(dead_code)]

// 701. Insert into a Binary Search Tree
// https://leetcode.com/problems/insert-into-a-binary-search-tree/
// https://leetcode.cn/problems/insert-into-a-binary-search-tree/
//
// ou are given the root node of a binary search tree (BST) and a value to insert into the tree.
// Return the root node of the BST after the insertion. It is guaranteed that the new value does not exist in the original BST.
//
// Notice that there may exist multiple valid ways for the insertion, as long as the tree remains a BST after insertion.
// You can return any of them.
//
// Example 1:
//
// Input: root = [4,2,7,1,3], val = 5
// Output: [4,2,7,1,3,5]
// Explanation: Another accepted tree is:
//
// Example 2:
//
// Input: root = [40,20,60,10,30,50,70], val = 25
// Output: [40,20,60,10,30,50,70,null,null,25]
//
// Example 3:
//
// Input: root = [4,2,7,1,3,null,null,null,null,null,null], val = 5
// Output: [4,2,7,1,3,5]
//
// Constraints:
//
// - The number of nodes in the tree will be in the range [0, 10^4].
// - -10^8 <= Node.val <= 10^8
// - All the values Node.val are unique.
// - -10^8 <= val <= 10^8
// - It's guaranteed that val does not exist in the original BST.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        pub fn _insert_into_bst(node: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            node.as_ref()?;
            let node_val = node.as_ref()?.borrow().val;
            match node_val.cmp(&val) {
                std::cmp::Ordering::Equal => node,
                std::cmp::Ordering::Less => {
                    if node.as_ref()?.borrow().right.is_none() {
                        node.as_ref()?.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    } else {
                        _insert_into_bst(node.as_ref()?.borrow().right.clone(), val);
                    }
                    node
                }
                std::cmp::Ordering::Greater => {
                    if node.as_ref()?.borrow().left.is_none() {
                        node.as_ref()?.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    } else {
                        _insert_into_bst(node.as_ref()?.borrow().left.clone(), val);
                    }
                    node
                }
            }
        }

        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        _insert_into_bst(root, val)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(4), Some(2), Some(7), Some(1), Some(3)]);
    let val = 5;
    let expected = TreeNode::from_vec(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)]);
    assert_eq!(Solution::insert_into_bst(root, val), expected);

    let root = TreeNode::from_vec(&[Some(40), Some(20), Some(60), Some(10), Some(30), Some(50), Some(70)]);
    let val = 25;
    let expected = TreeNode::from_vec(&[
        Some(40),
        Some(20),
        Some(60),
        Some(10),
        Some(30),
        Some(50),
        Some(70),
        None,
        None,
        Some(25),
    ]);
    assert_eq!(Solution::insert_into_bst(root, val), expected);

    let root = TreeNode::from_vec(&[Some(4), Some(2), Some(7), Some(1), Some(3), None, None]);
    let val = 5;
    let expected = TreeNode::from_vec(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)]);
    assert_eq!(Solution::insert_into_bst(root, val), expected);
}
