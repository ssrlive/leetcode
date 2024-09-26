#![allow(dead_code)]

// 3292. Minimum Number of Valid Strings to Form Target II
// https://leetcode.com/problems/minimum-number-of-valid-strings-to-form-target-ii/
// https://leetcode.cn/problems/minimum-number-of-valid-strings-to-form-target-ii/
//
// Hard
//
// You are given an array of strings words and a string target.
//
// A string x is called valid if x is a prefix of any string in words.
//
// Return the minimum number of valid strings that can be concatenated to form target.
// If it is not possible to form target, return -1.
//
// Example 1:
//
// Input: words = ["abc","aaaaa","bcdef"], target = "aabcdabc"
//
// Output: 3
//
// Explanation:
//
// The target string can be formed by concatenating:
//
// Prefix of length 2 of words[1], i.e. "aa".
// Prefix of length 3 of words[2], i.e. "bcd".
// Prefix of length 3 of words[0], i.e. "abc".
//
// Example 2:
//
// Input: words = ["abababab","ab"], target = "ababaababa"
//
// Output: 2
//
// Explanation:
//
// The target string can be formed by concatenating:
//
// Prefix of length 5 of words[0], i.e. "ababa".
// Prefix of length 5 of words[0], i.e. "ababa".
//
// Example 3:
//
// Input: words = ["abcdef"], target = "xyz"
//
// Output: -1
//
// Constraints:
//
// 1 <= words.length <= 100
// 1 <= words[i].length <= 5 * 10^4
// The input is generated such that sum(words[i].length) <= 10^5.
// words[i] consists only of lowercase English letters.
// 1 <= target.length <= 5 * 10^4
// target consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let mut ps = vec![0; target.len() + 1];
        for w in words.iter() {
            let pi = Self::prefix_function(&format!("{}#{}", w, target));
            for i in 1..=target.len() {
                ps[i] = ps[i].max(pi[w.len() + i]);
            }
        }
        let mut len = target.len() as i32;
        let mut res = 0;
        while len > 0 && ps[len as usize] > 0 {
            res += 1;
            len -= ps[len as usize];
        }
        if len == 0 {
            res
        } else {
            -1
        }
    }

    fn prefix_function(s: &str) -> Vec<i32> {
        let s = s.as_bytes();
        let n = s.len();
        let mut pi = vec![0_i32; n];
        let mut i = 1;
        while i < n {
            let mut j = pi[i - 1];
            while j > 0 && s[i] != s[j as usize] {
                j = pi[(j - 1) as usize];
            }
            if s[i] == s[j as usize] {
                j += 1;
            }
            pi[i] = j;
            i += 1;
        }
        pi
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_valid_strings(
            vec!["abc".to_string(), "aaaaa".to_string(), "bcdef".to_string()],
            "aabcdabc".to_string()
        ),
        3
    );
    assert_eq!(
        Solution::min_valid_strings(vec!["abababab".to_string(), "ab".to_string()], "ababaababa".to_string()),
        2
    );
    assert_eq!(Solution::min_valid_strings(vec!["abcdef".to_string()], "xyz".to_string()), -1);
}
