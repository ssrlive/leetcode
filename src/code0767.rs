#![allow(dead_code)]

// 767. Reorganize String
// https://leetcode.com/problems/reorganize-string/
// https://leetcode.cn/problems/reorganize-string/
//
// Given a string s, rearrange the characters of s so that any two adjacent characters are not the same.
//
// Return any possible rearrangement of s or return "" if not possible.
//
// Example 1:
//
// Input: s = "aab"
// Output: "aba"
//
// Example 2:
//
// Input: s = "aaab"
// Output: ""
//
// Constraints:
//
// - 1 <= s.length <= 500
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        use std::collections::BinaryHeap;

        const LETTERS: usize = (b'z' - b'a' + 1) as usize;

        let mut string = String::with_capacity(s.len());

        let mut characters = [0; LETTERS];
        for ch in s.as_bytes().iter().copied() {
            characters[(ch - b'a') as usize] += 1;
        }

        let mut heap = BinaryHeap::with_capacity(LETTERS);
        for (idx, count) in characters.iter().copied().enumerate() {
            if count > 0 {
                heap.push((count, (idx as u8 + b'a') as char));
            }
        }

        let mut last = None;
        while let Some((mut count, ch)) = heap.pop() {
            string.push(ch);
            count -= 1;

            if let Some(last) = last.take() {
                heap.push(last);
            }

            if count > 0 {
                last = Some((count, ch));
            }
        }

        if last.is_some() {
            string.truncate(0);
        }

        string
    }
}

#[test]
fn test() {
    let s = "aab".to_string();
    assert_eq!(Solution::reorganize_string(s), "aba".to_string());

    let s = "aaab".to_string();
    assert_eq!(Solution::reorganize_string(s), "".to_string());

    let s = "vvvlo".to_string();
    let expected = vec!["vlvov".to_string(), "vovlv".to_string()];
    assert!(expected.contains(&Solution::reorganize_string(s)));
}
