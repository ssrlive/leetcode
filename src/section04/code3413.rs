#![allow(dead_code)]

// 3413. Maximum Coins From K Consecutive Bags
// https://leetcode.com/problems/maximum-coins-from-k-consecutive-bags/
// https://leetcode.cn/problems/maximum-coins-from-k-consecutive-bags/
//
// Medium
//
// There are an infinite amount of bags on a number line, one bag for each coordinate. Some of these bags contain coins.
//
// You are given a 2D array coins, where coins[i] = [li, ri, ci] denotes that every bag from li to ri contains ci coins.
//
// The segments that coins contain are non-overlapping.
//
// You are also given an integer k.
//
// Return the maximum amount of coins you can obtain by collecting k consecutive bags.
//
// Example 1:
//
// Input: coins = [[8,10,1],[1,3,2],[5,6,4]], k = 4
//
// Output: 10
//
// Explanation:
//
// Selecting bags at positions [3, 4, 5, 6] gives the maximum number of coins: 2 + 0 + 4 + 4 = 10.
//
// Example 2:
//
// Input: coins = [[1,10,3]], k = 2
//
// Output: 6
//
// Explanation:
//
// Selecting bags at positions [1, 2] gives the maximum number of coins: 3 + 3 = 6.
//
// Constraints:
//
//     1 <= coins.length <= 10^5
//     1 <= k <= 10^9
//     coins[i] == [li, ri, ci]
//     1 <= li <= ri <= 10^9
//     1 <= ci <= 1000
//     The given segments are non-overlapping.
//

struct Solution;

impl Solution {
    pub fn maximum_coins(coins: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut coins = coins
            .iter()
            .map(|v| v.iter().map(|&x| x as i64).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();
        coins.sort();
        let n = coins.len();
        let k = k as i64;
        let mut res = 0;
        let mut cur = 0;
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while j < n && coins[j][1] < coins[i][0] + k {
                cur += (coins[j][1] - coins[j][0] + 1) * coins[j][2];
                j += 1;
            }
            if j < n {
                let part = 0.max(coins[i][0] + k - 1 - coins[j][0] + 1) * coins[j][2];
                res = res.max(cur + part);
            }
            cur -= (coins[i][1] - coins[i][0] + 1) * coins[i][2];
            i += 1;
        }

        cur = 0;
        i = 0;
        j = 0;
        while i < n {
            cur += (coins[i][1] - coins[i][0] + 1) * coins[i][2];
            while coins[j][1] < coins[i][1] - k + 1 {
                cur -= (coins[j][1] - coins[j][0] + 1) * coins[j][2];
                j += 1;
            }
            let part = 0.max(coins[i][1] - k - coins[j][0] + 1) * coins[j][2];
            res = res.max(cur - part);
            i += 1;
        }

        res
    }
}

#[test]
fn test() {
    let coins = vec![vec![8, 10, 1], vec![1, 3, 2], vec![5, 6, 4]];
    let k = 4;
    let res = 10;
    assert_eq!(Solution::maximum_coins(coins, k), res);

    let coins = vec![vec![1, 10, 3]];
    let k = 2;
    let res = 6;
    assert_eq!(Solution::maximum_coins(coins, k), res);
}
