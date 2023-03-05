#![allow(dead_code)]

/*

// 2180. Count Integers With Even Digit Sum
// https://leetcode.com/problems/count-integers-with-even-digit-sum/
// https://leetcode.cn/problems/count-integers-with-even-digit-sum/
//
// Easy
//
// Given a positive integer num, return the number of positive integers less than or equal to num whose digit sums are even.

The digit sum of a positive integer is the sum of all its digits.

Example 1:

Input: num = 4
Output: 2
Explanation:
The only integers less than or equal to 4 whose digit sums are even are 2 and 4.

Example 2:

Input: num = 30
Output: 14
Explanation:
The 14 integers less than or equal to 30 whose digit sums are even are
2, 4, 6, 8, 11, 13, 15, 17, 19, 20, 22, 24, 26, and 28.

Constraints:

    1 <= num <= 1000
*/

struct Solution;

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let sum_dig_is_even = |mut x: i32| -> bool {
            let mut s = 0;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            s % 2 == 0
        };
        (1..=num).filter(|&x| sum_dig_is_even(x)).count() as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_even(4), 2);
    assert_eq!(Solution::count_even(30), 14);
}
