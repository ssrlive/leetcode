#![allow(dead_code)]

// 118. Pascal's Triangle
// https://leetcode.com/problems/pascals-triangle/
// https://leetcode.cn/problems/pascals-triangle/
//
// Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.
// In Pascal's triangle, each number is the sum of the two numbers directly above it.

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        for i in 0..num_rows {
            let mut row = Vec::new();
            for j in 0..=i {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    row.push(ret[i as usize - 1][j as usize - 1] + ret[i as usize - 1][j as usize]);
                }
            }
            ret.push(row);
        }
        ret
    }
}

#[test]
fn test_generate() {
    assert_eq!(
        Solution::generate(5),
        vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]
    );
}
