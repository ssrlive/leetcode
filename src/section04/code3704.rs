#![allow(dead_code)]

// 3704. Count No-Zero Pairs That Sum to N
// https://leetcode.com/problems/count-no-zero-pairs-that-sum-to-n/
// https://leetcode.cn/problems/count-no-zero-pairs-that-sum-to-n/
//
// Hard
//
// A no-zero integer is a positive integer that does not contain the digit 0 in its decimal representation.
//
// Given an integer n, count the number of pairs (a, b) where:
//
// a and b are no-zero integers.
// a + b = n
// Return an integer denoting the number of such pairs.
//
// Example 1:
//
// Input: n = 2
//
// Output: 1
//
// Explanation:
//
// The only pair is (1, 1).
//
// Example 2:
//
// Input: n = 3
//
// Output: 2
//
// Explanation:
//
// The pairs are (1, 2) and (2, 1).
//
// Example 3:
//
// Input: n = 11
//
// Output: 8
//
// Explanation:
//
// The pairs are (2, 9), (3, 8), (4, 7), (5, 6), (6, 5), (7, 4), (8, 3), and (9, 2).
// Note that (1, 10) and (10, 1) do not satisfy the conditions because 10 contains 0 in its decimal representation.
//
// Constraints:
//
// 2 <= n <= 10^15
//

struct Solution;

impl Solution {
    pub fn count_no_zero_pairs(n: i64) -> i64 {
        fn solve(s: &[u8], ind: usize, carry: usize, za: bool, zb: bool, dp: &mut [[[[Option<i64>; 2]; 2]; 20]; 20]) -> i64 {
            if ind == s.len() {
                return if carry == 0 { 1 } else { 0 };
            }
            if let Some(v) = dp[ind][carry][za as usize][zb as usize] {
                return v;
            }
            let eda = if za { 0 } else { 9 };
            let edb = if zb { 0 } else { 9 };
            let sa = if ind == 0 { 1 } else { 0 };
            let sb = if ind == 0 { 1 } else { 0 };
            let num = (s[ind] - b'0') as usize;
            let mut cnt = 0;
            for i in sa..=eda {
                for j in sb..=edb {
                    let x = i + j + carry;
                    if x % 10 == num {
                        cnt += solve(s, ind + 1, x / 10, za || i == 0, zb || j == 0, dp);
                    }
                }
            }
            dp[ind][carry][za as usize][zb as usize] = Some(cnt);
            cnt
        }

        let mut s = n.to_string().into_bytes();
        s.reverse();
        solve(&s, 0, 0, false, false, &mut [[[[None; 2]; 2]; 20]; 20])
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_no_zero_pairs(2), 1);
    assert_eq!(Solution::count_no_zero_pairs(3), 2);
    assert_eq!(Solution::count_no_zero_pairs(11), 8);

    assert_eq!(Solution::count_no_zero_pairs(10), 9);
}
