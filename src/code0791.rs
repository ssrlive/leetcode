#![allow(dead_code)]

// 791. Custom Sort String
// https://leetcode.com/problems/custom-sort-string/
//
// You are given two strings order and s. All the characters of order are unique and were sorted in some custom order previously.
//
// Permute the characters of s so that they match the order that order was sorted. More specifically,
// if a character x occurs before a character y in order, then x should occur before y in the permuted string.
//
// Return any permutation of s that satisfies this property.
//
// Example 1:
//
// Input: order = "cba", s = "abcd"
// Output: "cbad"
// Explanation:
// "a", "b", "c" appear in order, so the order of "a", "b", "c" should be "c", "b", and "a".
// Since "d" does not appear in order, it can be at any position in the returned string. "dcba", "cdba", "cbda" are also valid outputs.
//
// Example 2:
//
// Input: order = "cbafg", s = "abcd"
// Output: "cbad"
//
// Constraints:
//
// - 1 <= order.length <= 26
// - 1 <= s.length <= 200
// - order and s consist of lowercase English letters.
// - All the characters of order are unique.
//

struct Solution;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut buckets = vec![0; 26];
        for b in s.bytes() {
            buckets[(b - b'a') as usize] += 1;
        }
        let mut ans = "".to_owned();
        for b in order.bytes() {
            let idx = (b - b'a') as usize;
            for _ in 0..buckets[idx] {
                ans.push(b as char);
            }
            buckets[idx] = 0;
        }
        for (idx, &size) in buckets.iter().enumerate() {
            let c = (b'a' + idx as u8) as char;
            for _ in 0..size {
                ans.push(c);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let data = vec![("cba", "abcd", "cbad"), ("cbafg", "abcd", "cbad")];
    let data = data
        .into_iter()
        .map(|(order, s, ans)| (order.to_string(), s.to_string(), ans.to_string()))
        .collect::<Vec<(String, String, String)>>();
    for (order, s, ans) in data {
        assert_eq!(Solution::custom_sort_string(order, s), ans);
    }
}
