#![allow(dead_code)]

// 3116. Kth Smallest Amount With Single Denomination Combination
// https://leetcode.com/problems/kth-smallest-amount-with-single-denomination-combination/
// https://leetcode.cn/problems/kth-smallest-amount-with-single-denomination-combination/
//
// Hard
//
// You are given an integer array coins representing coins of different denominations and an integer k.
//
// You have an infinite number of coins of each denomination. However, you are not allowed to combine coins of different denominations.
//
// Return the kth smallest amount that can be made using these coins.
//
// Example 1:
//
// Input: coins = [3,6,9], k = 3
//
// Output: 9
//
// Explanation: The given coins can make the following amounts:
// Coin 3 produces multiples of 3: 3, 6, 9, 12, 15, etc.
// Coin 6 produces multiples of 6: 6, 12, 18, 24, etc.
// Coin 9 produces multiples of 9: 9, 18, 27, 36, etc.
// All of the coins combined produce: 3, 6, 9, 12, 15, etc.
//
// Example 2:
//
// Input: coins = [5,2], k = 7
//
// Output: 12
//
// Explanation: The given coins can make the following amounts:
// Coin 5 produces multiples of 5: 5, 10, 15, 20, etc.
// Coin 2 produces multiples of 2: 2, 4, 6, 8, 10, 12, etc.
// All of the coins combined produce: 2, 4, 5, 6, 8, 10, 12, 14, 15, etc.
//
// Constraints:
//
//     1 <= coins.length <= 15
//     1 <= coins[i] <= 25
//     1 <= k <= 2 * 10^9
//     coins contains pairwise distinct integers.
//

struct Solution;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        fn gcd(mut a: i64, mut b: i64) -> i64 {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }

        let n = coins.len();
        let mut mult = vec![1i64; 1 << n];
        for (b, mult_b) in mult.iter_mut().enumerate().take(1 << n) {
            for (i, &coins_i) in coins.iter().enumerate().take(n) {
                if b & (1 << i) != 0 {
                    *mult_b = *mult_b / gcd(*mult_b, coins_i as i64) * coins_i as i64;
                }
            }
        }
        let mut lo: i64 = 1;
        let mut hi: i64 = 1e12 as i64;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut cnt = 0;
            for (b, &mult_b) in mult.iter().enumerate().take(1 << n).skip(1) {
                let v = mid / mult_b;
                if b.count_ones() & 1 == 1 {
                    cnt += v;
                } else {
                    cnt -= v;
                }
            }
            if cnt < k as i64 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 3), 9);
    assert_eq!(Solution::find_kth_smallest(vec![5, 2], 7), 12);
}
