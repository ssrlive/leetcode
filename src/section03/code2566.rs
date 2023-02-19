#![allow(dead_code)]

/*

// 2566. Maximum Difference by Remapping a Digit
// https://leetcode.com/problems/maximum-difference-by-remapping-a-digit/
// https://leetcode.cn/problems/maximum-difference-by-remapping-a-digit/
//
// Easy
//
// You are given an integer num. You know that Danny Mittal will sneakily remap one of the 10 possible digits (0 to 9) to another digit.

Return the difference between the maximum and minimum values Danny can make by remapping exactly one digit in num.

Notes:

    When Danny remaps a digit d1 to another digit d2, Danny replaces all occurrences of d1 in num with d2.
    Danny can remap a digit to itself, in which case num does not change.
    Danny can remap different digits for obtaining minimum and maximum values respectively.
    The resulting number after remapping can contain leading zeroes.
    We mentioned "Danny Mittal" to congratulate him on being in the top 10 in Weekly Contest 326.

Example 1:

Input: num = 11891
Output: 99009
Explanation:
To achieve the maximum value, Danny can remap the digit 1 to the digit 9 to yield 99899.
To achieve the minimum value, Danny can remap the digit 1 to the digit 0, yielding 890.
The difference between these two numbers is 99009.

Example 2:

Input: num = 90
Output: 99
Explanation:
The maximum value that can be returned by the function is 99 (if 0 is replaced by 9) and the minimum value that can be returned by the function is 0 (if 9 is replaced by 0).
Thus, we return 99.

Constraints:

    1 <= num <= 10^8
*/

struct Solution;

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let s = num.to_string().as_bytes().to_vec();
        let mut large = s.clone();
        let mut small = s.clone();
        let mut dl = -1;
        let mut ds = -1;
        for (i, &c) in s.iter().enumerate() {
            if dl == -1 && c != b'9' {
                dl = c as i32;
                large[i] = b'9';
            } else {
                let v = if c as i32 == dl { b'9' } else { c };
                large[i] = v;
            }

            if ds == -1 && c != b'0' {
                ds = c as i32;
                small[i] = b'0';
            } else {
                let v = if c as i32 == ds { b'0' } else { c };
                small[i] = v;
            }
        }
        let l = unsafe { std::str::from_utf8_unchecked(&large) }.parse::<i32>().unwrap();
        let s = unsafe { std::str::from_utf8_unchecked(&small) }.parse::<i32>().unwrap();
        l - s
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_max_difference(11891), 99009);
    assert_eq!(Solution::min_max_difference(90), 99);
    assert_eq!(Solution::min_max_difference(10), 90);
    assert_eq!(Solution::min_max_difference(1), 9);
    assert_eq!(Solution::min_max_difference(0), 9);
    assert_eq!(Solution::min_max_difference(100000000), 900000000);
}
