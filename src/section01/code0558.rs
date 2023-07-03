#![allow(dead_code)]

// 558. Logical OR of Two Binary Grids Represented as Quad-Trees
// https://leetcode.com/problems/logical-or-of-two-binary-grids-represented-as-quad-trees/
// https://leetcode.cn/problems/logical-or-of-two-binary-grids-represented-as-quad-trees/
//
// A Binary Matrix is a matrix in which all the elements are either 0 or 1.
//
// Given quadTree1 and quadTree2. quadTree1 represents a n * n binary matrix and quadTree2 represents another n * n binary matrix.
//
// Return a Quad-Tree representing the n * n binary matrix which is the result of logical bitwise OR of the two binary
// matrixes represented by quadTree1 and quadTree2.
//
// Notice that you can assign the value of a node to True or False when isLeaf is False, and both are accepted in the answer.
//
// A Quad-Tree is a tree data structure in which each internal node has exactly four children. Besides, each node has two attributes:
//
// - val: True if the node represents a grid of 1's or False if the node represents a grid of 0's.
// - isLeaf: True if the node is leaf node on the tree or False if the node has the four children.
//
//   class Node {
//       public boolean val;
//       public boolean isLeaf;
//       public Node topLeft;
//       public Node topRight;
//       public Node bottomLeft;
//       public Node bottomRight;
//   }
//
// We can construct a Quad-Tree from a two-dimensional area using the following steps:
//
// If the current grid has the same value (i.e all 1's or all 0's) set isLeaf True and set val to the value of the grid and set the four children to Null and stop.
// If the current grid has different values, set isLeaf to False and set val to any value and divide the current grid into four sub-grids as shown in the photo.
// Recurse for each of the children with the proper sub-grid.
//
// If you want to know more about the Quad-Tree, you can refer to the wiki.
//
// Quad-Tree format:
//
// The input/output represents the serialized format of a Quad-Tree using level order traversal,
// where null signifies a path terminator where no node exists below.
//
// It is very similar to the serialization of the binary tree. The only difference is that the node is represented as a list [isLeaf, val].
//
// If the value of isLeaf or val is True we represent it as 1 in the list [isLeaf, val] and if the value of isLeaf or val is False we represent it as 0.
//
// Example 1:
//
// Input: quadTree1 = [[0,1],[1,1],[1,1],[1,0],[1,0]]
// , quadTree2 = [[0,1],[1,1],[0,1],[1,1],[1,0],null,null,null,null,[1,0],[1,0],[1,1],[1,1]]
// Output: [[0,0],[1,1],[1,1],[1,1],[1,0]]
// Explanation: quadTree1 and quadTree2 are shown above. You can see the binary matrix which is represented by each Quad-Tree.
// If we apply logical bitwise OR on the two binary matrices we get the binary matrix below which is represented by the result Quad-Tree.
// Notice that the binary matrices shown are only for illustration, you don't have to construct the binary matrix to get the result tree.
//
// Example 2:
//
// Input: quadTree1 = [[1,0]], quadTree2 = [[1,0]]
// Output: [[1,0]]
// Explanation: Each tree represents a binary matrix of size 1*1. Each matrix contains only zero.
// The resulting matrix is of size 1*1 with also zero.
//
// Constraints:
//
// - quadTree1 and quadTree2 are both valid Quad-Trees each representing a n * n grid.
// - n == 2^x where 0 <= x <= 9.
//
// Follow up: Can you solve it in both recursively and iteratively?

use crate::quadtree::Node;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn intersect(quad_tree1: Option<Rc<RefCell<Node>>>, quad_tree2: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if quad_tree1.is_none() {
            return quad_tree2;
        }
        if quad_tree2.is_none() {
            return quad_tree1;
        }
        if quad_tree1.as_ref()?.borrow().is_leaf() {
            return if quad_tree1.as_ref()?.borrow().val() {
                quad_tree1
            } else {
                quad_tree2
            };
        }
        if quad_tree2.as_ref()?.borrow().is_leaf() {
            return if quad_tree2.as_ref()?.borrow().val() {
                quad_tree2
            } else {
                quad_tree1
            };
        }
        let tl = Solution::intersect(quad_tree1.as_ref()?.borrow().top_left(), quad_tree2.as_ref()?.borrow().top_left());
        let tr = Solution::intersect(quad_tree1.as_ref()?.borrow().top_right(), quad_tree2.as_ref()?.borrow().top_right());
        let bl = Solution::intersect(
            quad_tree1.as_ref()?.borrow().bottom_left(),
            quad_tree2.as_ref()?.borrow().bottom_left(),
        );
        let br = Solution::intersect(
            quad_tree1.as_ref()?.borrow().bottom_right(),
            quad_tree2.as_ref()?.borrow().bottom_right(),
        );
        if tl.is_some()
            && tr.is_some()
            && bl.is_some()
            && br.is_some()
            && tl.as_ref()?.borrow().is_leaf()
            && tr.as_ref()?.borrow().is_leaf()
            && bl.as_ref()?.borrow().is_leaf()
            && br.as_ref()?.borrow().is_leaf()
            && tl.as_ref()?.borrow().val() == tr.as_ref()?.borrow().val()
            && tl.as_ref()?.borrow().val() == bl.as_ref()?.borrow().val()
            && tl.as_ref()?.borrow().val() == br.as_ref()?.borrow().val()
        {
            Some(Rc::new(RefCell::new(Node::new(
                true,
                tl.as_ref()?.borrow().val(),
                None,
                None,
                None,
                None,
            ))))
        } else {
            Some(Rc::new(RefCell::new(Node::new(false, false, tl, tr, bl, br))))
        }
    }
}

#[test]
fn test() {
    let leaf_false = Some(Rc::new(RefCell::new(Node::new(true, false, None, None, None, None))));
    let leaf_true = Some(Rc::new(RefCell::new(Node::new(true, true, None, None, None, None))));

    let quad_tree1 = Some(Rc::new(RefCell::new(Node::new(
        false,
        true,
        leaf_true.clone(),
        leaf_true.clone(),
        leaf_false.clone(),
        leaf_false.clone(),
    ))));

    let quad_tree2_tr = Some(Rc::new(RefCell::new(Node::new(
        false,
        false,
        leaf_false.clone(),
        leaf_false.clone(),
        leaf_true.clone(),
        leaf_true.clone(),
    ))));

    let quad_tree2 = Some(Rc::new(RefCell::new(Node::new(
        false,
        true,
        leaf_true.clone(),
        quad_tree2_tr,
        leaf_true.clone(),
        leaf_false.clone(),
    ))));

    let res = Some(Rc::new(RefCell::new(Node::new(
        false,
        false,
        leaf_true.clone(),
        leaf_true.clone(),
        leaf_true.clone(),
        leaf_false.clone(),
    ))));
    assert_eq!(Solution::intersect(quad_tree1, quad_tree2), res);

    let quad_tree1 = leaf_false.clone();
    let quad_tree2 = leaf_false.clone();
    let res = leaf_false.clone();
    assert_eq!(Solution::intersect(quad_tree1, quad_tree2), res);
}
