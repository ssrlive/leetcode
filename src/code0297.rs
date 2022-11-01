#![allow(dead_code)]

// 297. Serialize and Deserialize Binary Tree
// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
//
// Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
//
// Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
//
// Example 1:
//
// Input: root = [1,2,3,null,null,4,5]
// Output: [1,2,3,null,null,4,5]
//
// Example 2:
//
// Input: root = []
// Output: []
//
// Example 3:
//
// Input: root = [1]
// Output: [1]
//
// Example 4:
//
// Input: root = [1,2]
// Output: [1,2]
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 104].
// -1000 <= Node.val <= 1000

use crate::treenode::TreeNode;

struct Codec;

use std::cell::RefCell;
use std::rc::Rc;
impl Codec {
    pub fn new() -> Self {
        Self
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut v = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if node.is_none() {
                v.push(None);
                continue;
            }
            v.push(Some(node.as_ref().unwrap().borrow().val));
            queue.push_back(node.as_ref().unwrap().borrow().left.clone());
            queue.push_back(node.as_ref().unwrap().borrow().right.clone());
        }
        while let Some(None) = v.last() {
            v.pop();
        }

        let f = |x: &Option<i32>| match x {
            Some(x) => x.to_string(),
            None => "null".to_string(),
        };
        v.iter().map(f).collect::<Vec<_>>().join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let f = |x: &str| match x {
            "null" => None,
            x => match x.parse::<i32>() {
                Ok(x) => Some(Rc::new(RefCell::new(TreeNode::new(x)))),
                _ => None,
            },
        };
        let nodes = data.split(',').map(f).collect::<Vec<_>>();
        if nodes.is_empty() {
            return None;
        }
        let root = nodes[0].clone();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < nodes.len() {
            let node = queue.pop_front()?;
            if let left @ Some(_) = nodes[i].clone() {
                node.as_ref()?.borrow_mut().left = left.clone();
                queue.push_back(left);
            }
            i += 1;
            if i >= nodes.len() {
                break;
            }
            if let right @ Some(_) = nodes[i].clone() {
                node.as_ref()?.borrow_mut().right = right.clone();
                queue.push_back(right);
            }
            i += 1;
        }
        root
    }
}

#[test]
fn test() {
    let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)];
    let root = TreeNode::from_vec(&v);
    let codec = Codec::new();
    let data = codec.serialize(root);
    assert_eq!(data, "1,2,3,null,null,4,5");
    let root = codec.deserialize(data);
    assert_eq!(root.as_ref().unwrap().borrow().to_vec(), v);
}
