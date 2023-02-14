#![allow(dead_code)]

/*

// 1641. Count Sorted Vowel Strings
// https://leetcode.com/problems/count-sorted-vowel-strings/
// https://leetcode.cn/problems/count-sorted-vowel-strings/
//
// Medium
//
// Given an integer n, return the number of strings of length n that consist only of vowels (a, e, i, o, u) and are lexicographically sorted.

A string s is lexicographically sorted if for all valid i, s[i] is the same as or comes before s[i+1] in the alphabet.

Example 1:

Input: n = 1
Output: 5
Explanation: The 5 sorted strings that consist of vowels only are ["a","e","i","o","u"].

Example 2:

Input: n = 2
Output: 15
Explanation: The 15 sorted strings that consist of vowels only are
["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"].
Note that "ea" is not a valid string since 'e' comes after 'a' in the alphabet.

Example 3:

Input: n = 33
Output: 66045

Constraints:

    1 <= n <= 50
*/

struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = [1, 1, 1, 1, 1];
        for _ in 1..n {
            for i in 0..4 {
                dp[i] = dp[i..].iter().sum();
            }
        }
        dp.iter().sum()
    }
}

#[test]
fn test() {
    let cases = vec![(1, 5), (2, 15), (33, 66045)];
    for (n, expected) in cases {
        assert_eq!(Solution::count_vowel_strings(n), expected);
    }
}
