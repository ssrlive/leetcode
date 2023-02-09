#![allow(dead_code)]

/*

// 1525. Number of Good Ways to Split a String
// https://leetcode.com/problems/number-of-good-ways-to-split-a-string/
// https://leetcode.cn/problems/number-of-good-ways-to-split-a-string/
//
// Medium
//
// You are given a string s.
//
// A split is called good if you can split s into two non-empty strings sleft and sright where their
// concatenation is equal to s (i.e., sleft + sright = s) and the number of distinct letters
// in sleft and sright is the same.

Return the number of good splits you can make in s.

Example 1:

Input: s = "aacaba"
Output: 2
Explanation: There are 5 ways to split "aacaba" and 2 of them are good.
("a", "acaba") Left string and right string contains 1 and 3 different letters respectively.
("aa", "caba") Left string and right string contains 1 and 3 different letters respectively.
("aac", "aba") Left string and right string contains 2 and 2 different letters respectively (good split).
("aaca", "ba") Left string and right string contains 2 and 2 different letters respectively (good split).
("aacab", "a") Left string and right string contains 3 and 1 different letters respectively.

Example 2:

Input: s = "abcd"
Output: 1
Explanation: Split the string as follows ("ab", "cd").

Constraints:

    1 <= s.length <= 10^5
    s consists of only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut left = [0; 26];
        let mut right = [0; 26];
        let mut left_count = 0;
        let mut right_count = 0;
        let mut result = 0;
        for c in s.chars() {
            let i = (c as u8 - b'a') as usize;
            if right[i] == 0 {
                right_count += 1;
            }
            right[i] += 1;
        }
        for c in s.chars() {
            let i = (c as u8 - b'a') as usize;
            if left[i] == 0 {
                left_count += 1;
            }
            left[i] += 1;
            right[i] -= 1;
            if right[i] == 0 {
                right_count -= 1;
            }
            if left_count == right_count {
                result += 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![("aacaba", 2), ("abcd", 1), ("aaaaa", 4), ("acbadbaada", 2)];
    for (s, expect) in cases {
        assert_eq!(Solution::num_splits(s.to_string()), expect);
    }
}
