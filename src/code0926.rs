#![allow(dead_code)]

// 926. Flip String to Monotone Increasing
// https://leetcode.com/problems/flip-string-to-monotone-increasing/
// https://leetcode.cn/problems/flip-string-to-monotone-increasing/
//
// A binary string is monotone increasing if it consists of some number of 0's (possibly none), followed by some number of 1's (also possibly none).
//
// You are given a binary string s. You can flip s[i] changing it from 0 to 1 or from 1 to 0.
//
// Return the minimum number of flips to make s monotone increasing.
//
// Example 1:
//
// Input: s = "00110"
// Output: 1
// Explanation: We flip the last digit to get 00111.
//
// Example 2:
//
// Input: s = "010110"
// Output: 2
// Explanation: We flip to get 011111, or alternatively 000111.
//
// Example 3:
//
// Input: s = "00011000"
// Output: 2
// Explanation: We flip to get 00000000.
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s[i] is either '0' or '1'.
//

struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        s.chars()
            .fold((0, 0), |(ones, x), c| match c {
                '0' => (ones, ones.min(x + 1)),
                _ => (ones + 1, x),
            })
            .1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_flips_mono_incr("00110".to_string()), 1);
    assert_eq!(Solution::min_flips_mono_incr("010110".to_string()), 2);
    assert_eq!(Solution::min_flips_mono_incr("00011000".to_string()), 2);
}
