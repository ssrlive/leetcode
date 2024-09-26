#![allow(dead_code)]

// 3295. Report Spam Message
// https://leetcode.com/problems/report-spam-message/
// https://leetcode.cn/problems/report-spam-message/
//
// Medium
//
// You are given an array of strings message and an array of strings bannedWords.
//
// An array of words is considered spam if there are at least two words in it that exactly match any word in bannedWords.
//
// Return true if the array message is spam, and false otherwise.
//
// Example 1:
//
// Input: message = ["hello","world","leetcode"], bannedWords = ["world","hello"]
//
// Output: true
//
// Explanation:
//
// The words "hello" and "world" from the message array both appear in the bannedWords array.
//
// Example 2:
//
// Input: message = ["hello","programming","fun"], bannedWords = ["world","programming","leetcode"]
//
// Output: false
//
// Explanation:
//
// Only one word from the message array ("programming") appears in the bannedWords array.
//
// Constraints:
//
// 1 <= message.length, bannedWords.length <= 10^5
// 1 <= message[i].length, bannedWords[i].length <= 15
// message[i] and bannedWords[i] consist only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        let banned_words: std::collections::HashSet<_> = banned_words.iter().collect();
        message.iter().filter(|m| banned_words.contains(m)).count() > 1
    }
}

#[test]
fn test() {
    let message = vec!["hello".to_string(), "world".to_string(), "leetcode".to_string()];
    let banned_words = vec!["world".to_string(), "hello".to_string()];
    assert!(Solution::report_spam(message, banned_words));

    let message = vec!["hello".to_string(), "programming".to_string(), "fun".to_string()];
    let banned_words = vec!["world".to_string(), "programming".to_string(), "leetcode".to_string()];
    assert!(!Solution::report_spam(message, banned_words));
}
