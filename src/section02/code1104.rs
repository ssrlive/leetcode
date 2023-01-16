#![allow(dead_code)]

// 1104. Path In Zigzag Labelled Binary Tree
// https://leetcode.com/problems/path-in-zigzag-labelled-binary-tree/
// https://leetcode.cn/problems/path-in-zigzag-labelled-binary-tree/
//
// In an infinite binary tree where every node has two children, the nodes are labelled in row order.
//
// In the odd numbered rows (ie., the first, third, fifth,...), the labelling is left to right,
// while in the even numbered rows (second, fourth, sixth,...), the labelling is right to left.
//
// Given the label of a node in this tree, return the labels in the path from the root of the tree to the node with that label.
//
// Example 1:
//
// Input: label = 14
// Output: [1,3,4,14]
//
// Example 2:
//
// Input: label = 26
// Output: [1,2,6,10,26]
//
// Constraints:
//
// - 1 <= label <= 10^6
//

struct Solution;

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut label = label;
        let mut vec = vec![];
        while label > 0 {
            vec.push(label);
            label /= 2;
            if label == 0 {
                break;
            }
            let mut exp = 0;
            while 2i32.pow(exp) <= label {
                exp += 1;
            }
            label = 2i32.pow(exp) - 1 - (label - 2i32.pow(exp - 1));
        }
        vec.iter().copied().rev().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::path_in_zig_zag_tree(14), vec![1, 3, 4, 14]);
    assert_eq!(Solution::path_in_zig_zag_tree(26), vec![1, 2, 6, 10, 26]);
}
