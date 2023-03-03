#![allow(dead_code)]

/*

// 2096. Step-By-Step Directions From a Binary Tree Node to Another
// https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/
// https://leetcode.cn/problems/step-by-step-directions-from-a-binary-tree-node-to-another/
//
// Medium
//
// You are given the root of a binary tree with n nodes. Each node is uniquely assigned a value from 1 to n. You are also given an integer startValue representing the value of the start node s, and a different integer destValue representing the value of the destination node t.

Find the shortest path starting from node s and ending at node t. Generate step-by-step directions of such path as a string consisting of only the uppercase letters 'L', 'R', and 'U'. Each letter indicates a specific direction:

    'L' means to go from a node to its left child node.
    'R' means to go from a node to its right child node.
    'U' means to go from a node to its parent node.

Return the step-by-step directions of the shortest path from node s to node t.

Example 1:

Input: root = [5,1,2,3,null,6,4], startValue = 3, destValue = 6
Output: "UURL"
Explanation: The shortest path is: 3 → 1 → 5 → 2 → 6.

Example 2:

Input: root = [2,1], startValue = 2, destValue = 1
Output: "L"
Explanation: The shortest path is: 2 → 1.

Constraints:

    The number of nodes in the tree is n.
    2 <= n <= 10^5
    1 <= Node.val <= n
    All the values in the tree are unique.
    1 <= startValue, destValue <= n
    startValue != destValue
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        let mut path_to_start: Vec<char> = Vec::new();
        let mut path_to_dest: Vec<char> = Vec::new();

        Self::find_path(&root, start_value, &mut path_to_start);
        Self::find_path(&root, dest_value, &mut path_to_dest);

        let mut start_idx = 0;
        for idx in 0..(path_to_start.len()).min(path_to_dest.len()) {
            if path_to_start[idx] == path_to_dest[idx] {
                start_idx = idx + 1;
            } else {
                break;
            }
        }

        path_to_start = path_to_start[start_idx..].to_vec();
        path_to_dest = path_to_dest[start_idx..].to_vec();

        let mut path_to_start_rev = vec!['U'; path_to_start.len()];
        path_to_start_rev.append(&mut path_to_dest);
        path_to_start_rev.iter().collect()
    }

    fn find_path(root: &Option<Rc<RefCell<TreeNode>>>, value: i32, path: &mut Vec<char>) -> bool {
        match root {
            None => false,
            Some(node) => {
                if node.borrow().val == value {
                    true
                } else {
                    path.push('L');
                    if Self::find_path(&node.borrow().left, value, path) {
                        return true;
                    }
                    path.pop();
                    path.push('R');
                    if Self::find_path(&node.borrow().right, value, path) {
                        return true;
                    }
                    path.pop();
                    false
                }
            }
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[5,1,2,3,null,6,4]");
    assert_eq!(Solution::get_directions(root, 3, 6), "UURL".to_string());

    let root = TreeNode::from_string("[2,1]");
    assert_eq!(Solution::get_directions(root, 2, 1), "L".to_string());
}
