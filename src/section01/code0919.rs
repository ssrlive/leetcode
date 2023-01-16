#![allow(dead_code)]

// 919. Complete Binary Tree Inserter
// https://leetcode.com/problems/complete-binary-tree-inserter/
// https://leetcode.cn/problems/complete-binary-tree-inserter/
//
// A complete binary tree is a binary tree in which every level, except possibly the last,
// is completely filled, and all nodes are as far left as possible.
//
// Design an algorithm to insert a new node to a complete binary tree keeping it complete after the insertion.
//
// Implement the CBTInserter class:
//
// CBTInserter(TreeNode root) Initializes the data structure with the root of the complete binary tree.
// int insert(int v) Inserts a TreeNode into the tree with value Node.val == val so that the tree remains complete,
// and returns the value of the parent of the inserted TreeNode.
// TreeNode get_root() Returns the root node of the tree.
//
// Example 1:
//
// Input
// ["CBTInserter", "insert", "insert", "get_root"]
// [[[1, 2]], [3], [4], []]
// Output
// [null, 1, 2, [1, 2, 3, 4]]
//
// Explanation
// CBTInserter cBTInserter = new CBTInserter([1, 2]);
// cBTInserter.insert(3);  // return 1
// cBTInserter.insert(4);  // return 2
// cBTInserter.get_root(); // return [1, 2, 3, 4]
//
// Constraints:
//
// - The number of nodes in the tree will be in the range [1, 1000].
// - 0 <= Node.val <= 5000
// - root is a complete binary tree.
// - 0 <= val <= 5000
// - At most 104 calls will be made to insert and get_root.
//

use crate::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

struct CBTInserter {
    nodes: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                if node.is_some() {
                    nodes.push(node.clone());
                    queue.push_back(node.as_ref().unwrap().borrow().left.clone());
                    queue.push_back(node.as_ref().unwrap().borrow().right.clone());
                }
            }
        }
        Self { nodes }
    }

    fn insert(&mut self, v: i32) -> i32 {
        let n = self.nodes.len();
        let node = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        self.nodes.push(node.clone());
        if n % 2 != 0 {
            self.nodes[(n - 1) / 2].as_ref().unwrap().borrow_mut().left = node;
        } else {
            self.nodes[(n - 1) / 2].as_ref().unwrap().borrow_mut().right = node;
        }
        self.nodes[(n - 1) / 2].as_ref().unwrap().borrow().val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.nodes[0].clone()
    }
}

#[test]
fn test() {
    let mut obj = CBTInserter::new(TreeNode::from_vec(&[Some(1), Some(2)]));
    assert_eq!(obj.insert(3), 1);
    assert_eq!(obj.insert(4), 2);
    let expected = vec![Some(1), Some(2), Some(3), Some(4)];
    assert_eq!(obj.get_root().unwrap().borrow().to_vec(), expected);

    let mut obj = CBTInserter::new(TreeNode::from_vec(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
    ]));
    assert_eq!(obj.insert(7), 3);
    assert_eq!(obj.insert(8), 4);
    let expected = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8)];
    assert_eq!(obj.get_root().unwrap().borrow().to_vec(), expected);
}
