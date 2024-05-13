#![allow(dead_code)]

// 819. Most Common Word
// https://leetcode.com/problems/most-common-word/
// https://leetcode.cn/problems/most-common-word/
//
// Given a string paragraph and a string array of the banned words banned, return the most frequent word that is not banned.
// It is guaranteed there is at least one word that is not banned, and that the answer is unique.
//
// The words in paragraph are case-insensitive and the answer should be returned in lowercase.
//
// Example 1:
//
// Input: paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.", banned = ["hit"]
// Output: "ball"
// Explanation:
// "hit" occurs 3 times, but it is a banned word.
// "ball" occurs twice (and no other word does), so it is the most frequent non-banned word in the paragraph.
// Note that words in the paragraph are not case sensitive,
// that punctuation is ignored (even if adjacent to words, such as "ball,"),
// and that "hit" isn't the answer even though it occurs more because it is banned.
//
// Example 2:
//
// Input: paragraph = "a.", banned = []
// Output: "a"
//
// Constraints:
//
// - 1 <= paragraph.length <= 1000
// - paragraph consists of English letters, space ' ', or one of the symbols: "!?',;.".
// - 0 <= banned.length <= 100
// - 1 <= banned[i].length <= 10
// - banned[i] consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        use std::collections::HashMap;
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut max: i32 = 0;
        let mut result: String = String::new();
        let mut word: String = String::new();
        let mut banned: Vec<String> = banned;
        banned.sort();
        for c in paragraph.chars() {
            if c.is_alphabetic() {
                word.push(c.to_ascii_lowercase());
            } else if !word.is_empty() {
                if banned.binary_search(&word).is_err() {
                    let count = map.entry(word.clone()).or_insert(0);
                    *count += 1;
                    if *count > max {
                        max = *count;
                        result.clone_from(&word);
                    }
                }
                word.clear();
            }
        }
        if !word.is_empty() && banned.binary_search(&word).is_err() {
            let count = map.entry(word.clone()).or_insert(0);
            *count += 1;
            if *count > max {
                result.clone_from(&word);
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
            vec!["hit".to_string()]
        ),
        "ball".to_string()
    );
    assert_eq!(Solution::most_common_word("a.".to_string(), vec![]), "a".to_string());
}
