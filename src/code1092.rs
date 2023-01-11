#![allow(dead_code)]

// 1092. Shortest Common Supersequence
// https://leetcode.com/problems/shortest-common-supersequence/
// https://leetcode.cn/problems/shortest-common-supersequence/
//
// Given two strings str1 and str2, return the shortest string that has both str1 and str2 as subsequences.
// If there are multiple valid strings, return any of them.
//
// A string s is a subsequence of string t if deleting some number of characters from t (possibly 0) results in the string s.
//
// Example 1:
//
// Input: str1 = "abac", str2 = "cab"
// Output: "cabac"
// Explanation:
// str1 = "abac" is a subsequence of "cabac" because we can delete the first "c".
// str2 = "cab" is a subsequence of "cabac" because we can delete the last "ac".
// The answer provided is the shortest such string that satisfies these properties.
//
// Example 2:
//
// Input: str1 = "aaaaaaaa", str2 = "aaaaaaaa"
// Output: "aaaaaaaa"
//
// Constraints:
//
// - 1 <= str1.length, str2.length <= 1000
// - str1 and str2 consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        fn eval(s1: &Vec<char>, s2: &Vec<char>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
            if dp[i][j] != -1 {
                return dp[i][j];
            }

            if i == 0 || j == 0 {
                dp[i][j] = i as i32 + j as i32;
                return dp[i][j];
            }

            eval(s1, s2, i - 1, j - 1, dp);
            eval(s1, s2, i - 1, j, dp);
            eval(s1, s2, i, j - 1, dp);

            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
                return dp[i][j];
            }

            dp[i][j] = dp[i - 1][j - 1] + 2;
            dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
            dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);

            dp[i][j]
        }

        fn construct(s1: &Vec<char>, s2: &Vec<char>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>, ret: &mut Vec<char>) {
            if i == 0 {
                ret[..j].copy_from_slice(&s2[..j]);
                return;
            }

            if j == 0 {
                ret[..i].copy_from_slice(&s1[..i]);
                return;
            }

            let k = dp[i][j] as usize - 1;

            if s1[i - 1] == s2[j - 1] {
                ret[k] = s1[i - 1];
                construct(s1, s2, i - 1, j - 1, dp, ret);
                return;
            }

            if dp[i][j] == dp[i - 1][j - 1] + 2 {
                ret[k] = s1[i - 1];
                ret[k - 1] = s2[j - 1];
                construct(s1, s2, i - 1, j - 1, dp, ret);
                return;
            }

            if dp[i][j] == dp[i - 1][j] + 1 {
                ret[k] = s1[i - 1];
                construct(s1, s2, i - 1, j, dp, ret);
                return;
            }

            ret[k] = s2[j - 1];
            construct(s1, s2, i, j - 1, dp, ret);
        }

        let (n1, n2) = (str1.len(), str2.len());
        let s1 = str1.chars().collect::<Vec<char>>();
        let s2 = str2.chars().collect::<Vec<char>>();

        let mut dp = vec![vec![-1; n2 + 1]; n1 + 1];

        let mut ret = vec![' '; eval(&s1, &s2, n1, n2, &mut dp) as usize];
        construct(&s1, &s2, n1, n2, &mut dp, &mut ret);

        ret.iter().collect()
    }
}

#[test]
fn test() {
    let cases = vec![("abac", "cab", "cabac"), ("aaaaaaaa", "aaaaaaaa", "aaaaaaaa")];
    for (str1, str2, expected) in cases {
        let str1 = str1.to_string();
        let str2 = str2.to_string();
        assert_eq!(Solution::shortest_common_supersequence(str1, str2), expected);
    }
}
