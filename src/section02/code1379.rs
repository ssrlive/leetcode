#![allow(dead_code)]

// 1379. Find a Corresponding Node of a Binary Tree in a Clone of That Tree
// https://leetcode.com/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/description/
// https://leetcode.cn/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/description/
//
// Easy
//
// Given two binary trees original and cloned and given a reference to a node target in the original tree.
//
// The cloned tree is a copy of the original tree.
//
// Return a reference to the same node in the cloned tree.
//
// Note that you are not allowed to change any of the two trees or the target node and the answer must be a reference to a node in the cloned tree.
//
// Example 1:
//
// Input: tree = [7,4,3,null,null,6,19], target = 3
// Output: 3
// Explanation: In all examples the original and cloned trees are shown. The target node is a green node from the original tree.
// The answer is the yellow node from the cloned tree.
//
// Example 2:
//
// Input: tree = [7], target =  7
// Output: 7
//
// Example 3:
//
// Input: tree = [8,null,6,null,5,null,4,null,3,null,2,null,1], target = 4
// Output: 4
//
// Constraints:
//
// -    The number of nodes in the tree is in the range [1, 104].
// -    The values of the nodes of the tree are unique.
// -    target node is a node from the original tree and is not null.
//
// Follow up: Could you solve the problem if repeated values on the tree are allowed?
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_target_copy(
        _original: Option<Rc<RefCell<TreeNode>>>,
        cloned: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            let node_ref = node.as_ref()?.borrow();
            if node_ref.val == val {
                return node.clone();
            }
            let l = dfs(&node_ref.left, val);
            if l.is_some() {
                return l;
            }
            dfs(&node_ref.right, val)
        }
        let val = target.as_ref()?.borrow().val;
        dfs(&cloned, val)
    }
}

#[test]
fn test() {
    let tree = TreeNode::from_string("[7,4,3,null,null,6,19]");
    let cloned = TreeNode::from_string("[7,4,3,null,null,6,19]");
    let target = TreeNode::from_string("[3, 6, 19]");
    let result = Solution::get_target_copy(tree, cloned, target);
    assert_eq!(result.unwrap().borrow().val, 3);
}
