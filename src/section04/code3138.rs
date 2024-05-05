#![allow(dead_code)]

// 3138. Minimum Length of Anagram Concatenation
// https://leetcode.com/problems/minimum-length-of-anagram-concatenation/
// https://leetcode.cn/problems/minimum-length-of-anagram-concatenation/
//
// Medium
//
// You are given a string s, which is known to be a concatenation of anagrams of some string t.
//
// Return the minimum possible length of the string t.
//
// An anagram is a word or phrase formed by rearranging the letters of a word or phrase,
// typically using all the original letters exactly once.
//
// Example 1:
//
// Input: s = "abba"
//
// Output: 2
//
// Explanation:
//
// One possible string t could be "ba".
//
// Example 2:
//
// Input: s = "cdef"
//
// Output: 4
//
// Explanation:
//
// One possible string t could be "cdef", notice that t can be equal to s.
//
// Constraints:
//
// 1 <= s.length <= 10^5
// s consist only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let n = s.len();
        for i in 1..=n {
            if n % i == 0 && Self::is_anagram(s.as_str(), i) {
                return i as i32;
            }
        }
        n as i32
    }

    fn is_anagram(s: &str, k: usize) -> bool {
        let n = s.len();
        let mut cnt = [0; 26];
        for i in 0..k {
            cnt[s.as_bytes()[i] as usize - b'a' as usize] += 1;
        }
        for i in (k..n).step_by(k) {
            let mut cnt2 = [0; 26];
            for j in i..i + k {
                cnt2[s.as_bytes()[j] as usize - b'a' as usize] += 1;
            }
            if cnt != cnt2 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_anagram_length("abba".to_string()), 2);
    assert_eq!(Solution::min_anagram_length("cdef".to_string()), 4);
}
