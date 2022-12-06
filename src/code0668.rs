#![allow(dead_code)]

// 668. Kth Smallest Element in a Sorted Matrix
// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
//
// Nearly everyone has used the Multiplication Table. The multiplication table of size m x n is
// an integer matrix mat where mat[i][j] == i * j (1-indexed).
//
// Given three integers m, n, and k, return the kth smallest element in the m x n multiplication table.
//
// Example 1:
//
// Input: m = 3, n = 3, k = 5
// Output: 3
// Explanation: The 5th smallest number is 3.
//
// Example 2:
//
// Input: m = 2, n = 3, k = 6
// Output: 6
// Explanation: The 6th smallest number is 6.
//
// Constraints:
//
// - 1 <= m, n <= 3 * 10^4
// - 1 <= k <= m * n
//

struct Solution;

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut left = 1;
        let mut right = m * n;
        while left < right {
            let mid = left + (right - left) / 2;
            let mut count = 0;
            for i in 1..=m {
                count += (mid / i).min(n);
            }
            if count < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_kth_number(3, 3, 5), 3);
    assert_eq!(Solution::find_kth_number(2, 3, 6), 6);
}
