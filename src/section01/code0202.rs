#![allow(dead_code)]

// 202. Happy Number
// https://leetcode.com/problems/happy-number/
// https://leetcode.cn/problems/happy-number/
//
// Write an algorithm to determine if a number is "happy".
//
// A happy number is a number defined by the following process:
//
// - Starting with any positive integer, replace the number by the sum of the squares of its digits.
// - Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
// - Those numbers for which this process ends in 1 are happy.
//
// Return true if n is a happy number, and false if not.
//

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn sum_of_squares(n: i32) -> i32 {
            let mut n = n;
            let mut sum = 0;
            while n > 0 {
                let digit = n % 10;
                sum += digit * digit;
                n /= 10;
            }
            sum
        }

        let mut n = n;
        let mut set = std::collections::HashSet::new();
        while n != 1 {
            if set.contains(&n) {
                return false;
            }
            set.insert(n);
            n = sum_of_squares(n);
        }
        true
    }
}

#[test]
fn test_is_happy() {
    assert_eq!(Solution::is_happy(19), true);
    assert_eq!(Solution::is_happy(2), false);
}
