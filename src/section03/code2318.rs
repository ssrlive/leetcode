#![allow(dead_code)]

/*

// 2318. Number of Distinct Roll Sequences
// https://leetcode.com/problems/number-of-distinct-roll-sequences/
// https://leetcode.cn/problems/number-of-distinct-roll-sequences/
//
// Hard
//
// You are given an integer n. You roll a fair 6-sided dice n times.
// Determine the total number of distinct sequences of rolls possible such that
// the following conditions are satisfied:

    The greatest common divisor of any adjacent values in the sequence is equal to 1.
    There is at least a gap of 2 rolls between equal valued rolls. More formally, if the value of the ith roll is equal to the value of the jth roll, then abs(i - j) > 2.

Return the total number of distinct sequences possible. Since the answer may be very large, return it modulo 109 + 7.

Two sequences are considered distinct if at least one element is different.

Example 1:

Input: n = 4
Output: 184
Explanation: Some of the possible sequences are (1, 2, 3, 4), (6, 1, 2, 3), (1, 2, 3, 1), etc.
Some invalid sequences are (1, 2, 1, 3), (1, 2, 3, 6).
(1, 2, 1, 3) is invalid since the first and third roll have an equal value and abs(1 - 3) = 2 (i and j are 1-indexed).
(1, 2, 3, 6) is invalid since the greatest common divisor of 3 and 6 = 3.
There are a total of 184 distinct sequences possible, so we return 184.

Example 2:

Input: n = 2
Output: 22
Explanation: Some of the possible sequences are (1, 2), (2, 1), (3, 2).
Some invalid sequences are (3, 6), (2, 4) since the greatest common divisor is not equal to 1.
There are a total of 22 distinct sequences possible, so we return 22.

Constraints:

    1 <= n <= 10^4
*/

struct Solution;

impl Solution {
    pub fn distinct_sequences(n: i32) -> i32 {
        fn _distinct_sequences(dp: &mut Vec<Vec<Vec<i32>>>, n: usize, p: usize, pp: usize) -> i32 {
            if n == 0 {
                return 1;
            }
            if dp[n][p][pp] == 0 {
                for d in 1..7 {
                    if d != p && d != pp && (p == 0 || gcd(d, p) == 1) {
                        dp[n][p][pp] = (dp[n][p][pp] + _distinct_sequences(dp, n - 1, d, p)) % 1000000007;
                    }
                }
            }
            dp[n][p][pp]
        }

        fn gcd(a: usize, b: usize) -> usize {
            if a == 0 {
                return b;
            }
            gcd(b % a, a)
        }

        let n = n as usize;
        let mut dp = vec![vec![vec![0; 7]; 7]; n + 1];
        _distinct_sequences(&mut dp, n, 0, 0)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::distinct_sequences(4), 184);
    assert_eq!(Solution::distinct_sequences(2), 22);
    assert_eq!(Solution::distinct_sequences(1), 6);
}
