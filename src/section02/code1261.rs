#![allow(dead_code)]

// 1261. Find Elements in a Contaminated Binary Tree
// https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/
// https://leetcode.cn/problems/find-elements-in-a-contaminated-binary-tree/
//
// Medium
//
// Given a binary tree with the following rules:
//
//     root.val == 0
//     If treeNode.val == x and treeNode.left != null, then treeNode.left.val == 2 * x + 1
//     If treeNode.val == x and treeNode.right != null, then treeNode.right.val == 2 * x + 2
//
// Now the binary tree is contaminated, which means all treeNode.val have been changed to -1.
//
// Implement the FindElements class:
//
//     FindElements(TreeNode* root) Initializes the object with a contaminated binary tree and recovers it.
//     bool find(int target) Returns true if the target value exists in the recovered binary tree.
//
// Example 1:
//
// Input
// ["FindElements","find","find"]
// [[[-1,null,-1]],[1],[2]]
// Output
// [null,false,true]
// Explanation
// FindElements findElements = new FindElements([-1,null,-1]);
// findElements.find(1); // return False
// findElements.find(2); // return True
//
// Example 2:
//
// Input
// ["FindElements","find","find","find"]
// [[[-1,-1,-1,-1,-1]],[1],[3],[5]]
// Output
// [null,true,true,false]
// Explanation
// FindElements findElements = new FindElements([-1,-1,-1,-1,-1]);
// findElements.find(1); // return True
// findElements.find(3); // return True
// findElements.find(5); // return False
//
// Example 3:
//
// Input
// ["FindElements","find","find","find","find"]
// [[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
// Output
// [null,true,false,false,true]
// Explanation
// FindElements findElements = new FindElements([-1,null,-1,-1,null,-1]);
// findElements.find(2); // return True
// findElements.find(3); // return False
// findElements.find(4); // return False
// findElements.find(5); // return True
//
// Constraints:
//
// -    TreeNode.val == -1
// -    The height of the binary tree is less than or equal to 20
// -    The total number of nodes is between [1, 10^4]
// -    Total calls of find() is between [1, 10^4]
// -    0 <= target <= 10^6
//

use crate::treenode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

struct FindElements {
    tree: OptNode,
}

impl FindElements {
    fn new(root: OptNode) -> Self {
        Self::recover(&root, 0);
        Self { tree: root }
    }

    fn recover(node: &OptNode, val: i32) {
        if let Some(n) = node.as_ref() {
            let mut b = n.borrow_mut();
            b.val = val;
            Self::recover(&b.left, val * 2 + 1);
            Self::recover(&b.right, val * 2 + 2);
        }
    }

    fn find(&self, target: i32) -> bool {
        let route = target as u32 + 1;
        let bit = (route + 1).next_power_of_two() >> 2;
        Self::find_node(&self.tree, route, bit)
    }

    fn find_node(node: &OptNode, route: u32, bit: u32) -> bool {
        match node.as_ref() {
            None => false,
            Some(n) => {
                if bit == 0 {
                    true
                } else if route & bit == 0 {
                    Self::find_node(&n.borrow().left, route, bit >> 1)
                } else {
                    Self::find_node(&n.borrow().right, route, bit >> 1)
                }
            }
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![Some(-1), None, Some(-1)], vec![1, 2], vec![false, true]),
        (
            vec![Some(-1), Some(-1), Some(-1), Some(-1), Some(-1)],
            vec![1, 3, 5],
            vec![true, true, false],
        ),
        (
            vec![Some(-1), None, Some(-1), Some(-1), None, Some(-1)],
            vec![2, 3, 4, 5],
            vec![true, false, false, true],
        ),
    ];
    for (tree, targets, expected) in cases {
        let tree = TreeNode::from_vec(&tree);
        let find_elements = FindElements::new(tree);
        for (target, expect) in targets.into_iter().zip(expected) {
            assert_eq!(find_elements.find(target), expect);
        }
    }
}
