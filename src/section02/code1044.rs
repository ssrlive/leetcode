#![allow(dead_code)]

// 1044. Longest Duplicate Substring
// https://leetcode.com/problems/longest-duplicate-substring/
// https://leetcode.cn/problems/longest-duplicate-substring/
//
// Given a string s, consider all duplicated substrings: (contiguous) substrings of s that occur 2 or more times. The occurrences may overlap.
//
// Return any duplicated substring that has the longest possible length. If s does not have a duplicated substring, the answer is "".
//
// Example 1:
//
// Input: s = "banana"
// Output: "ana"
//
// Example 2:
//
// Input: s = "abcd"
// Output: ""
//
// Constraints:
//
// - 2 <= s.length <= 3 * 10^4
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        use std::collections::HashMap;
        const P: i64 = 127;
        const MOD: i64 = 1_000_000_007;

        fn search(s: &[u8], v: &[i64], pow: i64, len: usize) -> Option<usize> {
            let mut hm = HashMap::new();
            for i in 0..=s.len() - len {
                let hash = (v[i + len] - v[i] * pow).rem_euclid(MOD);
                if let Some(&j) = hm.get(&hash)
                    && (0..len).all(|k| s[i + k] == s[j + k])
                {
                    return Some(i);
                }
                hm.insert(hash, i);
            }
            None
        }

        let mut v = vec![0; s.len() + 1];
        let mut pows = vec![1; s.len() + 1];
        for (i, u) in s.bytes().enumerate() {
            v[i + 1] = (v[i] * P + u as i64) % MOD;
            pows[i + 1] = pows[i] * P % MOD;
        }
        let mut answer = 0;
        let (mut lo, mut hi) = (0, s.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            if let Some(i) = search(s.as_bytes(), &v, pows[mid], mid) {
                answer = i;
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        s[answer..answer + lo - 1].to_string()
    }
}

#[test]
fn test() {
    let cases = vec![("banana", "ana"), ("abcd", "")];
    for (s, expected) in cases {
        assert_eq!(Solution::longest_dup_substring(s.to_string()), expected);
    }
}
