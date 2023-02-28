#![allow(dead_code)]

/*

// 1935. Maximum Number of Words You Can Type
// https://leetcode.com/problems/maximum-number-of-words-you-can-type/
// https://leetcode.cn/problems/maximum-number-of-words-you-can-type/
//
// Easy
//
// There is a malfunctioning keyboard where some letter keys do not work. All other keys on the keyboard work properly.

Given a string text of words separated by a single space (no leading or trailing spaces) and a string brokenLetters of all distinct letter keys that are broken, return the number of words in text you can fully type using this keyboard.

Example 1:

Input: text = "hello world", brokenLetters = "ad"
Output: 1
Explanation: We cannot type "world" because the 'd' key is broken.

Example 2:

Input: text = "leet code", brokenLetters = "lt"
Output: 1
Explanation: We cannot type "leet" because the 'l' and 't' keys are broken.

Example 3:

Input: text = "leet code", brokenLetters = "e"
Output: 0
Explanation: We cannot type either word because the 'e' key is broken.

Constraints:

    1 <= text.length <= 10^4
    0 <= brokenLetters.length <= 26
    text consists of words separated by a single space without any leading or trailing spaces.
    Each word only consists of lowercase English letters.
    brokenLetters consists of distinct lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_hash = broken_letters.into_bytes().iter().fold([false; 26], |mut acc, &b| {
            acc[(b - b'a') as usize] = true;
            acc
        });

        text.split_ascii_whitespace()
            .filter(|&s| s.as_bytes().iter().all(|&b| !broken_hash[(b - b'a') as usize]))
            .count() as _
    }
}

#[test]
fn test() {
    let cases = vec![
        ("hello world", "ad", 1),
        ("leet code", "lt", 1),
        ("leet code", "e", 0),
        ("hello world", "ad", 1),
        ("leet code", "lt", 1),
    ];
    for (text, broken_letters, expect) in cases {
        assert_eq!(
            Solution::can_be_typed_words(text.to_string(), broken_letters.to_string()),
            expect
        );
    }
}
