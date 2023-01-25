#![allow(dead_code)]

// 1309. Decrypt String from Alphabet to Integer Mapping
// https://leetcode.com/problems/decrypt-string-from-alphabet-to-integer-mapping/
// https://leetcode.cn/problems/decrypt-string-from-alphabet-to-integer-mapping/
//
// Easy
//
// You are given a string s formed by digits and '#'. We want to map s to English lowercase characters as follows:
//
//     Characters ('a' to 'i') are represented by ('1' to '9') respectively.
//     Characters ('j' to 'z') are represented by ('10#' to '26#') respectively.
//
// Return the string formed after mapping.
//
// The test cases are generated so that a unique mapping will always exist.
//
// Example 1:
//
// Input: s = "10#11#12"
// Output: "jkab"
// Explanation: "j" -> "10#" , "k" -> "11#" , "a" -> "1" , "b" -> "2".
//
// Example 2:
//
// Input: s = "1326#"
// Output: "acz"
//
// Constraints:
//
// -    1 <= s.length <= 1000
// -    s consists of digits and the '#' letter.
// -    s will be a valid string such that mapping is always possible.
//

struct Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut v = vec![];
        let mut i = 0;
        while i < s.len() {
            if i + 2 < s.len() && s.as_bytes()[i + 2] == b'#' {
                v.push((s.as_bytes()[i] - b'0') * 10 + s.as_bytes()[i + 1] - b'0' + b'a' - 1);
                i += 3;
            } else {
                v.push(s.as_bytes()[i] + b'a' - b'1');
                i += 1;
            }
        }
        String::from_utf8(v).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::freq_alphabets("10#11#12".to_string()), "jkab".to_string());
    assert_eq!(Solution::freq_alphabets("1326#".to_string()), "acz".to_string());
    assert_eq!(Solution::freq_alphabets("25#".to_string()), "y".to_string());
    assert_eq!(Solution::freq_alphabets("1326#".to_string()), "acz".to_string());
    assert_eq!(Solution::freq_alphabets("25#".to_string()), "y".to_string());
    assert_eq!(Solution::freq_alphabets("1326#".to_string()), "acz".to_string());
}
