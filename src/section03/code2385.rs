#![allow(dead_code)]

/*

// 2385. Amount of Time for Binary Tree to Be Infected
// https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/
// https://leetcode.cn/problems/amount-of-time-for-binary-tree-to-be-infected/
//
// Medium
//
// You are given the root of a binary tree with unique values, and an integer start.
// At minute 0, an infection starts from the node with value start.

Each minute, a node becomes infected if:

    The node is currently uninfected.
    The node is adjacent to an infected node.

Return the number of minutes needed for the entire tree to be infected.

Example 1:

Input: root = [1,5,3,null,4,10,6,9,2], start = 3
Output: 4
Explanation: The following nodes are infected during:
- Minute 0: Node 3
- Minute 1: Nodes 1, 10 and 6
- Minute 2: Node 5
- Minute 3: Node 4
- Minute 4: Nodes 9 and 2
It takes 4 minutes for the whole tree to be infected so we return 4.

Example 2:

Input: root = [1], start = 1
Output: 0
Explanation: At minute 0, the only node in the tree is infected so we return 0.

Constraints:

    The number of nodes in the tree is in the range [1, 10^5].
    1 <= Node.val <= 10^5
    Each node has a unique value.
    A node with a value of start exists in the tree.
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        use std::collections::{HashMap, HashSet};

        pub fn collect_graph(root: Option<Rc<RefCell<TreeNode>>>) -> HashMap<i32, Vec<i32>> {
            let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut stack = vec![(root, -1)];
            while let Some((node, prev_val)) = stack.pop() {
                if let Some(node) = node {
                    let node = node.borrow();
                    let val = node.val;
                    map.entry(val).and_modify(|v| v.push(prev_val)).or_insert_with(|| vec![prev_val]);
                    map.entry(prev_val).and_modify(|v| v.push(val)).or_insert_with(|| vec![val]);
                    stack.push((node.left.clone(), val));
                    stack.push((node.right.clone(), val));
                }
            }
            map
        }

        let graph = collect_graph(root);

        let mut max_lev = 0;
        let mut stack = vec![(start, 0)];
        let mut visited = HashSet::new();
        visited.insert(start);

        while let Some((node, lev)) = stack.pop() {
            if let Some(nodes) = graph.get(&node) {
                for &next_node in nodes.iter().filter(|&&n| n > -1) {
                    if visited.insert(next_node) {
                        stack.push((next_node, lev + 1));
                        max_lev = max_lev.max(lev + 1);
                    }
                }
            }
        }
        max_lev
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[1,5,3,null,4,10,6,9,2]");
    assert_eq!(Solution::amount_of_time(root, 3), 4);

    let root = TreeNode::from_vec(&[Some(1)]);
    assert_eq!(Solution::amount_of_time(root, 1), 0);
}
