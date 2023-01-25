#![allow(dead_code)]

// 1305. All Elements in Two Binary Search Trees
// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/
// https://leetcode.cn/problems/all-elements-in-two-binary-search-trees/
//
// Medium
//
// Given two binary search trees root1 and root2, return a list containing all the integers from both trees sorted in ascending order.
//
// Example 1:
//
// Input: root1 = [2,1,4], root2 = [1,0,3]
// Output: [0,1,1,2,3,4]
//
// Example 2:
//
// Input: root1 = [1,null,8], root2 = [8,1]
// Output: [1,1,8,8]
//
// Constraints:
//
// -    The number of nodes in each tree is in the range [0, 5000].
// -    -10^5 <= Node.val <= 10^5
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
            if let Some(node) = root {
                inorder(&node.borrow().left, v);
                v.push(node.borrow().val);
                inorder(&node.borrow().right, v);
            }
        }

        let mut v1 = vec![];
        let mut v2 = vec![];
        inorder(&root1, &mut v1);
        inorder(&root2, &mut v2);
        let mut v = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < v1.len() && j < v2.len() {
            if v1[i] < v2[j] {
                v.push(v1[i]);
                i += 1;
            } else {
                v.push(v2[j]);
                j += 1;
            }
        }
        v.extend_from_slice(&v1[i..]);
        v.extend_from_slice(&v2[j..]);
        v
    }
}

#[test]
fn test() {
    let root1 = TreeNode::from_vec(&[Some(2), Some(1), Some(4)]);
    let root2 = TreeNode::from_vec(&[Some(1), Some(0), Some(3)]);
    let v = Solution::get_all_elements(root1, root2);
    assert_eq!(v, vec![0, 1, 1, 2, 3, 4]);

    let root1 = TreeNode::from_vec(&[Some(1), None, Some(8)]);
    let root2 = TreeNode::from_vec(&[Some(8), Some(1)]);
    let v = Solution::get_all_elements(root1, root2);
    assert_eq!(v, vec![1, 1, 8, 8]);
}
