#![allow(dead_code)]

// 2842. Count K-Subsequences of a String With Maximum Beauty
// https://leetcode.com/problems/count-k-subsequences-of-a-string-with-maximum-beauty/
// https://leetcode.cn/problems/count-k-subsequences-of-a-string-with-maximum-beauty/
//
// Hard
//
// You are given a string s and an integer k.
//
// A k-subsequence is a subsequence of s, having length k, and all its characters are unique, i.e., every character occurs once.
//
// Let f(c) denote the number of times the character c occurs in s.
//
// The beauty of a k-subsequence is the sum of f(c) for every character c in the k-subsequence.
//
// For example, consider s = "abbbdd" and k = 2:
//
//     f('a') = 1, f('b') = 3, f('d') = 2
//     Some k-subsequences of s are:
//         "abbbdd" -> "ab" having a beauty of f('a') + f('b') = 4
//         "abbbdd" -> "ad" having a beauty of f('a') + f('d') = 3
//         "abbbdd" -> "bd" having a beauty of f('b') + f('d') = 5
//
// Return an integer denoting the number of k-subsequences whose beauty is the maximum
// among all k-subsequences. Since the answer may be too large, return it modulo 109 + 7.
//
// A subsequence of a string is a new string formed from the original string
// by deleting some (possibly none) of the characters without disturbing the relative positions of the remaining characters.
//
// Notes
//
//     f(c) is the number of times a character c occurs in s, not a k-subsequence.
//     Two k-subsequences are considered different if one is formed by an index
//     that is not present in the other. So, two k-subsequences may form the same string.
//
// Example 1:
//
// Input: s = "bcca", k = 2
// Output: 4
// Explanation: From s we have f('a') = 1, f('b') = 1, and f('c') = 2.
// The k-subsequences of s are:
// bcca having a beauty of f('b') + f('c') = 3
// bcca having a beauty of f('b') + f('c') = 3
// bcca having a beauty of f('b') + f('a') = 2
// bcca having a beauty of f('c') + f('a') = 3
// bcca having a beauty of f('c') + f('a') = 3
// There are 4 k-subsequences that have the maximum beauty, 3.
// Hence, the answer is 4.
//
// Example 2:
//
// Input: s = "abbcd", k = 4
// Output: 2
// Explanation: From s we have f('a') = 1, f('b') = 2, f('c') = 1, and f('d') = 1.
// The k-subsequences of s are:
// abbcd having a beauty of f('a') + f('b') + f('c') + f('d') = 5
// abbcd having a beauty of f('a') + f('b') + f('c') + f('d') = 5
// There are 2 k-subsequences that have the maximum beauty, 5.
// Hence, the answer is 2.
//
// Constraints:
//
//     1 <= s.length <= 2 * 10^5
//     1 <= k <= s.length
//     s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn count_k_subsequences_with_max_beauty(s: String, k: i32) -> i32 {
        let mut count = [0; 26];
        for c in s.chars().collect::<Vec<_>>() {
            count[c as usize - 'a' as usize] += 1;
        }
        count.sort();
        count.reverse();
        if k > count.len() as i32 || count[k as usize - 1] == 0 {
            return 0;
        }

        const MOD: i64 = 1_000_000_007;
        let mut ret = 1;

        for &item in count.iter().take(k as usize) {
            ret = (ret * item as i64) % MOD;
        }

        let (mut l, mut r) = (k as usize - 1, k as usize - 1);
        while l > 0 && count[l - 1] == count[l] {
            l -= 1;
        }
        while r + 1 < 26 && count[r + 1] == count[r] {
            r += 1;
        }

        let (cnt, len) = (k as i64 - l as i64, r as i64 - l as i64 + 1);
        for i in 0..cnt {
            ret = (ret * (len - i)) % MOD;
            ret = (ret * Self::divide(i + 1)) % MOD;
        }

        ret as _
    }

    fn divide(a: i64) -> i64 {
        const MOD: i64 = 1_000_000_007;
        let (mut ret, mut base, mut k) = (1, a, MOD - 2);
        while k > 0 {
            if k % 2 == 1 {
                ret = (ret * base) % MOD;
            }
            base = (base * base) % MOD;
            k /= 2;
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_k_subsequences_with_max_beauty("bcca".to_string(), 2), 4);
    assert_eq!(Solution::count_k_subsequences_with_max_beauty("abbcd".to_string(), 4), 2);
}
