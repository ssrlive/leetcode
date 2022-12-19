#![allow(dead_code)]

// 316. Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// https://leetcode.cn/problems/remove-duplicates-from-sorted-array/
//
// Given a string s, remove duplicate letters so that every letter appears once and only once. You must make sure your result is
// `the smallest in lexicographical order` among all possible results.
//
// Example 1:
//
// Input: s = "bcabc"
// Output: "abc"
//
// Example 2:
//
// Input: s = "cbacdcbc"
// Output: "acdb"
//
// Constraints:
//
// - 1 <= s.length <= 10^4
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack = Vec::new();
        let mut last = [0; 26];
        let mut seen = [false; 26];
        let chars = s.chars().collect::<Vec<char>>();
        for (i, c) in chars.iter().enumerate() {
            last[(*c as u8 - b'a') as usize] = i;
        }
        for (i, c) in chars.iter().enumerate() {
            let idx = (*c as u8 - b'a') as usize;
            if seen[idx] {
                continue;
            }
            while let Some(&top) = stack.last() {
                if top > *c && i < last[(top as u8 - b'a') as usize] {
                    seen[(top as u8 - b'a') as usize] = false;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(*c);
            seen[idx] = true;
        }
        stack.iter().collect()
    }
}

#[test]
fn test_remove_duplicate_letters() {
    assert_eq!(Solution::remove_duplicate_letters("bcabc".to_string()), "abc");
    assert_eq!(Solution::remove_duplicate_letters("cbacdcbc".to_string()), "acdb");
}
