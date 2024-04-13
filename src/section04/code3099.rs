#![allow(dead_code)]

// 3099. Harshad Number
// https://leetcode.com/problems/harshad-number/
// https://leetcode.cn/problems/harshad-number/
//
// Easy
//
// An integer divisible by the sum of its digits is said to be a Harshad number. You are given an integer x.
// Return the sum of the digits of x if x is a Harshad number, otherwise, return -1.
//
// Example 1:
//
// Input: x = 18
//
// Output: 9
//
// Explanation:
//
// The sum of digits of x is 9. 18 is divisible by 9. So 18 is a Harshad number and the answer is 9.
//
// Example 2:
//
// Input: x = 23
//
// Output: -1
//
// Explanation:
//
// The sum of digits of x is 5. 23 is not divisible by 5. So 23 is not a Harshad number and the answer is -1.
//
// Constraints:
//
//     1 <= x <= 100
//

struct Solution;

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut s: i32 = 0;
        let mut temp: i32 = x;

        while temp > 0 {
            s += temp % 10;
            temp /= 10;
        }

        if x % s == 0 {
            s
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
    assert_eq!(Solution::sum_of_the_digits_of_harshad_number(23), -1);
    assert_eq!(Solution::sum_of_the_digits_of_harshad_number(1), 1);
}
