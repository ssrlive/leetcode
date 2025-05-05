#![allow(dead_code)]

// 3536. Maximum Product of Two Digits
// https://leetcode.com/problems/maximum-product-of-two-digits
// https://leetcode.cn/problems/maximum-product-of-two-digits
//
// Easy
//
// You are given a positive integer n.
//
// Return the maximum product of any two digits in n.
//
// Note: You may use the same digit twice if it appears more than once in n.
//
// Example 1:
//
// Input: n = 31
//
// Output: 3
//
// Explanation:
//
//     The digits of n are [3, 1].
//     The possible products of any two digits are: 3 * 1 = 3.
//     The maximum product is 3.
//
// Example 2:
//
// Input: n = 22
//
// Output: 4
//
// Explanation:
//
//     The digits of n are [2, 2].
//     The possible products of any two digits are: 2 * 2 = 4.
//     The maximum product is 4.
//
// Example 3:
//
// Input: n = 124
//
// Output: 8
//
// Explanation:
//
//     The digits of n are [1, 2, 4].
//     The possible products of any two digits are: 1 * 2 = 2, 1 * 4 = 4, 2 * 4 = 8.
//     The maximum product is 8.
//
// Constraints:
//
//     10 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        use std::collections::BinaryHeap;
        use std::iter::successors;

        let mut digits = successors(Some(n), |&x| (x >= 10).then_some(x / 10))
            .map(|x| x % 10)
            .collect::<BinaryHeap<_>>();
        let (Some(a), Some(b)) = (digits.pop(), digits.pop()) else {
            panic!()
        };
        a * b
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_product(31), 3);
    assert_eq!(Solution::max_product(22), 4);
    assert_eq!(Solution::max_product(124), 8);
}
