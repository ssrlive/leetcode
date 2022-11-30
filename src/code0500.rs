#![allow(dead_code)]

// 500. Keyboard Row
// https://leetcode.com/problems/keyboard-row/
//
// Given an array of strings words, return the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below.
//
// In the American keyboard:
//
// the first row consists of the characters "qwertyuiop",
// the second row consists of the characters "asdfghjkl", and
// the third row consists of the characters "zxcvbnm".
//
// Example 1:
//
// Input: words = ["Hello","Alaska","Dad","Peace"]
// Output: ["Alaska","Dad"]
//
// Example 2:
//
// Input: words = ["omk"]
// Output: []
//
// Example 3:
//
// Input: words = ["adsdf","sfd"]
// Output: ["adsdf","sfd"]
//
// Constraints:
//
// - 1 <= words.length <= 20
// - 1 <= words[i].length <= 100
// - words[i] consists of English letters (both lowercase and uppercase).
//

struct Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut row1 = std::collections::HashSet::new();
        let mut row2 = std::collections::HashSet::new();
        let mut row3 = std::collections::HashSet::new();
        for c in "qwertyuiop".chars() {
            row1.insert(c);
        }
        for c in "asdfghjkl".chars() {
            row2.insert(c);
        }
        for c in "zxcvbnm".chars() {
            row3.insert(c);
        }
        for word in words {
            let mut row = 0;
            for c in word.chars() {
                let c = c.to_ascii_lowercase();
                if row1.contains(&c) {
                    if row == 0 {
                        row = 1;
                    } else if row != 1 {
                        row = 0;
                        break;
                    }
                } else if row2.contains(&c) {
                    if row == 0 {
                        row = 2;
                    } else if row != 2 {
                        row = 0;
                        break;
                    }
                } else if row3.contains(&c) {
                    if row == 0 {
                        row = 3;
                    } else if row != 3 {
                        row = 0;
                        break;
                    }
                }
            }
            if row != 0 {
                result.push(word);
            }
        }
        result
    }
}

#[test]
fn test() {
    let words = vec![
        "Hello".to_string(),
        "Alaska".to_string(),
        "Dad".to_string(),
        "Peace".to_string(),
    ];
    let result = Solution::find_words(words);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "Alaska");
    assert_eq!(result[1], "Dad");
}
