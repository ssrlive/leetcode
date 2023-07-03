#![allow(dead_code)]

/*
// 2641. Cousins in Binary Tree II
// https://leetcode.com/problems/cousins-in-binary-tree-ii/
// https://leetcode.cn/problems/cousins-in-binary-tree-ii/
//
// Medium
//
// Given the root of a binary tree, replace the value of each node in the tree with the sum of all its cousins' values.

Two nodes of a binary tree are cousins if they have the same depth with different parents.

Return the root of the modified tree.

Note that the depth of a node is the number of edges in the path from the root node to it.

Example 1:

Input: root = [5,4,9,1,10,null,7]
Output: [0,0,0,7,7,null,11]
Explanation: The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
- Node with value 5 does not have any cousins so its sum is 0.
- Node with value 4 does not have any cousins so its sum is 0.
- Node with value 9 does not have any cousins so its sum is 0.
- Node with value 1 has a cousin with value 7 so its sum is 7.
- Node with value 10 has a cousin with value 7 so its sum is 7.
- Node with value 7 has cousins with values 1 and 10 so its sum is 11.
Example 2:

Input: root = [3,1,2]
Output: [0,0,0]
Explanation: The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
- Node with value 3 does not have any cousins so its sum is 0.
- Node with value 1 does not have any cousins so its sum is 0.
- Node with value 2 does not have any cousins so its sum is 0.

Constraints:

The number of nodes in the tree is in the range [1, 10^5].
1 <= Node.val <= 10^4
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        let mut l_sum = vec![];
        Solution::level_sum(&mut l_sum, &root, 0);
        let sib_sum = root.as_ref()?.borrow().val;
        Solution::update_sum(&l_sum, &mut root, sib_sum, 0);
        root
    }

    fn level_sum(l_sum: &mut Vec<i32>, n: &Option<Rc<RefCell<TreeNode>>>, d: usize) -> Option<()> {
        if n.is_none() {
            return Some(());
        }
        if l_sum.len() < d + 1 {
            l_sum.resize(d + 1, 0);
        }
        let n = n.as_ref()?.borrow();
        l_sum[d] += n.val;
        Solution::level_sum(l_sum, &n.left, d + 1);
        Solution::level_sum(l_sum, &n.right, d + 1);
        Some(())
    }

    fn update_sum(l_sum: &Vec<i32>, n: &mut Option<Rc<RefCell<TreeNode>>>, sib_sum: i32, d: usize) -> Option<()> {
        if n.is_none() {
            return Some(());
        }
        let mut n = n.as_mut()?.borrow_mut();
        n.val = l_sum[d] - sib_sum;
        let sib_sum = n.left.as_ref().map(|n| n.borrow().val).unwrap_or(0) + n.right.as_ref().map(|n| n.borrow().val).unwrap_or(0);
        Solution::update_sum(l_sum, &mut n.left, sib_sum, d + 1);
        Solution::update_sum(l_sum, &mut n.right, sib_sum, d + 1);
        Some(())
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[5, 4, 9, 1, 10, null, 7]");
    let root = Solution::replace_value_in_tree(root);
    assert_eq!(root.as_ref().unwrap().borrow().to_string(), "[0, 0, 0, 7, 7, null, 11]");

    let root = TreeNode::from_string("[3, 1, 2]");
    let root = Solution::replace_value_in_tree(root);
    assert_eq!(root.as_ref().unwrap().borrow().to_string(), "[0, 0, 0]");
}
