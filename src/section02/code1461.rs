#![allow(dead_code)]

/*
// 1461. Check If a String Contains All Binary Codes of Size K
Medium

Given a binary string s and an integer k, return true if every binary code of length k is a substring of s. Otherwise, return false.

Example 1:

Input: s = "00110110", k = 2
Output: true
Explanation: The binary codes of length 2 are "00", "01", "10" and "11". They can be all found as substrings at indices 0, 1, 3 and 2 respectively.

Example 2:

Input: s = "0110", k = 1
Output: true
Explanation: The binary codes of length 1 are "0" and "1", it is clear that both exist as a substring.

Example 3:

Input: s = "0110", k = 2
Output: false
Explanation: The binary code "00" is of length 2 and does not exist in the array.

Constraints:

    1 <= s.length <= 5 * 10^5
    s[i] is either '0' or '1'.
    1 <= k <= 20
*/

struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut set = std::collections::HashSet::new();
        let mut i = 0;
        let mut j = k as usize;
        while j <= s.len() {
            set.insert(&s[i..j]);
            i += 1;
            j += 1;
        }
        set.len() == 2usize.pow(k as u32)
    }
}

#[test]
fn test() {
    let cases = vec![("00110110", 2, true), ("0110", 1, true), ("0110", 2, false)];
    for (s, k, expected) in cases {
        assert_eq!(Solution::has_all_codes(s.to_string(), k), expected);
    }
}
