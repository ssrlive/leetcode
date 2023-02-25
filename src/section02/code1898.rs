#![allow(dead_code)]

/*

// 1898. Maximum Number of Removable Characters
// https://leetcode.com/problems/maximum-number-of-removable-characters/
// https://leetcode.cn/problems/maximum-number-of-removable-characters/
//
// Medium
//
// You are given two strings s and p where p is a subsequence of s. You are also given a distinct 0-indexed integer array removable containing a subset of indices of s (s is also 0-indexed).

You want to choose an integer k (0 <= k <= removable.length) such that, after removing k characters from s using the first k indices in removable, p is still a subsequence of s. More formally, you will mark the character at s[removable[i]] for each 0 <= i < k, then remove all marked characters and check if p is still a subsequence.

Return the maximum k you can choose such that p is still a subsequence of s after the removals.

A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

Example 1:

Input: s = "abcacb", p = "ab", removable = [3,1,0]
Output: 2
Explanation: After removing the characters at indices 3 and 1, "abcacb" becomes "accb".
"ab" is a subsequence of "accb".
If we remove the characters at indices 3, 1, and 0, "abcacb" becomes "ccb", and "ab" is no longer a subsequence.
Hence, the maximum k is 2.

Example 2:

Input: s = "abcbddddd", p = "abcd", removable = [3,2,1,4,5,6]
Output: 1
Explanation: After removing the character at index 3, "abcbddddd" becomes "abcddddd".
"abcd" is a subsequence of "abcddddd".

Example 3:

Input: s = "abcab", p = "abc", removable = [0,1,2,3,4]
Output: 0
Explanation: If you remove the first index in the array removable, "abc" is no longer a subsequence.

Constraints:

    1 <= p.length <= s.length <= 10^5
    0 <= removable.length < s.length
    0 <= removable[i] < s.length
    p is a subsequence of s.
    s and p both consist of lowercase English letters.
    The elements in removable are distinct.
*/

struct Solution;

impl Solution {
    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let s = s.into_bytes();
        let p = p.into_bytes();
        let mut tmp = s.clone();
        let mut l = 0;
        let mut r = removable.len();
        while l < r {
            let mid = (l + r) / 2;
            for i in 0..=mid {
                tmp[removable[i] as usize] = b' ';
            }
            if Self::is_subsequence(&tmp, &p) {
                l = mid + 1;
            } else {
                for i in 0..=mid {
                    tmp[removable[i] as usize] = s[removable[i] as usize];
                }
                r = mid;
            }
        }
        l as i32
    }

    fn is_subsequence(s: &[u8], p: &[u8]) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < p.len() {
            if s[i] == p[j] {
                j += 1;
            }
            i += 1;
        }
        j == p.len()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("abcacb", "ab", vec![3, 1, 0], 2),
        ("abcbddddd", "abcd", vec![3, 2, 1, 4, 5, 6], 1),
        ("abcab", "abc", vec![0, 1, 2, 3, 4], 0),
    ];
    for (s, p, rm, ex) in cases {
        assert_eq!(Solution::maximum_removals(s.to_string(), p.to_string(), rm), ex);
    }
}
