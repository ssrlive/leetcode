#![allow(dead_code)]

// 552. Student Attendance Record II
// https://leetcode.com/problems/student-attendance-record-ii/
// https://leetcode.cn/problems/student-attendance-record-ii/
//
// An attendance record for a student can be represented as a string where each character signifies whether the
// student was absent, late, or present on that day. The record only contains the following three characters:
//
// - 'A': Absent.
// - 'L': Late.
// - 'P': Present.
//
// Any student is eligible for an attendance award if they meet both of the following criteria:
//
// - The student was absent ('A') for strictly fewer than 2 days total.
// - The student was never late ('L') for 3 or more consecutive days.
//
// Given an integer n, return the number of possible attendance records of length n that make a student eligible
// for an attendance award. The answer may be very large, so return it modulo 109 + 7.
//
// Example 1:
//
// Input: n = 2
// Output: 8
// Explanation: There are 8 records with length 2 that are eligible for an award:
// "PP", "AP", "PA", "LP", "PL", "AL", "LA", "LL"
// Only "AA" is not eligible because there are 2 absences (there need to be fewer than 2).
//
// Example 2:
//
// Input: n = 1
// Output: 3
//
// Example 3:
//
// Input: n = 10101
// Output: 183236316
//
// Constraints:
//
// - 1 <= n <= 10^5
//

struct Solution;

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![vec![0_i64; 4]; 3]; n + 1];
        dp[0][0][0] = 1;
        for i in 0..n {
            for j in 0..2 {
                for k in 0..3 {
                    dp[i + 1][j][0] = (dp[i + 1][j][0] + dp[i][j][k]) % 1000000007;
                    dp[i + 1][j][k + 1] = (dp[i + 1][j][k + 1] + dp[i][j][k]) % 1000000007;
                    if j == 0 {
                        dp[i + 1][1][0] = (dp[i + 1][1][0] + dp[i][j][k]) % 1000000007;
                    }
                }
            }
        }
        let mut ans = 0;
        for j in 0..2 {
            for k in 0..3 {
                ans = (ans + dp[n][j][k]) % 1000000007;
            }
        }
        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_record(2), 8);
    assert_eq!(Solution::check_record(1), 3);
    assert_eq!(Solution::check_record(10101), 183236316);
    assert_eq!(Solution::check_record(99998), 852638723);
}
