#![allow(dead_code)]

// 119. Pascal's Triangle II
// https://leetcode.com/problems/pascals-triangle-ii/
// https://leetcode.cn/problems/pascals-triangle-ii/
//
// Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.
// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
//

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![1];
        for i in 1..=row_index {
            let mut next_row = vec![1; i as usize + 1];
            for j in 1..i {
                next_row[j as usize] = row[j as usize - 1] + row[j as usize];
            }
            row = next_row;
        }
        row
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    assert_eq!(Solution::get_row(0), vec![1]);
    assert_eq!(Solution::get_row(1), vec![1, 1]);
}
