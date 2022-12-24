#![allow(dead_code)]

// 96. Unique Binary Search Trees
// https://leetcode.com/problems/unique-binary-search-trees/
// https://leetcode.cn/problems/unique-binary-search-trees/
//
// Given an integer n, return the number of structurally unique BST's (binary search trees)
// which has exactly n nodes of unique values from 1 to n.
//
// Example 1:
//
// Input: n = 3
// Output: 5
//
// Example 2:
//
// Input: n = 1
// Output: 1
//
// Constraints:
//
// - 1 <= n <= 19
//

struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    let cases = vec![
        (3, 5),
        (1, 1),
        (2, 2),
        (4, 14),
        (5, 42),
        (6, 132),
        (7, 429),
        (8, 1430),
        (9, 4862),
        (10, 16796),
        (11, 58786),
        (12, 208012),
        (13, 742900),
        (14, 2674440),
        (15, 9694845),
        (16, 35357670),
        (17, 129644790),
        (18, 477638700),
        (19, 1767263190),
    ];
    for (n, expected) in cases {
        assert_eq!(Solution::num_trees(n), expected);
    }
}
