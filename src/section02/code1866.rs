#![allow(dead_code)]

/*

// 1866. Number of Ways to Rearrange Sticks With K Sticks Visible
// https://leetcode.com/problems/number-of-ways-to-rearrange-sticks-with-k-sticks-visible/
// https://leetcode.cn/problems/number-of-ways-to-rearrange-sticks-with-k-sticks-visible/
//
// Hard
//
// There are n uniquely-sized sticks whose lengths are integers from 1 to n. You want to arrange the sticks such that exactly k sticks are visible from the left. A stick is visible from the left if there are no longer sticks to the left of it.

    For example, if the sticks are arranged [1,3,2,5,4], then the sticks with lengths 1, 3, and 5 are visible from the left.

Given n and k, return the number of such arrangements. Since the answer may be large, return it modulo 109 + 7.

Example 1:

Input: n = 3, k = 2
Output: 3
Explanation: [1,3,2], [2,3,1], and [2,1,3] are the only arrangements such that exactly 2 sticks are visible.
The visible sticks are underlined.

Example 2:

Input: n = 5, k = 5
Output: 1
Explanation: [1,2,3,4,5] is the only arrangement such that all 5 sticks are visible.
The visible sticks are underlined.

Example 3:

Input: n = 20, k = 11
Output: 647427950
Explanation: There are 647427950 (mod 109 + 7) ways to rearrange the sticks such that exactly 11 sticks are visible.

Constraints:

    1 <= n <= 1000
    1 <= k <= n
*/

struct Solution;

impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        let mut dp = vec![vec![0; k + 1]; n + 1];
        Self::_rearrange_sticks(&mut dp, n, k) as _
    }

    fn _rearrange_sticks(dp: &mut Vec<Vec<i64>>, n: usize, k: usize) -> i64 {
        let modu = 1_000_000_007;
        if n == k {
            return 1;
        }
        if k == 0 {
            return 0;
        }
        if dp[n][k] == 0 {
            let v1 = Self::_rearrange_sticks(dp, n - 1, k - 1);
            let v2 = Self::_rearrange_sticks(dp, n - 1, k) * (n - 1) as i64;
            dp[n][k] = (v1 + v2) % modu;
        }
        dp[n][k]
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (3, 2, 3),
        (5, 5, 1),
        (20, 11, 647427950),
    ];
    for (n, k, expected) in cases {
        assert_eq!(Solution::rearrange_sticks(n, k), expected);
    }
}
