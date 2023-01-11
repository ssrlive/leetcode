#![allow(dead_code)]

// 1028. Recover a Tree From Preorder Traversal
// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/
// https://leetcode.cn/problems/recover-a-tree-from-preorder-traversal/
//
// We run a preorder depth-first search (DFS) on the root of a binary tree.
//
// At each node in this traversal, we output D dashes (where D is the depth of this node), then we output the value of this node.
// If the depth of a node is D, the depth of its immediate child is D + 1.  The depth of the root node is 0.
//
// If a node has only one child, that child is guaranteed to be the left child.
//
// Given the output traversal of this traversal, recover the tree and return its root.
//
// Example 1:
//
// Input: traversal = "1-2--3--4-5--6--7"
// Output: [1,2,5,3,4,6,7]
//
// Example 2:
//
// Input: traversal = "1-2--3---4-5--6---7"
// Output: [1,2,5,3,null,6,null,4,null,7]
//
// Example 3:
//
// Input: traversal = "1-401--349---90--88"
// Output: [1,401,null,349,88,90]
//
// Constraints:
//
// - The number of nodes in the original tree is in the range [1, 1000].
// - 1 <= Node.val <= 10^9
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<(i32, Rc<RefCell<TreeNode>>)> = vec![];
        let mut depth = 0;
        for node_val in traversal.split('-') {
            if node_val.is_empty() {
                depth += 1;
                continue;
            }
            let value = node_val.parse().unwrap();
            let child = Rc::new(RefCell::new(TreeNode::new(value)));
            if !stack.is_empty() {
                if stack[stack.len() - 1].0 == depth {
                    stack[stack.len() - 1].1.borrow_mut().left = Some(child.clone());
                } else {
                    while stack[stack.len() - 1].0 > depth {
                        stack.pop();
                    }
                    stack[stack.len() - 1].1.borrow_mut().right = Some(child.clone());
                }
            }
            stack.push((depth + 1, child));
            depth = 1;
        }
        Some(stack[0].1.clone())
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            "1-2--3--4-5--6--7",
            vec![Some(1), Some(2), Some(5), Some(3), Some(4), Some(6), Some(7)],
        ),
        (
            "1-2--3---4-5--6---7",
            vec![
                Some(1),
                Some(2),
                Some(5),
                Some(3),
                None,
                Some(6),
                None,
                Some(4),
                None,
                Some(7),
            ],
        ),
        (
            "1-401--349---90--88",
            vec![Some(1), Some(401), None, Some(349), Some(88), Some(90)],
        ),
    ];
    for (traversal, expected) in cases {
        let root = Solution::recover_from_preorder(traversal.to_string());
        let result = root.as_ref().unwrap().borrow().to_vec();
        assert_eq!(result, expected);
    }
}
