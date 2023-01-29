#![allow(dead_code)]

// 1432. Max Difference You Can Get From Changing an Integer
// https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer/
// https://leetcode.cn/problems/max-difference-you-can-get-from-changing-an-integer/
//
// Medium
//
// You are given an integer num. You will apply the following steps exactly two times:
//
//     Pick a digit x (0 <= x <= 9).
//     Pick another digit y (0 <= y <= 9). The digit y can be equal to x.
//     Replace all the occurrences of x in the decimal representation of num by y.
//     The new integer cannot have any leading zeros, also the new integer cannot be 0.
//
// Let a and b be the results of applying the operations to num the first and second times, respectively.
//
// Return the max difference between a and b.
//
// Example 1:
//
// Input: num = 555
// Output: 888
// Explanation: The first time pick x = 5 and y = 9 and store the new integer in a.
// The second time pick x = 5 and y = 1 and store the new integer in b.
// We have now a = 999 and b = 111 and max difference = 888
//
// Example 2:
//
// Input: num = 9
// Output: 8
// Explanation: The first time pick x = 9 and y = 9 and store the new integer in a.
// The second time pick x = 9 and y = 1 and store the new integer in b.
// We have now a = 9 and b = 1 and max difference = 8
//
// Constraints:
//
// -    1 <= num <= 10^8
//

struct Solution;

impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        fn replace_digits(num: i64, x: i64, y: i64) -> i64 {
            if num == 0 {
                return 0;
            }
            let digit = num % 10;
            let digit = if digit == x { y } else { digit };
            replace_digits(num / 10, x, y) * 10 + digit
        }

        let mut num = num as i64;
        let mut a = num;
        let mut b = num;
        let mut digits = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        for &digit in digits.iter() {
            if digit != 9 {
                a = replace_digits(a, digit, 9);
                break;
            }
        }
        if digits[0] != 1 {
            b = replace_digits(b, digits[0], 1);
        } else {
            for &digit in digits.iter().skip(1) {
                if digit != 0 && digit != 1 {
                    b = replace_digits(b, digit, 0);
                    break;
                }
            }
        }
        (a - b) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_diff(555), 888);
    assert_eq!(Solution::max_diff(9), 8);
    assert_eq!(Solution::max_diff(123456), 820000);
}
