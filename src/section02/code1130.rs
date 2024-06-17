#![allow(dead_code)]

// 1130. Minimum Cost Tree From Leaf Values
// https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/
// https://leetcode.cn/problems/minimum-cost-tree-from-leaf-values/
//
// Given an array arr of positive integers, consider all binary trees such that:
//
// Each node has either 0 or 2 children;
// The values of arr correspond to the values of each leaf in an in-order traversal of the tree.
// The value of each non-leaf node is equal to the product of the largest leaf value in its left and right subtree, respectively.
// Among all possible binary trees considered, return the smallest possible sum of the values of each non-leaf node.
// It is guaranteed this sum fits into a 32-bit integer.
//
// A node is a leaf if and only if it has zero children.
//
// Example 1:
//
// Input: arr = [6,2,4]
// Output: 32
// Explanation: There are two possible trees shown.
// The first has a non-leaf node sum 36, and the second has non-leaf node sum 32.
//
// Example 2:
//
// Input: arr = [4,11]
// Output: 44
//
// Constraints:
//
// - 2 <= arr.length <= 40
// - 1 <= arr[i] <= 15
// - It is guaranteed that the answer fits into a 32-bit signed integer (i.e., it is less than 231).
//

struct Solution;

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        pub fn min_index(array: &[i32]) -> usize {
            let mut i = 0;
            for (j, &value) in array.iter().enumerate() {
                if value < array[i] {
                    i = j;
                }
            }
            i
        }

        use std::cmp::min;
        let mut arr = arr;
        let mut a: Vec<i32> = vec![];
        a.push(i32::MAX);
        a.append(&mut arr);
        a.push(i32::MAX);
        let mut res = 0;
        while a.len() > 3 {
            let min_index = min_index(&a);
            res += min(a[min_index - 1], a[min_index + 1]) * a[min_index];
            a.remove(min_index);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::mct_from_leaf_values(vec![6, 2, 4]), 32);
    assert_eq!(Solution::mct_from_leaf_values(vec![4, 11]), 44);
}
