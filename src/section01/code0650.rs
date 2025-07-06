#![allow(dead_code)]

// 650. 2 Keys Keyboard
// https://leetcode.com/problems/2-keys-keyboard/
// https://leetcode.cn/problems/2-keys-keyboard/
//
// There is only one character 'A' on the screen of a notepad. You can perform one of two operations on this notepad for each step:
//
// Copy All: You can copy all the characters present on the screen (a partial copy is not allowed).
// Paste: You can paste the characters which are copied last time.
// Given an integer n, return the minimum number of operations to get the character 'A' exactly n times on the screen.
//
// Example 1:
//
// Input: n = 3
// Output: 3
// Explanation: Initially, we have one character 'A'.
// In step 1, we use Copy All operation.
// In step 2, we use Paste operation to get 'AA'.
// In step 3, we use Paste operation to get 'AAA'.
//
// Example 2:
//
// Input: n = 1
// Output: 0
//
// Constraints:
//
// - 1 <= n <= 1000
//

struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for i in 2..=n {
            dp[i] = i;
            for j in 2..=i / 2 {
                if i.is_multiple_of(j) {
                    dp[i] = dp[j] + dp[i / j];
                    break;
                }
            }
        }
        dp[n] as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_steps(3), 3);
    assert_eq!(Solution::min_steps(1), 0);
}
