#![allow(dead_code)]

// 472. Concatenated Words
// https://leetcode.com/problems/concatenated-words/
//
// Given an array of strings words (without duplicates), return all the concatenated words in the given list of words.
//
// A concatenated word is defined as a string that is comprised entirely of at least two shorter words in the given array.
//
// Example 1:
//
// Input: words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
// Output: ["catsdogcats","dogcatsdog","ratcatdogcat"]
// Explanation: "catsdogcats" can be concatenated by "cats", "dog" and "cats";
// "dogcatsdog" can be concatenated by "dog", "cats" and "dog";
// "ratcatdogcat" can be concatenated by "rat", "cat", "dog" and "cat".
//
// Example 2:
//
// Input: words = ["cat","dog","catdog"]
// Output: ["catdog"]
//
// Constraints:
//
// - 1 <= words.length <= 104
// - 1 <= words[i].length <= 30
// - words[i] consists of only lowercase English letters.
// - All the strings of words are unique.
// - 1 <= sum(words[i].length) <= 105
//

struct Solution;

impl Solution {
    fn can_build(s: &str, prev: &std::collections::HashSet<&str>) -> bool {
        if prev.is_empty() {
            return false;
        }
        let mut dp: Vec<bool> = vec![false; s.len() + 1];
        dp[0] = true;
        for i in 1..=s.len() {
            for j in 0..i {
                if dp[j] && prev.get(&s[j..i]).is_some() {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[s.len()]
    }

    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        // 1. sort strings by length, O(nlogn)
        words.sort_by_key(|l| l.len());
        // 2. iterate over strings and check if you can build it with previously considered strings (smaller length)
        let mut res: Vec<String> = Vec::new();
        let mut previous: std::collections::HashSet<&str> = std::collections::HashSet::with_capacity(words.len());

        for s in &words {
            if Self::can_build(s, &previous) {
                res.push(s.clone());
            }
            previous.insert(s);
        }
        res
    }
}

#[test]
fn test() {
    let words = vec![
        "cat",
        "cats",
        "catsdogcats",
        "dog",
        "dogcatsdog",
        "hippopotamuses",
        "rat",
        "ratcatdogcat",
    ];
    let words = words.iter().map(|s| s.to_string()).collect();
    let mut output = Solution::find_all_concatenated_words_in_a_dict(words);
    output.sort();
    assert_eq!(output, vec!["catsdogcats", "dogcatsdog", "ratcatdogcat"]);

    let words = vec!["cat", "dog", "catdog"];
    let words = words.iter().map(|s| s.to_string()).collect();
    let output = Solution::find_all_concatenated_words_in_a_dict(words);
    assert_eq!(output, vec!["catdog"]);
}
