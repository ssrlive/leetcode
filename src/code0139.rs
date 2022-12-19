#![allow(dead_code)]

// 139. Word Break
// https://leetcode.com/problems/word-break/
// https://leetcode.cn/problems/word-break/
//
// Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.
//
// Note that the same word in the dictionary may be reused multiple times in the segmentation.
//

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut pref_dp: Vec<bool> = vec![false; n + 1];
        pref_dp[0] = true;
        let dict: std::collections::HashSet<String> = word_dict.into_iter().collect();
        for i in 1..=n {
            for j in (0..=i - 1).rev() {
                if pref_dp[j] && dict.contains(&s[j..=i - 1]) {
                    pref_dp[i] = true;
                    break;
                }
            }
        }
        pref_dp[n]
    }
}

#[test]
fn test() {
    let s = "leetcode".to_string();
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    assert_eq!(Solution::word_break(s, word_dict), true);
    let s = "applepenapple".to_string();
    let word_dict = vec!["apple".to_string(), "pen".to_string()];
    assert_eq!(Solution::word_break(s, word_dict), true);
    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    assert_eq!(Solution::word_break(s, word_dict), false);
}
