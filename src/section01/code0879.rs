#![allow(dead_code)]

// 879. Profitable Schemes
// https://leetcode.com/problems/profitable-schemes/
// https://leetcode.cn/problems/profitable-schemes/
//
// There is a group of n members, and a list of various crimes they could commit.
// The ith crime generates a profit[i] and requires group[i] members to participate in it.
// If a member participates in one crime, that member can't participate in another crime.
//
// Let's call a profitable scheme any subset of these crimes that generates at least minProfit profit,
// and the total number of members participating in that subset of crimes is at most n.
//
// Return the number of schemes that can be chosen. Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: n = 5, minProfit = 3, group = [2,2], profit = [2,3]
// Output: 2
// Explanation: To make a profit of at least 3, the group could either commit crimes 0 and 1, or just crime 1.
// In total, there are 2 schemes.
//
// Example 2:
//
// Input: n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
// Output: 7
// Explanation: To make a profit of at least 5, the group could commit any crimes, as long as they commit one.
// There are 7 possible schemes: (0), (1), (2), (0,1), (0,2), (1,2), and (0,1,2).
//
// Constraints:
//
// - 1 <= n <= 100
// - 0 <= minProfit <= 100
// - 1 <= group.length <= 100
// - 1 <= group[i] <= 100
// - profit.length == group.length
// - 0 <= profit[i] <= 100
//

struct Solution;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0_i128; 1 + n as usize]; 1 + min_profit as usize];
        dp[0][0] = 1;
        for (p, g) in profit.iter().zip(group.iter()) {
            for i in (0..=min_profit).rev() {
                for j in (0..=n - g).rev() {
                    dp[std::cmp::min(i + p, min_profit) as usize][(j + g) as usize] += dp[i as usize][j as usize];
                }
            }
        }
        (dp[min_profit as usize].iter().sum::<i128>() % 1_000_000_007) as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::profitable_schemes(5, 3, vec![2, 2], vec![2, 3]), 2);
    assert_eq!(Solution::profitable_schemes(10, 5, vec![2, 3, 5], vec![6, 7, 8]), 7);

    let group = vec![
        2, 5, 36, 2, 5, 5, 14, 1, 12, 1, 14, 15, 1, 1, 27, 13, 6, 59, 6, 1, 7, 1, 2, 7, 6, 1, 6, 1, 3, 1, 2, 11, 3, 39, 21, 20, 1, 27, 26,
        22, 11, 17, 3, 2, 4, 5, 6, 18, 4, 14, 1, 1, 1, 3, 12, 9, 7, 3, 16, 5, 1, 19, 4, 8, 6, 3, 2, 7, 3, 5, 12, 6, 15, 2, 11, 12, 12, 21,
        5, 1, 13, 2, 29, 38, 10, 17, 1, 14, 1, 62, 7, 1, 14, 6, 4, 16, 6, 4, 32, 48,
    ];
    let profit = vec![
        21, 4, 9, 12, 5, 8, 8, 5, 14, 18, 43, 24, 3, 0, 20, 9, 0, 24, 4, 0, 0, 0, 3, 13, 6, 5, 19, 6, 3, 14, 9, 5, 5, 6, 4, 7, 20, 2, 13,
        0, 1, 19, 4, 0, 11, 9, 6, 15, 15, 7, 1, 25, 17, 4, 4, 3, 43, 46, 82, 15, 12, 4, 1, 8, 24, 3, 15, 3, 6, 3, 0, 8, 10, 8, 10, 1, 21,
        13, 10, 28, 11, 27, 17, 1, 13, 10, 11, 4, 36, 26, 4, 2, 2, 2, 10, 0, 11, 5, 22, 6,
    ];
    assert_eq!(Solution::profitable_schemes(100, 100, group, profit), 539737848);
}
