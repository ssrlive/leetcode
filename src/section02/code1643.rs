#![allow(dead_code)]

/*

// 1643. Kth Smallest Instructions
// https://leetcode.com/problems/kth-smallest-instructions/
// https://leetcode.cn/problems/kth-smallest-instructions/
//
// Hard
//
// Bob is standing at cell (0, 0), and he wants to reach destination: (row, column). He can only travel right and down. You are going to help Bob by providing instructions for him to reach destination.

The instructions are represented as a string, where each character is either:

    'H', meaning move horizontally (go right), or
    'V', meaning move vertically (go down).

Multiple instructions will lead Bob to destination. For example, if destination is (2, 3), both "HHHVV" and "HVHVH" are valid instructions.

However, Bob is very picky. Bob has a lucky number k, and he wants the kth lexicographically smallest instructions that will lead him to destination. k is 1-indexed.

Given an integer array destination and an integer k, return the kth lexicographically smallest instructions that will take Bob to destination.

Example 1:

Input: destination = [2,3], k = 1
Output: "HHHVV"
Explanation: All the instructions that reach (2, 3) in lexicographic order are as follows:
["HHHVV", "HHVHV", "HHVVH", "HVHHV", "HVHVH", "HVVHH", "VHHHV", "VHHVH", "VHVHH", "VVHHH"].

Example 2:

Input: destination = [2,3], k = 2
Output: "HHVHV"

Example 3:

Input: destination = [2,3], k = 3
Output: "HHVVH"

Constraints:

    destination.length == 2
    1 <= row, column <= 15
    1 <= k <= nCr(row + column, row), where nCr(a, b) denotes a choose b​​​​​.
*/

struct Solution;

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        fn c(n: usize, k: usize) -> usize {
            let mut ans = 1;
            for i in 0..k {
                ans = ans * (n - i) / (i + 1);
            }
            ans
        }

        let (mut row, mut col) = (destination[0] as usize, destination[1] as usize);
        let mut k = k as usize;
        let mut ans = String::new();
        while row > 0 || col > 0 {
            if row == 0 {
                ans.push('H');
                col -= 1;
            } else if col == 0 {
                ans.push('V');
                row -= 1;
            } else {
                let n = c(row + col - 1, col - 1);
                if k <= n {
                    ans.push('H');
                    col -= 1;
                } else {
                    ans.push('V');
                    row -= 1;
                    k -= n;
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        ((vec![2, 3], 1), "HHHVV".to_string()),
        ((vec![2, 3], 2), "HHVHV".to_string()),
        ((vec![2, 3], 3), "HHVVH".to_string()),
    ];
    for ((destination, k), expected) in cases {
        assert_eq!(Solution::kth_smallest_path(destination, k), expected);
    }
}
