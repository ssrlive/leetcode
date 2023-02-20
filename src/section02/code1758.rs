#![allow(dead_code)]

/*

// 1758. Minimum Changes To Make Alternating Binary String
// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/
// https://leetcode.cn/problems/minimum-changes-to-make-alternating-binary-string/
//
// Easy
//
// You are given a string s consisting only of the characters '0' and '1'. In one operation, you can change any '0' to '1' or vice versa.

The string is called alternating if no two adjacent characters are equal. For example, the string "010" is alternating, while the string "0100" is not.

Return the minimum number of operations needed to make s alternating.

Example 1:

Input: s = "0100"
Output: 1
Explanation: If you change the last character to '1', s will be "0101", which is alternating.

Example 2:

Input: s = "10"
Output: 0
Explanation: s is already alternating.

Example 3:

Input: s = "1111"
Output: 2
Explanation: You need two operations to reach "0101" or "1010".

Constraints:

    1 <= s.length <= 10^4
    s[i] is either '0' or '1'.
*/

struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut cnt0 = 0;
        let mut cnt1 = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                if c == '1' {
                    cnt0 += 1;
                } else {
                    cnt1 += 1;
                }
            } else if c == '0' {
                cnt0 += 1;
            } else {
                cnt1 += 1;
            }
        }
        cnt0.min(cnt1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operations("0100".to_string()), 1);
    assert_eq!(Solution::min_operations("10".to_string()), 0);
    assert_eq!(Solution::min_operations("1111".to_string()), 2);
}
