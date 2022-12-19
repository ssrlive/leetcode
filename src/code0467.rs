#![allow(dead_code)]

// 467. Unique Substrings in Wraparound String
// https://leetcode.com/problems/unique-substrings-in-wraparound-string/
// https://leetcode.cn/problems/unique-substrings-in-wraparound-string/
//
// We define the string base to be the infinite wraparound string of "abcdefghijklmnopqrstuvwxyz", so base will look like this:
//
// - "...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....".
//
// Given a string s, return the number of unique non-empty substrings of s are present in base.
//
// Example 1:
//
// Input: s = "a"
// Output: 1
// Explanation: Only the substring "a" of s is in base.
//
// Example 2:
//
// Input: s = "cac"
// Output: 2
// Explanation: There are two substrings ("a", "c") of s in base.
//
// Example 3:
//
// Input: s = "zab"
// Output: 6
// Explanation: There are six substrings ("z", "a", "b", "za", "ab", and "zab") of s in base.
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        fn _find_substring_in_wrapround_string(p: String) -> Option<i32> {
            let mut counts = vec![0; 26];
            let mut len = 1;
            let p = p.as_bytes();
            for (i, ch) in p.iter().enumerate() {
                let ch = *ch as i32;
                let v = if i > 0 { *p.get(i - 1)? as i32 } else { 0 };

                if i > 0 && (ch - v == 1 || ch - v == -25) {
                    len += 1;
                } else {
                    len = 1;
                }
                let idx = ch as usize - 'a' as usize;
                counts[idx] = counts[idx].max(len);
            }
            Some(counts.iter().sum())
        }

        _find_substring_in_wrapround_string(p).unwrap_or_default()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_substring_in_wrapround_string("a".to_string()), 1);
    assert_eq!(Solution::find_substring_in_wrapround_string("cac".to_string()), 2);
    assert_eq!(Solution::find_substring_in_wrapround_string("zab".to_string()), 6);
}
