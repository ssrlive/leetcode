#![allow(dead_code)]

// 49. Group Anagrams
// https://leetcode.com/problems/group-anagrams/
// https://leetcode.cn/problems/group-anagrams/
//
// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
//
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
// typically using all the original letters exactly once.
//
// Example 1:
//
// Input: strs = ["eat","tea","tan","ate","nat","bat"]
// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
//
// Example 2:
//
// Input: strs = [""]
// Output: [[""]]
//
// Example 3:
//
// Input: strs = ["a"]
// Output: [["a"]]
//
// Constraints:
//
// - 1 <= strs.length <= 10^4
// - 0 <= strs[i].length <= 100
// - strs[i] consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        type MapType = HashMap<[u8; 26], Vec<String>>;
        let f = |mut acc: MapType, word: String| {
            let f = |mut acc: [u8; 26], c: char| {
                acc[c as usize - 'a' as usize] += 1;
                acc
            };
            let val = word.chars().fold([0u8; 26], f);
            acc.entry(val).or_default().push(word);
            acc
        };
        strs.into_iter().fold(MapType::new(), f).into_values().collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["eat", "tea", "tan", "ate", "nat", "bat"],
            vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]],
        ),
        (vec![""], vec![vec![""]]),
        (vec!["a"], vec![vec!["a"]]),
    ];

    for (strs, expected) in cases {
        let strs = strs.into_iter().map(|s| s.to_string()).collect();
        let mut result = Solution::group_anagrams(strs);
        result.sort();
        assert_eq!(result, expected);
    }
}
