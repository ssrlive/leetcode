#![allow(dead_code)]

// 633. Sum of Square Numbers
// https://leetcode.com/problems/sum-of-square-numbers/
//
// Given a non-negative integer c, decide whether there're two integers a and b such that a2 + b2 = c.
//
// Example 1:
//
// Input: c = 5
// Output: true
// Explanation: 1 * 1 + 2 * 2 = 5
//
// Example 2:
//
// Input: c = 3
// Output: false
//
// Constraints:
//
// 0 <= c <= 2^31 - 1
//

struct Solution;

// use std::cmp::Ordering;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        pub fn _judge_square_sum(c: i64) -> bool {
            let mut a = 0;
            let mut b = (c as f64).sqrt() as i64;

            while a <= b {
                match (a * a + b * b).cmp(&c) {
                    std::cmp::Ordering::Less => a += 1,
                    std::cmp::Ordering::Equal => return true,
                    std::cmp::Ordering::Greater => b -= 1,
                }
            }

            false
        }
        _judge_square_sum(c as i64)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::judge_square_sum(2147483600), true);
    assert_eq!(Solution::judge_square_sum(5), true);
    assert_eq!(Solution::judge_square_sum(3), false);
}
