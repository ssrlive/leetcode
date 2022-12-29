#![allow(dead_code)]

// 872. Leaf-Similar Trees
// https://leetcode.com/problems/leaf-similar-trees/
// https://leetcode.cn/problems/leaf-similar-trees/
//
// Consider all the leaves of a binary tree, from left to right order, the values of those leaves form a leaf value sequence.
//
// For example, in the given tree above, the leaf value sequence is (6, 7, 4, 9, 8).
//
// Two binary trees are considered leaf-similar if their leaf value sequence is the same.
//
// Return true if and only if the two given trees with head nodes root1 and root2 are leaf-similar.
//
// Example 1:
//
// Input: root1 = [3,5,1,6,2,9,8,null,null,7,4], root2 = [3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]
// Output: true
//
// Example 2:
//
// Input: root1 = [1,2,3], root2 = [1,3,2]
// Output: false
//
// Constraints:
//
// - The number of nodes in each tree will be in the range [1, 200].
// - Both of the given trees will have values in the range [0, 200].
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    v.push(node.val);
                }
                dfs(node.left.clone(), v);
                dfs(node.right.clone(), v);
            }
        }

        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        dfs(root1, &mut v1);
        dfs(root2, &mut v2);
        v1 == v2
    }
}

#[test]
fn test() {
    let root1 = TreeNode::from_vec(&[
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(9),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);
    let root2 = TreeNode::from_vec(&[
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(7),
        Some(4),
        Some(2),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(9),
        Some(8),
    ]);
    assert_eq!(Solution::leaf_similar(root1, root2), true);

    let root1 = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
    let root2 = TreeNode::from_vec(&[Some(1), Some(3), Some(2)]);
    assert_eq!(Solution::leaf_similar(root1, root2), false);
}
