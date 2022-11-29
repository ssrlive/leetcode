#![allow(dead_code)]

// 451. Sort Characters By Frequency
// https://leetcode.com/problems/sort-characters-by-frequency/
//
// Given a string s, sort it in decreasing order based on the frequency of the characters. The frequency of a character is the number of times it appears in the string.
//
// Return the sorted string. If there are multiple answers, return any of them.
//
// Example 1:
//
// Input: s = "tree"
// Output: "eert"
// Explanation: 'e' appears twice while 'r' and 't' both appear once.
// So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.
//
// Example 2:
//
// Input: s = "cccaaa"
// Output: "aaaccc"
// Explanation: Both 'c' and 'a' appear three times, so both "cccaaa" and "aaaccc" are valid answers.
// Note that "cacaca" is incorrect, as the same characters must be together.
//
// Example 3:
//
// Input: s = "Aabb"
// Output: "bbAa"
// Explanation: "bbaA" is also a valid answer, but "Aabb" is incorrect.
// Note that 'A' and 'a' are treated as two different characters.
//
// Constraints:
//
// - 1 <= s.length <= 5 * 105
// - s consists of uppercase and lowercase English letters and digits.
//

struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map = std::collections::HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut v: Vec<_> = map.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v.iter().map(|(c, n)| c.to_string().repeat(*n)).collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::frequency_sort("tree".to_string()), "eetr".to_string());
    assert_eq!(Solution::frequency_sort("cccaaa".to_string()), "aaaccc".to_string());
    assert_eq!(Solution::frequency_sort("Aabb".to_string()), "bbAa".to_string());
}
