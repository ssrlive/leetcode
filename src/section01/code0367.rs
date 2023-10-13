#![allow(dead_code)]

// 367. Valid Perfect Square
// https://leetcode.com/problems/valid-perfect-square/
// https://leetcode.cn/problems/valid-perfect-square/
//
// Given a positive integer num, write a function which returns True if num is a perfect square else False.
//
// Follow up: Do not use any built-in library function such as sqrt.
//
// Example 1:
//
// Input: num = 16
// Output: true
//
// Example 2:
//
// Input: num = 14
// Output: false
//
// Constraints:
//
// 1 <= num <= 2^31 - 1
//

struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        fn _is_perfect_square(num: i64) -> bool {
            let mut left = 1;
            let mut right = num;
            while left <= right {
                let mid = left + (right - left) / 2;
                let square = mid * mid;
                match square.cmp(&num) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Equal => return true,
                    std::cmp::Ordering::Greater => right = mid - 1,
                }
            }
            false
        }
        _is_perfect_square(num as i64)
    }
}

#[test]
fn test() {
    assert!(Solution::is_perfect_square(1));
    assert!(!Solution::is_perfect_square(2));
    assert!(!Solution::is_perfect_square(3));
    assert!(Solution::is_perfect_square(4));
    assert!(!Solution::is_perfect_square(5));
    assert!(!Solution::is_perfect_square(6));
    assert!(!Solution::is_perfect_square(7));
    assert!(!Solution::is_perfect_square(8));
    assert!(Solution::is_perfect_square(9));
    assert!(!Solution::is_perfect_square(10));
    assert!(!Solution::is_perfect_square(11));
    assert!(Solution::is_perfect_square(16));
    assert!(!Solution::is_perfect_square(17));
    assert!(Solution::is_perfect_square(25));
    assert!(!Solution::is_perfect_square(26));
    assert!(!Solution::is_perfect_square(27));
    assert!(!Solution::is_perfect_square(28));
    assert!(Solution::is_perfect_square(36));
}
