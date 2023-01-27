#![allow(dead_code)]

// 1359. Count All Valid Pickup and Delivery Options
// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/
// https://leetcode.cn/problems/count-all-valid-pickup-and-delivery-options/
//
// Hard
//
// Given n orders, each order consist in pickup and delivery services.
//
// Count all valid pickup/delivery possible sequences such that delivery(i) is always after of pickup(i).
//
// Since the answer may be too large, return it modulo 10^9 + 7.
//
// Example 1:
//
// Input: n = 1
// Output: 1
// Explanation: Unique order (P1, D1), Delivery 1 always is after of Pickup 1.
//
// Example 2:
//
// Input: n = 2
// Output: 6
// Explanation: All possible orders:
// (P1,P2,D1,D2), (P1,P2,D2,D1), (P1,D1,P2,D2), (P2,P1,D1,D2), (P2,P1,D2,D1) and (P2,D2,P1,D1).
// This is an invalid order (P1,D2,P2,D1) because Pickup 2 is after of Delivery 2.
//
// Example 3:
//
// Input: n = 3
// Output: 90
//
// Constraints:
//
// -    1 <= n <= 500
//

struct Solution;

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut last = 1;
        for i in 2..=(n as u64) {
            let gaps: u64 = 2 * i - 1;
            last *= (gaps + 1) * gaps / 2;
            last %= 1_000_000_007_u64;
        }
        last as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_orders(1), 1);
    assert_eq!(Solution::count_orders(2), 6);
    assert_eq!(Solution::count_orders(3), 90);
}
