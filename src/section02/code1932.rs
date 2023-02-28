#![allow(dead_code)]

/*

// 1932. Merge BSTs to Create Single BST
// https://leetcode.com/problems/merge-bsts-to-create-single-bst/
// https://leetcode.cn/problems/merge-bsts-to-create-single-bst/
//
// Hard
//
// You are given n BST (binary search tree) root nodes for n separate BSTs stored in an array trees (0-indexed). Each BST in trees has at most 3 nodes, and no two roots have the same value. In one operation, you can:

    Select two distinct indices i and j such that the value stored at one of the leaves of trees[i] is equal to the root value of trees[j].
    Replace the leaf node in trees[i] with trees[j].
    Remove trees[j] from trees.

Return the root of the resulting BST if it is possible to form a valid BST after performing n - 1 operations, or null if it is impossible to create a valid BST.

A BST (binary search tree) is a binary tree where each node satisfies the following property:

    Every node in the node's left subtree has a value strictly less than the node's value.
    Every node in the node's right subtree has a value strictly greater than the node's value.

A leaf is a node that has no children.

Example 1:

Input: trees = [[2,1],[3,2,5],[5,4]]
Output: [3,2,5,1,null,4]
Explanation:
In the first operation, pick i=1 and j=0, and merge trees[0] into trees[1].
Delete trees[0], so trees = [[3,2,5,1],[5,4]].

In the second operation, pick i=0 and j=1, and merge trees[1] into trees[0].
Delete trees[1], so trees = [[3,2,5,1,null,4]].

The resulting tree, shown above, is a valid BST, so return its root.

Example 2:

Input: trees = [[5,3,8],[3,2,6]]
Output: []
Explanation:
Pick i=0 and j=1 and merge trees[1] into trees[0].
Delete trees[1], so trees = [[5,3,8,2,6]].

The resulting tree is shown above. This is the only valid operation that can be performed, but the resulting tree is not a valid BST, so return null.

Example 3:

Input: trees = [[5,4],[3]]
Output: []
Explanation: It is impossible to perform any operations.

Constraints:

    n == trees.length
    1 <= n <= 5 * 10^4
    The number of nodes in each tree is in the range [1, 3].
    Each node in the input may have children but no grandchildren.
    No two roots of trees have the same value.
    All the trees in the input are valid BSTs.
    1 <= TreeNode.val <= 5 * 10^4.
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::{HashMap, HashSet};
        fn add_leaf(
            r: Option<Rc<RefCell<TreeNode>>>,
            leaves: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
            roots: &mut HashMap<i32, Option<Rc<RefCell<TreeNode>>>>,
            unique_vals: &mut HashSet<i32>,
        ) {
            if r.is_some() {
                let val = r.as_ref().unwrap().borrow().val;
                unique_vals.insert(val);
                if roots.contains_key(&val) {
                    leaves.push(r);
                }
            }
        }

        fn valid_nodes(r: Option<Rc<RefCell<TreeNode>>>, min_left: i32, max_right: i32) -> usize {
            let val = r.as_ref().map(|r| r.borrow().val).unwrap_or(0);
            if r.is_none() || val <= min_left || val >= max_right {
                return 0;
            }
            let left = r.as_ref().unwrap().borrow().left.clone();
            let right = r.as_ref().unwrap().borrow().right.clone();
            let v_left = valid_nodes(left, min_left, val);
            let v_right = valid_nodes(right, val, max_right);
            1 + v_left + v_right
        }

        let mut unique_vals = HashSet::new();
        let mut roots = HashMap::new();
        let mut leaves = Vec::new();
        for t in trees.iter() {
            let val = t.as_ref()?.borrow().val;
            roots.insert(val, t.clone());
        }
        for t in trees.iter() {
            let val = t.as_ref()?.borrow().val;
            let left = t.as_ref()?.borrow().left.clone();
            let right = t.as_ref()?.borrow().right.clone();
            unique_vals.insert(val);
            add_leaf(left, &mut leaves, &mut roots, &mut unique_vals);
            add_leaf(right, &mut leaves, &mut roots, &mut unique_vals);
        }
        for leaf in leaves {
            let val = leaf.as_ref()?.borrow().val;
            let root = roots.get_mut(&val)?;
            leaf.as_ref()?.borrow_mut().left = root.as_ref()?.borrow().left.clone();
            leaf.as_ref()?.borrow_mut().right = root.as_ref()?.borrow().right.clone();
            roots.remove(&val);
        }
        let first_node = roots.values().next().cloned()?;
        if roots.len() == 1 && valid_nodes(first_node.clone(), std::i32::MIN, std::i32::MAX) == unique_vals.len() {
            first_node
        } else {
            None
        }
    }
}

#[test]
fn test() {
    let trees = vec![
        TreeNode::from_string("[2,1,3]"),
        TreeNode::from_string("[1]"),
        TreeNode::from_string("[3]"),
    ];
    let result = TreeNode::from_string("[2, 1, 3]");
    assert_eq!(Solution::can_merge(trees), result);

    let trees = vec![
        TreeNode::from_string("[2,1]"),
        TreeNode::from_string("[3,2,5]"),
        TreeNode::from_string("[5,4]"),
    ];
    let result = TreeNode::from_string("[3,2,5,1,null,4]");
    assert_eq!(Solution::can_merge(trees), result);

    #[rustfmt::skip]
    let trees = vec![
        TreeNode::from_string("[5,3,8]"),
        TreeNode::from_string("[3,2,6]"),
    ];
    let result = TreeNode::from_string("[]");
    assert_eq!(Solution::can_merge(trees), result);

    #[rustfmt::skip]
    let trees = vec![
        TreeNode::from_string("[5,4]"),
        TreeNode::from_string("[3]"),
    ];
    let result = TreeNode::from_string("[]");
    assert_eq!(Solution::can_merge(trees), result);
}
