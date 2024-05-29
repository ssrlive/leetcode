#![allow(dead_code)]

// 3163. String Compression III
// https://leetcode.com/problems/string-compression-iii/
// https://leetcode.cn/problems/string-compression-iii/
//
// Medium
//
// Given a string word, compress it using the following algorithm:
//
// Begin with an empty string comp. While word is not empty, use the following operation:
// Remove a maximum length prefix of word made of a single character c repeating at most 9 times.
// Append the length of the prefix followed by c to comp.
// Return the string comp.
//
// Example 1:
//
// Input: word = "abcde"
//
// Output: "1a1b1c1d1e"
//
// Explanation:
//
// Initially, comp = "". Apply the operation 5 times, choosing "a", "b", "c", "d", and "e" as the prefix in each operation.
//
// For each prefix, append "1" followed by the character to comp.
//
// Example 2:
//
// Input: word = "aaaaaaaaaaaaaabb"
//
// Output: "9a5a2b"
//
// Explanation:
//
// Initially, comp = "". Apply the operation 3 times, choosing "aaaaaaaaa", "aaaaa", and "bb" as the prefix in each operation.
//
// For prefix "aaaaaaaaa", append "9" followed by "a" to comp.
// For prefix "aaaaa", append "5" followed by "a" to comp.
// For prefix "bb", append "2" followed by "b" to comp.
//
// Constraints:
//
// 1 <= word.length <= 2 * 10^5
// word consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut ans = String::new();
        let mut chars = word.chars().peekable();

        while let Some(&current_char) = chars.peek() {
            let mut count = 0;
            while chars.peek() == Some(&current_char) && count < 9 {
                chars.next();
                count += 1;
            }
            ans.push_str(&count.to_string());
            ans.push(current_char);
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::compressed_string("abcde".to_string()), "1a1b1c1d1e");
    assert_eq!(Solution::compressed_string("aaaaaaaaaaaaaabb".to_string()), "9a5a2b");
}
