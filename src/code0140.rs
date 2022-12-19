#![allow(dead_code)]

// 140. Word Break II
// https://leetcode.com/problems/word-break-ii/
// https://leetcode.cn/problems/word-break-ii/
//
// Given a string s and a dictionary of strings wordDict, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences in any order.
//
// Note that the same word in the dictionary may be reused multiple times in the segmentation.
//

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn walk(result: &mut Vec<String>, buffer: &mut Vec<String>, s: &str, word_dict: &Vec<String>) {
            if s.is_empty() {
                result.push(buffer.join(" "));
            }

            for word in word_dict.iter() {
                let len = word.len();
                if s.len() >= len && word == &s[0..len] {
                    buffer.push(s[0..len].to_string());
                    walk(result, buffer, &s[len..], word_dict);
                    buffer.pop();
                }
            }
        }

        let mut buf = Vec::new();
        let mut res = Vec::new();

        walk(&mut res, &mut buf, &s, &word_dict);
        res
    }
}

#[test]
fn test_word_break() {
    let s = "catsanddog".to_string();
    let word_dict = vec![
        "cat".to_string(),
        "cats".to_string(),
        "and".to_string(),
        "sand".to_string(),
        "dog".to_string(),
    ];
    let mut ans = Solution::word_break(s, word_dict);
    ans.sort();
    let mut expected = vec!["cats and dog".to_string(), "cat sand dog".to_string()];
    expected.sort();
    assert_eq!(ans, expected);
    let s = "pineapplepenapple".to_string();
    let word_dict = vec![
        "apple".to_string(),
        "pen".to_string(),
        "applepen".to_string(),
        "pine".to_string(),
        "pineapple".to_string(),
    ];
    let mut ans = Solution::word_break(s, word_dict);
    ans.sort();
    let mut expected = vec![
        "pine apple pen apple".to_string(),
        "pineapple pen apple".to_string(),
        "pine applepen apple".to_string(),
    ];
    expected.sort();
    assert_eq!(ans, expected);
    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    assert_eq!(Solution::word_break(s, word_dict), Vec::<String>::new());
}
