#![allow(dead_code)]

/*

// 2272. Substring With Largest Variance
// https://leetcode.com/problems/substring-with-largest-variance/
// https://leetcode.cn/problems/substring-with-largest-variance/
//
// Hard
//
// The variance of a string is defined as the largest difference between the number of occurrences of any 2 characters present in the string. Note the two characters may or may not be the same.

Given a string s consisting of lowercase English letters only, return the largest variance possible among all substrings of s.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: s = "aababbb"
Output: 3
Explanation:
All possible variances along with their respective substrings are listed below:
- Variance 0 for substrings "a", "aa", "ab", "abab", "aababb", "ba", "b", "bb", and "bbb".
- Variance 1 for substrings "aab", "aba", "abb", "aabab", "ababb", "aababbb", and "bab".
- Variance 2 for substrings "aaba", "ababbb", "abbb", and "babb".
- Variance 3 for substring "babbb".
Since the largest possible variance is 3, we return it.

Example 2:

Input: s = "abcde"
Output: 0
Explanation:
No letter occurs more than once in s, so the variance of every substring is 0.

Constraints:

    1 <= s.length <= 10^4
    s consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let mut best = 0;
        let mut running_counts = [0i32; 26];
        let mut max_ci_minus_cj = [[0; 26]; 26];
        let mut max_ci_minus_cj_stable = [[-1000000; 26]; 26];
        for b in s.bytes() {
            let idx = (b - b'a') as usize;
            for oth in 0..26 {
                max_ci_minus_cj_stable[idx][oth] = max_ci_minus_cj[idx][oth];
            }
            running_counts[idx] += 1;
            for oth in 0..26 {
                max_ci_minus_cj[idx][oth] = (max_ci_minus_cj[idx][oth]).max(running_counts[idx] - running_counts[oth]);
                let idx_best = running_counts[idx] - running_counts[oth] + max_ci_minus_cj_stable[oth][idx];
                let oth_best = running_counts[oth] - running_counts[idx] + max_ci_minus_cj_stable[idx][oth];
                best = best.max(idx_best);
                best = best.max(oth_best);
            }
        }
        best
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("aababbb", 3),
        ("abcde", 0),
        ("a", 0),
        ("aa", 0),
        ("aaa", 0),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::largest_variance(s.to_string()), expected);
    }
}
