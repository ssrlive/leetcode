#![allow(dead_code)]

// 87. Scramble String
// https://leetcode.com/problems/scramble-string/
// https://leetcode.cn/problems/scramble-string/
//
// We can scramble a string s to get a string t using the following algorithm:
//
// If the length of the string is 1, stop.
// If the length of the string is > 1, do the following:
// Split the string into two non-empty substrings at a random index, i.e., if the string is s, divide it to x and y where s = x + y.
// Randomly decide to swap the two substrings or to keep them in the same order. i.e., after this step, s may become s = x + y or s = y + x.
// Apply step 1 recursively on each of the two substrings x and y.
// Given two strings s1 and s2 of the same length, return true if s2 is a scrambled string of s1, otherwise, return false.
//
// Example 1:
//
// Input: s1 = "great", s2 = "rgeat"
// Output: true
// Explanation: One possible scenario applied on s1 is:
// "great" --> "gr/eat" // divide at random index.
// "gr/eat" --> "gr/eat" // random decision is not to swap the two substrings and keep them in order.
// "gr/eat" --> "g/r / e/at" // apply the same algorithm recursively on both substrings. divide at random index each of them.
// "g/r / e/at" --> "r/g / e/at" // random decision was to swap the first substring and to keep the second substring in the same order.
// "r/g / e/at" --> "r/g / e/ a/t" // again apply the algorithm recursively, divide "at" to "a/t".
// "r/g / e/ a/t" --> "r/g / e/ a/t" // random decision is to keep both substrings in the same order.
// The algorithm stops now, and the result string is "rgeat" which is s2.
// As one possible scenario led s1 to be scrambled to s2, we return true.
//
// Example 2:
//
// Input: s1 = "abcde", s2 = "caebd"
// Output: false
//
// Example 3:
//
// Input: s1 = "a", s2 = "a"
// Output: true
//
// Constraints:
//
// - s1.length == s2.length
// - 1 <= s1.length <= 30
// - s1 and s2 consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        fn check(s1: &Vec<char>, s2: &Vec<char>, i: usize, j: usize, len: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if dp[i][j][len] != -1 {
                return dp[i][j][len];
            }

            let mut same = true;
            for k in 0..len {
                if s1[i + k] == s2[j + k] {
                    continue;
                }
                same = false;
                break;
            }
            if same {
                dp[i][j][len] = 1;
                return dp[i][j][len];
            }

            for k in 1..len {
                if check(s1, s2, i, j + len - k, k, dp) == 1 && check(s1, s2, i + k, j, len - k, dp) == 1 {
                    dp[i][j][len] = 1;
                    return dp[i][j][len];
                }

                if check(s1, s2, i, j, k, dp) == 1 && check(s1, s2, i + k, j + k, len - k, dp) == 1 {
                    dp[i][j][len] = 1;
                    return dp[i][j][len];
                }
            }

            dp[i][j][len] = 0;
            dp[i][j][len]
        }

        let n = s1.len();
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![vec![-1i32; n + 1]; n]; n];

        check(&s1, &s2, 0, 0, n, &mut dp) == 1
    }
}

#[test]
fn test() {
    let cases = vec![
        ("great", "rgeat", true),
        ("abcde", "caebd", false),
        ("a", "a", true),
        ("abc", "bca", true),
        ("abc", "acb", true),
        ("abc", "bac", true),
        ("abcd", "dabc", true),
        ("abcd", "dcab", true),
    ];
    for (s1, s2, expected) in cases {
        assert_eq!(Solution::is_scramble(s1.to_string(), s2.to_string()), expected);
    }
}
