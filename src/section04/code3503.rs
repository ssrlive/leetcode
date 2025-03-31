#![allow(dead_code)]

// 3503. Longest Palindrome After Substring Concatenation I
// https://leetcode.com/problems/longest-palindrome-after-substring-concatenation-i/
// https://leetcode.cn/problems/longest-palindrome-after-substring-concatenation-i/
//
// Medium
//
// You are given two strings, s and t.
//
// You can create a new string by selecting a substring from s (possibly empty) and a substring
// from t (possibly empty), then concatenating them in order.
//
// Return the length of the longest that can be formed this way.
//
// Example 1:
//
// Input: s = "a", t = "a"
//
// Output: 2
//
// Explanation:
//
// Concatenating "a" from s and "a" from t results in "aa", which is a palindrome of length 2.
//
// Example 2:
//
// Input: s = "abc", t = "def"
//
// Output: 1
//
// Explanation:
//
// Since all characters are different, the longest palindrome is any single character, so the answer is 1.
//
// Example 3:
//
// Input: s = "b", t = "aaaa"
//
// Output: 4
//
// Explanation:
//
// Selecting "aaaa" from t is the longest palindrome, so the answer is 4.
//
// Example 4:
//
// Input: s = "abcde", t = "ecdba"
//
// Output: 5
//
// Explanation:
//
// Concatenating "abc" from s and "ba" from t results in "abcba", which is a palindrome of length 5.
//
// Constraints:
//
//     1 <= s.length, t.length <= 30
//     s and t consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        fn check_pal(s: &[u8], mut ll: i32, mut rr: i32) -> bool {
            while ll < rr {
                if s[ll as usize] != s[rr as usize] {
                    return false;
                }
                ll += 1;
                rr -= 1;
            }
            true
        }

        fn longest_palindromes(s: &[u8]) -> Vec<i32> {
            let n = s.len();
            let mut res = vec![0; n + 1];
            for (i, res_i) in res.iter_mut().enumerate().take(n) {
                let mut j = n as i32 - 1;
                let i = i as i32;
                while i <= j && *res_i < 2 {
                    *res_i = if check_pal(s, i, j) { j + 1 - i } else { 1 };
                    j -= 1;
                }
            }
            res
        }

        let mut t = t.into_bytes();
        t.reverse();
        let s = s.into_bytes();
        let s_pal = longest_palindromes(&s);
        let t_pal = longest_palindromes(&t);
        let m = s.len();
        let n = t.len();
        let mut res = 0;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                if s[i] == t[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = 0;
                }
                res = res.max(2 * dp[i + 1][j + 1] + s_pal[i + (s[i] == t[j]) as usize].max(t_pal[j + (s[i] == t[j]) as usize]));
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_palindrome("a".to_string(), "a".to_string()), 2);
    assert_eq!(Solution::longest_palindrome("abc".to_string(), "def".to_string()), 1);
    assert_eq!(Solution::longest_palindrome("b".to_string(), "aaaa".to_string()), 4);
    assert_eq!(Solution::longest_palindrome("abcde".to_string(), "ecdba".to_string()), 5);
}
