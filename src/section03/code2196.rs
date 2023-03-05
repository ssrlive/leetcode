#![allow(dead_code)]

/*

// 2196. Create Binary Tree From Descriptions
// https://leetcode.com/problems/create-binary-tree-from-descriptions/
// https://leetcode.cn/problems/create-binary-tree-from-descriptions/
//
// Medium
//
// You are given a 2D integer array descriptions where descriptions[i] = [parenti, childi, isLefti] indicates that parenti is the parent of childi in a binary tree of unique values. Furthermore,

    If isLefti == 1, then childi is the left child of parenti.
    If isLefti == 0, then childi is the right child of parenti.

Construct the binary tree described by descriptions and return its root.

The test cases will be generated such that the binary tree is valid.

Example 1:

Input: descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
Output: [50,20,80,15,17,19]
Explanation: The root node is the node with value 50 since it has no parent.
The resulting binary tree is shown in the diagram.

Example 2:

Input: descriptions = [[1,2,1],[2,3,0],[3,4,1]]
Output: [1,2,null,null,3,4]
Explanation: The root node is the node with value 1 since it has no parent.
The resulting binary tree is shown in the diagram.

Constraints:

    1 <= descriptions.length <= 10^4
    descriptions[i].length == 3
    1 <= parenti, childi <= 10^5
    0 <= isLefti <= 1
    The binary tree described by descriptions is valid.
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::{HashMap, HashSet};
        fn build(d: &HashMap<i32, (Option<i32>, Option<i32>)>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some((l, r)) = d.get(&val) {
                let mut node = TreeNode::new(val);
                if l.is_some() {
                    node.left = build(d, l.unwrap());
                }
                if r.is_some() {
                    node.right = build(d, r.unwrap());
                }
                Some(Rc::new(RefCell::new(node)))
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(val))))
            }
        }

        let mut d: HashMap<i32, (Option<i32>, Option<i32>)> = HashMap::new();
        let mut c: HashSet<i32> = HashSet::new();
        for desc in descriptions.iter() {
            let org = d.entry(desc[0]).or_insert((None, None));
            let new = if desc[2] == 0 {
                (org.0, Some(desc[1]))
            } else {
                (Some(desc[1]), org.1)
            };
            d.insert(desc[0], new);
            c.insert(desc[1]);
        }
        let mut val = 0;
        for k in d.keys() {
            if !c.contains(k) {
                val = *k;
                break;
            }
        }
        build(&d, val)
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (
            vec![vec![20, 15, 1], vec![20, 17, 0], vec![50, 20, 1], vec![50, 80, 0], vec![80, 19, 1]],
            vec![Some(50), Some(20), Some(80), Some(15), Some(17), Some(19)],
        ),
        (
            vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]],
            vec![Some(1), Some(2), None, None, Some(3), Some(4)],
        ),
    ];
    for (descriptions, expected) in cases {
        let root = Solution::create_binary_tree(descriptions);
        assert_eq!(TreeNode::to_vec(&root), expected);
    }
}
