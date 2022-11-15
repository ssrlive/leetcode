#![allow(dead_code)]

// 120. Triangle
// https://leetcode.com/problems/triangle/
//
// Given a triangle array, return the minimum path sum from top to bottom.
// For each step, you may move to an adjacent number of the row below. More formally,
// if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.
//

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.is_empty() {
            return 0;
        }
        let mut dp = triangle;
        for i in 1..dp.len() {
            for j in 0..dp[i].len() {
                let left: i32 = if j > 0 { dp[i - 1][j - 1] } else { i32::MAX };
                let right: i32 = if j < dp[i - 1].len() { dp[i - 1][j] } else { i32::MAX };

                dp[i][j] += if left < right { left } else { right };
            }
        }
        // println!("{:?}", dp);
        *dp[dp.len() - 1].iter().min().unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_total(vec![vec![10]]), 10);
    assert_eq!(
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
}
