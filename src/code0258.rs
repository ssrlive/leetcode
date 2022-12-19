#![allow(dead_code)]

// 258. Add Digits
// https://leetcode.com/problems/add-digits/
// https://leetcode.cn/problems/add-digits/
//
// Given a non-negative integer num, repeatedly add all its digits until the
// result has only one digit.
//
// For example:
//
// Given num = 38, the process is like: 3 + 8 = 11, 1 + 1 = 2. Since 2 has only
// one digit, return it.
//

struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        let mut n = num;
        while n >= 10 {
            n = n / 10 + n % 10;
        }
        n
    }
}

#[test]
fn test() {
    assert_eq!(Solution::add_digits(0), 0);
    assert_eq!(Solution::add_digits(1), 1);
    assert_eq!(Solution::add_digits(9), 9);
    assert_eq!(Solution::add_digits(10), 1);
    assert_eq!(Solution::add_digits(11), 2);
    assert_eq!(Solution::add_digits(12), 3);
    assert_eq!(Solution::add_digits(13), 4);
    assert_eq!(Solution::add_digits(14), 5);
    assert_eq!(Solution::add_digits(15), 6);
    assert_eq!(Solution::add_digits(16), 7);
    assert_eq!(Solution::add_digits(17), 8);
    assert_eq!(Solution::add_digits(18), 9);
    assert_eq!(Solution::add_digits(19), 1);
    assert_eq!(Solution::add_digits(20), 2);
    assert_eq!(Solution::add_digits(21), 3);
    assert_eq!(Solution::add_digits(22), 4);
    assert_eq!(Solution::add_digits(23), 5);
    assert_eq!(Solution::add_digits(24), 6);
    assert_eq!(Solution::add_digits(38), 2);
}
