#![allow(dead_code)]

// 606. Construct String from Binary Tree
// https://leetcode.com/problems/construct-string-from-binary-tree/
// https://leetcode.cn/problems/construct-string-from-binary-tree/
//
// Given the root of a binary tree, construct a string consisting of parenthesis and integers from a binary tree with the preorder traversal way, and return it.
//
// Omit all the empty parenthesis pairs that do not affect the one-to-one mapping relationship between the string and the original binary tree.
//
// Example 1:
//
// Input: root = [1,2,3,4]
// Output: "1(2(4))(3)"
// Explanation: Originally, it needs to be "1(2(4)())(3()())", but you need to omit all the unnecessary empty parenthesis pairs. And it will be "1(2(4))(3)"
//
// Example 2:
//
// Input: root = [1,2,3,null,4]
// Output: "1(2()(4))(3)"
// Explanation: Almost the same as the first example, except we cannot omit the first parenthesis pair to break the one-to-one mapping relationship between the input and the output.
//
// Constraints:
//
// The number of nodes in the tree is in the range [1, 10^4].
// -1000 <= Node.val <= 1000
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        pub fn _tree2str(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
            if let Some(node) = root {
                let node = node.borrow();
                let mut s = node.val.to_string();
                if node.left.is_some() || node.right.is_some() {
                    s.push('(');
                    s.push_str(&_tree2str(&node.left));
                    s.push(')');
                }
                if node.right.is_some() {
                    s.push('(');
                    s.push_str(&_tree2str(&node.right));
                    s.push(')');
                }
                s
            } else {
                "".to_string()
            }
        }
        _tree2str(&root)
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::tree2str(TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4)])),
        "1(2(4))(3)".to_string()
    );
    assert_eq!(
        Solution::tree2str(TreeNode::from_vec(&[Some(1), Some(2), Some(3), None, Some(4)])),
        "1(2()(4))(3)".to_string()
    );
}
