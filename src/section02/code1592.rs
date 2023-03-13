#![allow(dead_code)]

/*

// 1592. Rearrange Spaces Between Words
Easy
354
292
Companies

You are given a string text of words that are placed among some number of spaces. Each word consists of one or more lowercase English letters and are separated by at least one space. It's guaranteed that text contains at least one word.

Rearrange the spaces so that there is an equal number of spaces between every pair of adjacent words and that number is maximized. If you cannot redistribute all the spaces equally, place the extra spaces at the end, meaning the returned string should be the same length as text.

Return the string after rearranging the spaces.

Example 1:

Input: text = "  this   is  a sentence "
Output: "this   is   a   sentence"
Explanation: There are a total of 9 spaces and 4 words. We can evenly divide the 9 spaces between the words: 9 / (4-1) = 3 spaces.

Example 2:

Input: text = " practice   makes   perfect"
Output: "practice   makes   perfect "
Explanation: There are a total of 7 spaces and 3 words. 7 / (3-1) = 3 spaces plus 1 extra space. We place this extra space at the end of the string.

Constraints:

    1 <= text.length <= 100
    text consists of lowercase English letters and ' '.
    text contains at least one word.
*/

struct Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let mut spaces = 0;
        let mut words = Vec::new();
        let mut word = String::new();
        for c in text.chars() {
            if c == ' ' {
                spaces += 1;
                if !word.is_empty() {
                    words.push(word);
                    word = String::new();
                }
            } else {
                word.push(c);
            }
        }
        if !word.is_empty() {
            words.push(word);
        }
        let n = words.len();
        if n == 1 {
            return words[0].to_string() + &" ".repeat(spaces);
        }
        let mut res = String::new();
        let space = spaces / (n - 1);
        let extra = spaces % (n - 1);
        for (i, word) in words.iter().enumerate() {
            res.push_str(word);
            if i < n - 1 {
                res.push_str(&" ".repeat(space));
            }
        }
        res.push_str(&" ".repeat(extra));
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        ("  this   is  a sentence ", "this   is   a   sentence"),
        (" practice   makes   perfect", "practice   makes   perfect "),
        ("hello   world", "hello   world"),
        (
            "  walks  udp package   into  bar a",
            "walks  udp  package  into  bar  a ",
        ),
        ("a", "a"),
    ];
    for (text, expected) in cases {
        assert_eq!(Solution::reorder_spaces(text.to_string()), expected);
    }
}
