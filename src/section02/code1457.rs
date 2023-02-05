#![allow(dead_code)]

/*

// 1457. Pseudo-Palindromic Paths in a Binary Tree
Medium

Given a binary tree where node values are digits from 1 to 9. A path in the binary tree is said to be pseudo-palindromic if at least one permutation of the node values in the path is a palindrome.

Return the number of pseudo-palindromic paths going from the root node to leaf nodes.

Example 1:

Input: root = [2,3,1,3,1,null,1]
Output: 2
Explanation: The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the red path [2,3,3], the green path [2,1,1], and the path [2,3,1]. Among these paths only red path and green path are pseudo-palindromic paths since the red path [2,3,3] can be rearranged in [3,2,3] (palindrome) and the green path [2,1,1] can be rearranged in [1,2,1] (palindrome).

Example 2:

Input: root = [2,1,1,1,3,null,null,null,null,null,1]
Output: 1
Explanation: The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the green path [2,1,1], the path [2,1,3,1], and the path [2,1]. Among these paths only the green path is pseudo-palindromic since [2,1,1] can be rearranged in [1,2,1] (palindrome).

Example 3:

Input: root = [9]
Output: 1

Constraints:

    The number of nodes in the tree is in the range [1, 10^5].
    1 <= Node.val <= 9
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![(root, 0u16)];
        let mut rez = 0;

        while let Some((node_rc, mut odds)) = stack.pop() {
            let node_ref = node_rc.as_ref().unwrap().borrow();
            odds ^= 1 << node_ref.val;
            match (node_ref.left.clone(), node_ref.right.clone()) {
                (None, None) => {
                    if odds.count_ones() < 2 {
                        rez += 1
                    }
                }
                (None, right @ Some(_)) => stack.push((right, odds)),
                (left @ Some(_), None) => stack.push((left, odds)),
                (left @ Some(_), right @ Some(_)) => {
                    stack.push((left, odds));
                    stack.push((right, odds));
                }
            }
        }

        rez
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[2,3,1,3,1,null,1]");
    assert_eq!(Solution::pseudo_palindromic_paths(root), 2);
    let root = TreeNode::from_string("[2,1,1,1,3,null,null,null,null,null,1]");
    assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
    let root = TreeNode::from_string("[9]");
    assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
}
