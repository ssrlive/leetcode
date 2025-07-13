#![allow(dead_code)]

// 3612. Process String with Special Operations I
// https://leetcode.com/problems/process-string-with-special-operations-i/
// https://leetcode.cn/problems/process-string-with-special-operations-i/
//
// Medium
//
// You are given a string s consisting of lowercase English letters and the special characters: *, #, and %.
//
// Build a new string result by processing s according to the following rules from left to right:
//
//     If the letter is a lowercase English letter append it to result.
//     A '*' removes the last character from result, if it exists.
//     A '#' duplicates the current result and appends it to itself.
//     A '%' reverses the current result.
//
// Return the final string result after processing all characters in s.
//
// Example 1:
//
// Input: s = "a#b%*"
//
// Output: "ba"
//
// Explanation:
// i	s[i]	Operation	Current result
// 0	'a'	Append 'a'	"a"
// 1	'#'	Duplicate result	"aa"
// 2	'b'	Append 'b'	"aab"
// 3	'%'	Reverse result	"baa"
// 4	'*'	Remove the last character	"ba"
//
// Thus, the final result is "ba".
//
// Example 2:
//
// Input: s = "z*#"
//
// Output: ""
//
// Explanation:
// i	s[i]	Operation	Current result
// 0	'z'	Append 'z'	"z"
// 1	'*'	Remove the last character	""
// 2	'#'	Duplicate the string	""
//
// Thus, the final result is "".
//
// Constraints:
//
//     1 <= s.length <= 20
//     s consists of only lowercase English letters and special characters *, #, and %.
//

struct Solution;

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            match c {
                '*' => {
                    res.pop();
                }
                '#' => {
                    res = res.clone() + &res;
                }
                '%' => {
                    res = res.chars().rev().collect();
                }
                _ => {
                    res.push(c);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::process_str("a#b%*".into()), "ba");
    assert_eq!(Solution::process_str("z*#".into()), "");
}
