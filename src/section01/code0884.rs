#![allow(dead_code)]

// 884. Uncommon Words from Two Sentences
// https://leetcode.com/problems/uncommon-words-from-two-sentences/
// https://leetcode.cn/problems/uncommon-words-from-two-sentences/
//
// A sentence is a string of single-space separated words where each word consists only of lowercase letters.
//
// A word is uncommon if it appears exactly once in one of the sentences, and does not appear in the other sentence.
//
// Given two sentences s1 and s2, return a list of all the uncommon words. You may return the answer in any order.
//
// Example 1:
//
// Input: s1 = "this apple is sweet", s2 = "this apple is sour"
// Output: ["sweet","sour"]
//
// Example 2:
//
// Input: s1 = "apple apple", s2 = "banana"
// Output: ["banana"]
//
// Constraints:
//
// - 1 <= s1.length, s2.length <= 200
// - s1 and s2 consist of lowercase English letters and spaces.
// - s1 and s2 do not have leading or trailing spaces.
// - All the words in s1 and s2 are separated by a single space.
//

struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        s1.split(' ').chain(s2.split(' ')).for_each(|s| *map.entry(s).or_insert(0) += 1);
        map.iter()
            .filter_map(|(key, val)| match *val == 1 {
                true => Some((*key).to_string()),
                false => None,
            })
            .collect::<Vec<_>>()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("this apple is sweet", "this apple is sour", vec!["sour", "sweet"]),
        ("apple apple", "banana", vec!["banana"]),
    ];
    for (s1, s2, answer) in cases {
        let mut res = Solution::uncommon_from_sentences(s1.to_string(), s2.to_string());
        res.sort();
        assert_eq!(res, answer);
    }
}
