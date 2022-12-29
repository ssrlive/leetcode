#![allow(dead_code)]

// 869. Reordered Power of 2
// https://leetcode.com/problems/reordered-power-of-2/
// https://leetcode.cn/problems/reordered-power-of-2/
//
// You are given an integer n. We reorder the digits in any order (including the original order) such that the leading digit is not zero.
//
// Return true if and only if we can do this so that the resulting number is a power of two.
//
// Example 1:
//
// Input: n = 1
// Output: true
//
// Example 2:
//
// Input: n = 10
// Output: false
//
// Constraints:
//
// - 1 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut dict = vec![0; 10];
        for c in n.to_string().chars() {
            dict[(c as u8 - b'0') as usize] += 1;
        }

        for i in 0..=30 {
            let v = 2usize.pow(i);
            let mut memo = vec![0; 10];
            for c in v.to_string().chars() {
                memo[(c as u8 - b'0') as usize] += 1;
            }

            let mut success = true;
            for i in 0..10 {
                if dict[i] != memo[i] {
                    success = false;
                    break;
                }
            }
            if success {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reordered_power_of2(1), true);
    assert_eq!(Solution::reordered_power_of2(10), false);
    assert_eq!(Solution::reordered_power_of2(16), true);
    assert_eq!(Solution::reordered_power_of2(24), false);
    assert_eq!(Solution::reordered_power_of2(46), true);
}
