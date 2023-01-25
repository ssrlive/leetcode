#![allow(dead_code)]

// 1110. Delete Nodes And Return Forest
// https://leetcode.com/problems/delete-nodes-and-return-forest/
// https://leetcode.cn/problems/delete-nodes-and-return-forest/
//
// Given the root of a binary tree, each node in the tree has a distinct value.
//
// After deleting all nodes with a value in to_delete, we are left with a forest (a disjoint union of trees).
//
// Return the roots of the trees in the remaining forest. You may return the result in any order.
//
// Example 1:
//
// Input: root = [1,2,3,4,5,6,7], to_delete = [3,5]
// Output: [[1,2,null,4],[6],[7]]
//
// Example 2:
//
// Input: root = [1,2,4,null,3], to_delete = [3]
// Output: [[1,2,4]]
//
// Constraints:
//
// - The number of nodes in the given tree is at most 1000.
// - Each node has a distinct value between 1 and 1000.
// - to_delete.length <= 1000
// - to_delete contains distinct values between 1 and 1000.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        use std::collections::HashSet;

        fn collect(
            root: &mut Option<Rc<RefCell<TreeNode>>>,
            ret: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
            s: &HashSet<i32>,
        ) {
            let mut node = root.as_ref().unwrap().borrow_mut();

            if node.left.is_some() {
                collect(&mut node.left, ret, s);

                let val = node.left.as_ref().unwrap().borrow().val;

                if s.contains(&node.val) && !s.contains(&val) {
                    ret.push(node.left.take());
                }
                if s.contains(&val) {
                    node.left = None;
                }
            }

            if node.right.is_some() {
                collect(&mut node.right, ret, s);

                let val = node.right.as_ref().unwrap().borrow().val;

                if s.contains(&node.val) && !s.contains(&val) {
                    ret.push(node.right.take());
                }
                if s.contains(&val) {
                    node.right = None;
                }
            }
        }

        let mut ret = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
        if root.is_none() {
            return ret;
        }

        let mut s = HashSet::new();
        for a in to_delete {
            s.insert(a);
        }

        let mut root = root;
        collect(&mut root, &mut ret, &s);

        let val = root.as_ref().unwrap().borrow().val;
        if !s.contains(&val) {
            ret.push(root);
        }

        ret
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
    let to_delete = vec![3, 5];
    let ret = Solution::del_nodes(root, to_delete);
    let mut ret = ret.iter().map(|x| TreeNode::to_vec(x)).collect::<Vec<_>>();
    ret.sort();
    assert_eq!(
        ret,
        vec![vec![Some(1), Some(2), None, Some(4)], vec![Some(6)], vec![Some(7)]]
    );

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(4), None, Some(3)]);
    let to_delete = vec![3];
    let ret = Solution::del_nodes(root, to_delete);
    let mut ret = ret.iter().map(|x| TreeNode::to_vec(x)).collect::<Vec<_>>();
    ret.sort();
    assert_eq!(ret, vec![vec![Some(1), Some(2), Some(4)]]);
}
