#![allow(dead_code)]

// 557. Reverse Words in a String III
// https://leetcode.com/problems/reverse-words-in-a-string-iii/
// https://leetcode.cn/problems/reverse-words-in-a-string-iii/
//
// Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
//
// Example 1:
//
// Input: s = "Let's take LeetCode contest"
// Output: "s'teL ekat edoCteeL tsetnoc"
//
// Example 2:
//
// Input: s = "God Ding"
// Output: "doG gniD"
//
// Constraints:
//
// - 1 <= s.length <= 5 * 10^4
// - s contains printable ASCII characters.
// - s does not contain any leading or trailing spaces.
// - There is at least one word in s.
// - All the words in s are separated by a single space.
//
// Follow up: Could you solve it in-place with O(1) extra space?

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::new();
        let mut word = String::new();
        for c in s.chars() {
            if c == ' ' {
                result.push_str(&word);
                result.push(' ');
                word.clear();
            } else {
                word.insert(0, c);
            }
        }
        result.push_str(&word);
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reverse_words("Let's take LeetCode contest".to_string()),
        "s'teL ekat edoCteeL tsetnoc".to_string()
    );
    assert_eq!(Solution::reverse_words("God Ding".to_string()), "doG gniD".to_string());
}
