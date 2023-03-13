#![allow(dead_code)]

// 935. Knight Dialer
// https://leetcode.com/problems/knight-dialer/
// https://leetcode.cn/problems/knight-dialer/
//
// The chess knight has a unique movement, it may move two squares vertically and one square horizontally,
// or two squares horizontally and one square vertically (with both forming the shape of an L).
// The possible movements of chess knight are shown in this diagaram:
//
// A chess knight can move as indicated in the chess diagram below:
//
// We have a chess knight and a phone pad as shown below, the knight can only stand on a numeric cell (i.e. blue cell).
//
// Given an integer n, return how many distinct phone numbers of length n we can dial.
//
// You are allowed to place the knight on any numeric cell initially and then you should perform n - 1 jumps
// to dial a number of length n. All jumps should be valid knight jumps.
//
// As the answer may be very large, return the answer modulo 109 + 7.
//
// Example 1:
//
// Input: n = 1
// Output: 10
// Explanation: We need to dial a number of length 1, so placing the knight over any numeric cell of the 10 cells is sufficient.
//
// Example 2:
//
// Input: n = 2
// Output: 20
// Explanation: All the valid number we can dial are [04, 06, 16, 18, 27, 29, 34, 38, 40, 43, 49, 60, 61, 67, 72, 76, 81, 83, 92, 94]
//
// Example 3:
//
// Input: n = 3131
// Output: 136006598
// Explanation: Please take care of the mod.
//
// Constraints:
//
// - 1 <= n <= 5000
//

struct Solution;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut curr = vec![1; 10];
        let mut next = curr.clone();
        for _ in 2..=n {
            next[0] = (curr[4] % MOD + curr[6] % MOD) % MOD;
            next[1] = (curr[8] % MOD + curr[6] % MOD) % MOD;
            next[2] = (curr[9] % MOD + curr[7] % MOD) % MOD;
            next[3] = (curr[4] % MOD + curr[8] % MOD) % MOD;
            next[4] = (curr[3] % MOD + curr[9] % MOD + curr[0] % MOD) % MOD;
            next[5] = 0;
            next[6] = (curr[1] % MOD + curr[7] % MOD + curr[0] % MOD) % MOD;
            next[7] = (curr[2] % MOD + curr[6] % MOD) % MOD;
            next[8] = (curr[1] % MOD + curr[3] % MOD) % MOD;
            next[9] = (curr[4] % MOD + curr[2] % MOD) % MOD;
            curr = next.clone();
        }
        let mut ans = 0;
        for &item in next.iter() {
            ans = (ans + item) % MOD;
        }
        ans as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::knight_dialer(1), 10);
    assert_eq!(Solution::knight_dialer(2), 20);
    assert_eq!(Solution::knight_dialer(3131), 136_006_598);
}
