#![allow(dead_code)]

// 427. Construct Quad Tree
// https://leetcode.com/problems/construct-quad-tree/
//
// Given a n * n matrix grid of 0's and 1's only. We want to represent the grid with a Quad-Tree.
//
// Return the root of the Quad-Tree representing the grid.
//
// Notice that you can assign the value of a node to True or False when isLeaf is False, and both are accepted in the answer.
//
// A Quad-Tree is a tree data structure in which each internal node has exactly four children. Besides, each node has two attributes:
//
// val: True if the node represents a grid of 1's or False if the node represents a grid of 0's.
// isLeaf: True if the node is leaf node on the tree or False if the node has the four children.
// class Node {
//     public boolean val;
//     public boolean isLeaf;
//     public Node topLeft;
//     public Node topRight;
//     public Node bottomLeft;
//     public Node bottomRight;
// }
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
// The output represents the serialized format of a Quad-Tree using level order traversal, where null signifies a path terminator where no node exists below.
//
// It is very similar to the serialization of the binary tree. The only difference is that the node is represented as a list [isLeaf, val].
//
// If the value of isLeaf or val is True we represent it as 1 in the list [isLeaf, val] and if the value of isLeaf or val is False we represent it as 0.
//
// Example 1:
//
// Input: grid = [[0,1],[1,0]]
// Output: [[0,1],[1,0],[1,1],[1,1],[1,0]]
// Explanation: The explanation of this example is shown below:
// Notice that 0 represnts False and 1 represents True in the photo representing the Quad-Tree.
//
// Example 2:
//
// Input: grid = [[1,1,1,1,0,0,0,0],[1,1,1,1,0,0,0,0],[1,1,1,1,1,1,1,1],[1,1,1,1,1,1,1,1],[1,1,1,1,0,0,0,0],[1,1,1,1,0,0,0,0],[1,1,1,1,0,0,0,0],[1,1,1,1,0,0,0,0]]
// Output: [[0,1],[1,1],[0,1],[1,1],[1,0],null,null,null,null,[1,0],[1,0],[1,1],[1,1]]
// Explanation: All values in the grid are not the same. We divide the grid into four sub-grids.
// The topLeft, bottomLeft and bottomRight each has the same value.
// The topRight have different values so we divide it into 4 sub-grids where each has the same value.
// Explanation is shown in the photo below:
//
// Constraints:
//
// - n == grid.length == grid[i].length
// - n == 2x where 0 <= x <= 6
//

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: bool,
    pub is_leaf: bool,
    pub top_left: Option<Box<Node>>,
    pub top_right: Option<Box<Node>>,
    pub bottom_left: Option<Box<Node>>,
    pub bottom_right: Option<Box<Node>>,
}

impl Node {
    #[inline]
    pub fn new(
        val: bool,
        is_leaf: bool,
        top_left: Option<Box<Node>>,
        top_right: Option<Box<Node>>,
        bottom_left: Option<Box<Node>>,
        bottom_right: Option<Box<Node>>,
    ) -> Self {
        Node {
            val,
            is_leaf,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
}

struct Solution;

impl Solution {
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Box<Node>> {
        Self::helper(&grid, 0, 0, grid.len())
    }

    fn helper(grid: &Vec<Vec<i32>>, x: usize, y: usize, len: usize) -> Option<Box<Node>> {
        if len == 1 {
            return Some(Box::new(Node::new(grid[x][y] != 0, true, None, None, None, None)));
        }
        let mut result = Node::new(false, false, None, None, None, None);
        let top_left = Self::helper(grid, x, y, len / 2);
        let top_right = Self::helper(grid, x, y + len / 2, len / 2);
        let bottom_left = Self::helper(grid, x + len / 2, y, len / 2);
        let bottom_right = Self::helper(grid, x + len / 2, y + len / 2, len / 2);
        if top_left.as_ref()?.is_leaf
            && top_right.as_ref()?.is_leaf
            && bottom_left.as_ref()?.is_leaf
            && bottom_right.as_ref()?.is_leaf
            && top_left.as_ref()?.val == top_right.as_ref()?.val
            && top_right.as_ref()?.val == bottom_left.as_ref()?.val
            && bottom_left.as_ref()?.val == bottom_right.as_ref()?.val
        {
            result.is_leaf = true;
            result.val = top_left.as_ref()?.val;
        } else {
            result.top_left = top_left;
            result.top_right = top_right;
            result.bottom_left = bottom_left;
            result.bottom_right = bottom_right;
        }
        Some(Box::new(result))
    }
}
