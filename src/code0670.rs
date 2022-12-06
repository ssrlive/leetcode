#![allow(dead_code)]

// 670. Maximum Swap
// https://leetcode.com/problems/maximum-swap/
//
// You are given an integer num. You can swap two digits at most once to get the maximum valued number.
//
// Return the maximum valued number you can get.
//
// Example 1:
//
// Input: num = 2736
// Output: 7236
// Explanation: Swap the number 2 and the number 7.
//
// Example 2:
//
// Input: num = 9973
// Output: 9973
// Explanation: No swap.
//
// Constraints:
//
// - 0 <= num <= 10^8
//
// Follow up: Could you solve it in O(n) time complexity?

struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        fn check_max(digits: &[i32]) -> Option<(i32, usize)> {
            if digits.is_empty() {
                return None;
            }
            let mut max = *digits.last()?;
            let mut max_index = None;
            for (index, digit) in digits.iter().enumerate() {
                if *digit > max {
                    max = *digit;
                    max_index = Some(index);
                }
            }
            max_index.map(|max_index| (max, max_index))
        }

        let mut digits = Vec::new();
        let mut n = num;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        let mut result = 0;
        while check_max(&digits).is_none() && !digits.is_empty() {
            result = result * 10 + digits.pop().unwrap();
        }
        if let Some((_, max_index)) = check_max(&digits) {
            let len = digits.len();
            digits.swap(len - 1, max_index);
        }
        while let Some(n) = digits.pop() {
            result = result * 10 + n;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_swap(2736), 7236);
    assert_eq!(Solution::maximum_swap(9973), 9973);
    assert_eq!(Solution::maximum_swap(98368), 98863);
    assert_eq!(Solution::maximum_swap(1993), 9913);
}
