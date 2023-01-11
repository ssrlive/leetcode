#![allow(dead_code)]

// 1078. Occurrences After Bigram
// https://leetcode.com/problems/occurrences-after-bigram/
// https://leetcode.cn/problems/occurrences-after-bigram/
//
// Given two strings first and second, consider occurrences in some text of the form "first second third",
// where second comes immediately after first, and third comes immediately after second.
//
// Return an array of all the words third for each occurrence of "first second third".
//
// Example 1:
//
// Input: text = "alice is a good girl she is a good student", first = "a", second = "good"
// Output: ["girl","student"]
//
// Example 2:
//
// Input: text = "we will we will rock you", first = "we", second = "will"
// Output: ["we","rock"]
//
// Constraints:
//
// - 1 <= text.length <= 1000
// - text consists of lowercase English letters and spaces.
// - All the words in text a separated by a single space.
// - 1 <= first.length, second.length <= 10
// - first and second consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut counter = 0u8;
        let mut res = Vec::new();
        let mut prev_ind = 0;
        let eq_strings = first == second;

        for (ind, word) in text.split(' ').enumerate() {
            if counter == 0b11 && (ind - prev_ind == 1) {
                res.push(word.to_string());
                counter = (eq_strings && word == first) as u8;
            }

            match (counter, word == first, word == second) {
                (_, true, false) | (0, true, true) => counter = 0b01,
                (_, _, true) if ind - prev_ind == 1 => counter |= 0b10,
                _ => continue,
            }

            prev_ind = ind;
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            "alice is a good girl she is a good student",
            "a",
            "good",
            vec!["girl", "student"],
        ),
        ("we will we will rock you", "we", "will", vec!["we", "rock"]),
    ];
    for (text, first, second, expect) in cases {
        let text = text.to_string();
        let first = first.to_string();
        let second = second.to_string();
        assert_eq!(Solution::find_ocurrences(text, first, second), expect);
    }
}
