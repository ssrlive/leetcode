#![allow(dead_code)]

// 2443. Sum of Number and Its Reverse
// https://leetcode.com/problems/sum-of-number-and-its-reverse/
// https://leetcode.cn/problems/sum-of-number-and-its-reverse/
//
// Given a non-negative integer num, return true if num can be expressed as the sum of any non-negative integer and its reverse, or false otherwise.
//
// Example 1:
//
// Input: num = 443
// Output: true
// Explanation: 172 + 271 = 443 so we return true.
//
// Example 2:
//
// Input: num = 63
// Output: false
// Explanation: 63 cannot be expressed as the sum of a non-negative integer and its reverse so we return false.
//
// Example 3:
//
// Input: num = 181
// Output: true
// Explanation: 140 + 041 = 181 so we return true. Note that when a number is reversed, there may be leading zeros.
//
// Constraints:
//
// - 0 <= num <= 10^5
//

struct Solution;

impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        fn reverse_num(num: u32) -> u32 {
            num.to_string().chars().rev().collect::<String>().parse().unwrap()
        }

        if num == 0 {
            return true;
        }
        let num = num as u32;

        let upper_bound = if num > 100 { num - 9 } else { num };
        for i in 0..upper_bound {
            let i_r = reverse_num(i);
            if i_r + i == num {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::sum_of_number_and_reverse(443));
    assert!(!Solution::sum_of_number_and_reverse(63));
    assert!(Solution::sum_of_number_and_reverse(181));
    assert!(Solution::sum_of_number_and_reverse(0));
    assert!(!Solution::sum_of_number_and_reverse(1));
}
