#![allow(dead_code)]

// 652. Find Duplicate Subtrees
// https://leetcode.com/problems/find-duplicate-subtrees/
// https://leetcode.cn/problems/find-duplicate-subtrees/
//
// Given the root of a binary tree, return all duplicate subtrees.
//
// For each kind of duplicate subtrees, you only need to return the root node of any one of them.
//
// Two trees are duplicate if they have the same structure with the same node values.
//
// Example 1:
//
// Input: root = [1,2,3,4,null,2,4,null,null,4]
// Output: [[2,4],[4]]
//
// Example 2:
//
// Input: root = [2,1,1]
// Output: [[1]]
//
// Example 3:
//
// Input: root = [2,2,2,3,null,3,null]
// Output: [[2,3],[3]]
//
// Constraints:
//
// - The number of the nodes in the tree will be in the range [1, 5000]
// - -200 <= Node.val <= 200
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut str_to_node_and_count = std::collections::HashMap::new();
        Solution::rec_duplicate(root, &mut str_to_node_and_count);

        str_to_node_and_count
            .into_iter()
            .filter(|&(_, (_, count))| count > 1)
            .map(|(_, (node, _))| Some(node))
            .collect()
    }

    fn rec_duplicate(
        root: Option<Rc<RefCell<TreeNode>>>,
        str_to_node_and_count: &mut std::collections::HashMap<String, (Rc<RefCell<TreeNode>>, i32)>,
    ) -> Option<String> {
        root.as_ref()?;

        match root {
            None => None,
            Some(node) => {
                let mut s = String::new();

                if let Some(r) = Solution::rec_duplicate(node.borrow().right.clone(), str_to_node_and_count) {
                    s.push('R');
                    s.push_str(&r);
                }

                if let Some(l) = Solution::rec_duplicate(node.borrow().left.clone(), str_to_node_and_count) {
                    s.push('L');
                    s.push_str(&l);
                }

                s.push('M');
                s.push_str(node.borrow().val.to_string().as_str());

                let mut count = 1;
                if str_to_node_and_count.contains_key(&s) {
                    let (_, c) = str_to_node_and_count.get(&s)?;
                    count += *c;
                }
                str_to_node_and_count.insert(s.clone(), (node, count));

                println!("s = {s}");

                Some(s)
            }
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        Some(2),
        Some(4),
        None,
        None,
        Some(4),
    ]);
    let result = Solution::find_duplicate_subtrees(root);
    println!("{:?}", result);
    assert_eq!(result.len(), 2);

    let root = TreeNode::from_vec(&[Some(2), Some(1), Some(1)]);
    let result = Solution::find_duplicate_subtrees(root);
    println!("{:?}", result);
    assert_eq!(result.len(), 1);

    let root = TreeNode::from_vec(&[Some(2), Some(2), Some(2), Some(3), None, Some(3), None]);
    let result = Solution::find_duplicate_subtrees(root);
    println!("{:?}", result);
    assert_eq!(result.len(), 2);

    let root = TreeNode::from_vec(&[
        Some(0),
        Some(0),
        Some(0),
        Some(0),
        None,
        None,
        Some(0),
        None,
        None,
        None,
        Some(0),
    ]);
    let result = Solution::find_duplicate_subtrees(root);
    println!("{:?}", result);
    assert_eq!(result.len(), 1);
}
