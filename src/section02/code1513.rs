#![allow(dead_code)]

/*

// 1513. Number of Substrings With Only 1s
// https://leetcode.com/problems/number-of-substrings-with-only-1s/
// https://leetcode.cn/problems/number-of-substrings-with-only-1s/
//
// Medium
//
// Given a binary string s, return the number of substrings with all characters 1's. Since the answer may be too large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: s = "0110111"
Output: 9
Explanation: There are 9 substring in total with only 1's characters.
"1" -> 5 times.
"11" -> 3 times.
"111" -> 1 time.

Example 2:

Input: s = "101"
Output: 2
Explanation: Substring "1" is shown 2 times in s.

Example 3:

Input: s = "111111"
Output: 21
Explanation: Each substring contains only 1's characters.

Constraints:

    1 <= s.length <= 10^5
    s[i] is either '0' or '1'.
*/

struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut count = 0;
        let mut sum = 0_i64;
        for c in s.chars() {
            if c == '1' {
                count += 1;
            } else {
                sum += count * (count + 1) / 2;
                count = 0;
            }
        }
        sum += count * (count + 1) / 2;
        (sum % 1_000_000_007) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_sub("0110111".to_string()), 9);
    assert_eq!(Solution::num_sub("101".to_string()), 2);
    assert_eq!(Solution::num_sub("111111".to_string()), 21);
}
