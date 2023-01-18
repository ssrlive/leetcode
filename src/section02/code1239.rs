#![allow(dead_code)]

// 1239. Maximum Length of a Concatenated String with Unique Characters
// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
// https://leetcode.cn/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
//
// Medium
//
// You are given an array of strings arr. A string s is formed by the concatenation of a subsequence of arr that has unique characters.
//
// Return the maximum possible length of s.
//
// A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
//
// Example 1:
//
// Input: arr = ["un","iq","ue"]
// Output: 4
// Explanation: All the valid concatenations are:
// - ""
// - "un"
// - "iq"
// - "ue"
// - "uniq" ("un" + "iq")
// - "ique" ("iq" + "ue")
// Maximum length is 4.
//
// Example 2:
//
// Input: arr = ["cha","r","act","ers"]
// Output: 6
// Explanation: Possible longest valid concatenations are "chaers" ("cha" + "ers") and "acters" ("act" + "ers").
//
// Example 3:
//
// Input: arr = ["abcdefghijklmnopqrstuvwxyz"]
// Output: 26
// Explanation: The only string in arr has all 26 characters.
//
// Constraints:
//
// -    1 <= arr.length <= 16
// -    1 <= arr[i].length <= 26
// -    arr[i] contains only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let f = |s: &String| {
            let mut bitset = 0;
            for bm in s.bytes().map(|b| 1 << (b - b'a')) {
                if bitset & bm != 0 {
                    return None;
                } else {
                    bitset |= bm;
                }
            }
            Some(bitset)
        };
        let sets = arr.iter().filter_map(f).collect::<Vec<_>>();

        let mut stack = vec![(0_i32, 0)];
        let mut rez = 0;

        while let Some((bitset, i)) = stack.pop() {
            if i == sets.len() {
                rez = rez.max(bitset.count_ones());
            } else {
                if bitset & sets[i] == 0 {
                    stack.push((bitset | sets[i], i + 1));
                }
                stack.push((bitset, i + 1));
            }
        }
        rez as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["un", "iq", "ue"], 4),
        (vec!["cha", "r", "act", "ers"], 6),
        (vec!["abcdefghijklmnopqrstuvwxyz"], 26),
    ];
    for (arr, expected) in cases {
        let arr = arr.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::max_length(arr), expected);
    }
}
