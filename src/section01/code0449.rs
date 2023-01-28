#![allow(dead_code)]

// 449. Serialize and Deserialize BST
// https://leetcode.com/problems/serialize-and-deserialize-bst/
// https://leetcode.cn/problems/serialize-and-deserialize-bst/
//
// Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
//
// Design an algorithm to serialize and deserialize a binary search tree. There is no restriction on how your serialization/deserialization algorithm should work. You need to ensure that a binary search tree can be serialized to a string, and this string can be deserialized to the original tree structure.
//
// The encoded string should be as compact as possible.
//
// Example 1:
//
// Input: root = [2,1,3]
// Output: [2,1,3]
//
// Example 2:
//
// Input: root = []
// Output: []
//
// Constraints:
//
// - The number of nodes in the tree is in the range [0, 10^4].
// - 0 <= Node.val <= 10^4
// - The input tree is guaranteed to be a binary search tree.
//
// Follow up: Could you design your serialization algorithm such that it does not use extra space?

use crate::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "".to_string();
        }
        let nodes = Self::pre_order_traversal(&root);
        format!("{nodes:?}")
    }

    pub fn pre_order_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();
                ret.push(v.val);
                helper(&v.left, ret);
                helper(&v.right, ret);
            }
        }
        let mut ret = vec![];
        helper(root, &mut ret);
        ret
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let data = Self::parse(&data)?
            .iter()
            .map(|v| v.parse::<i32>().unwrap_or_default())
            .collect::<Vec<_>>();
        let mut root = None;
        for &n in data.iter() {
            if root.is_none() {
                root = Some(Rc::new(RefCell::new(TreeNode::new(n))));
            } else {
                Self::insert(root.clone(), n);
            }
        }
        root
    }

    pub fn insert(root: Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(v) = root {
            let mut v = v.borrow_mut();
            if val <= v.val {
                if v.left.is_none() {
                    v.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                } else {
                    Self::insert(v.left.clone(), val);
                }
            } else if v.right.is_none() {
                v.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            } else {
                Self::insert(v.right.clone(), val);
            }
        }
    }

    fn parse(input: &str) -> Option<Vec<String>> {
        let mut part = String::new();
        let mut collected = Vec::new();
        let mut char_iter = input.chars();
        if char_iter.next() != Some('[') {
            return None;
        }
        loop {
            match char_iter.next()? {
                ']' => {
                    collected.push(part);
                    return Some(collected);
                }
                ',' | ' ' => {
                    if !part.is_empty() {
                        collected.push(part);
                        part = String::new();
                    }
                }
                x => part.push(x),
            }
        }
    }
}

#[test]
fn test_serialize() {
    let codec = Codec::new();
    let root = TreeNode::from_vec(&[Some(2), Some(1), Some(3)]);
    let ans = "[2, 1, 3]";
    assert_eq!(codec.serialize(root), ans);
    assert_eq!(
        codec.deserialize(ans.to_string()),
        TreeNode::from_vec(&[Some(2), Some(1), Some(3)])
    );
}
