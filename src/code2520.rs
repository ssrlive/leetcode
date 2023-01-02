#![allow(dead_code)]

// 2520. Count the Digits That Divide a Number
// https://leetcode.com/problems/count-the-digits-that-divide-a-number/
// https://leetcode.cn/problems/count-the-digits-that-divide-a-number/
//
// Given an integer num, return the number of digits in num that divide num.
//
// An integer val divides nums if nums % val == 0.
//
// Example 1:
//
// Input: num = 7
// Output: 1
// Explanation: 7 divides itself, hence the answer is 1.
//
// Example 2:
//
// Input: num = 121
// Output: 2
// Explanation: 121 is divisible by 1, but not 2. Since 1 occurs twice as a digit, we return 2.
//
// Example 3:
//
// Input: num = 1248
// Output: 4
// Explanation: 1248 is divisible by all of its digits, hence the answer is 4.
//
// Constraints:
//
// - 1 <= num <= 10^9
// - num does not contain 0 as one of its digits.
//

struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut count = 0;
        let new = num.to_string();
        for i in 0..new.len() {
            if num % new[i..i + 1].parse::<i32>().unwrap() == 0 {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_digits(7), 1);
    assert_eq!(Solution::count_digits(121), 2);
    assert_eq!(Solution::count_digits(1248), 4);
}
