#![allow(dead_code)]

// 69. Sqrt(x)
// https://leetcode.com/problems/sqrtx/
// https://leetcode.cn/problems/sqrtx/
//
// Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
//
// You must not use any built-in exponent function or operator.
//
// For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
//
// Example 1:
//
// Input: x = 4
// Output: 2
// Explanation: The square root of 4 is 2, so we return 2.
//
// Example 2:
//
// Input: x = 8
// Output: 2
// Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
//
// Constraints:
//
// - 0 <= x <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        use std::cmp::Ordering;
        if x <= 1 {
            return x;
        }

        let (mut low, mut high) = (0, x / 2 + 1);
        let mut result = 0;

        while low <= high {
            let mid = low + (high - low) / 2;
            match mid.cmp(&(x / mid)) {
                Ordering::Equal => return mid,
                Ordering::Greater => high = mid - 1,
                Ordering::Less => {
                    low = mid + 1;
                    result = mid
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![(4, 2), (8, 2), (9, 3), (10, 3), (2147395599, 46339)];
    for (x, expected) in cases {
        assert_eq!(Solution::my_sqrt(x), expected);
    }
}
