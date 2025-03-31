#![allow(dead_code)]

// 3504. Longest Palindrome After Substring Concatenation II
// https://leetcode.com/problems/longest-palindrome-after-substring-concatenation-ii/
// https://leetcode.cn/problems/longest-palindrome-after-substring-concatenation-ii/
//
// Hard
//
// You are given two strings, s and t.
//
// You can create a new string by selecting a substring from s (possibly empty) and a substring from t (possibly empty), then concatenating them in order.
//
// Return the length of the longest substring that can be formed this way.
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
//     1 <= s.length, t.length <= 1000
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
            let n = s.len() as i32;
            let mut res = vec![0; n as usize + 1];
            for i in 0..n {
                let mut j = n - 1;
                while i <= j && res[i as usize] < 2 {
                    res[i as usize] = if check_pal(s, i, j) { j - i + 1 } else { 1 };
                    j -= 1;
                }
            }
            res
        }

        let mut t = t.as_bytes().to_vec();
        t.reverse();
        let s = s.as_bytes();
        let s_pal = longest_palindromes(s);
        let t_pal = longest_palindromes(&t);
        let m = s.len() as i32;
        let n = t.len() as i32;
        let mut res = 0;
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
        for i in 0..m {
            for j in 0..n {
                if s[i as usize] == t[j as usize] {
                    dp[(i + 1) as usize][(j + 1) as usize] = dp[i as usize][j as usize] + 1;
                } else {
                    dp[(i + 1) as usize][(j + 1) as usize] = 0;
                }
                res = res.max(
                    2 * dp[(i + 1) as usize][(j + 1) as usize]
                        + s_pal[i as usize + (s[i as usize] == t[j as usize]) as usize]
                            .max(t_pal[j as usize + (s[i as usize] == t[j as usize]) as usize]),
                );
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
