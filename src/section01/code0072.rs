#![allow(dead_code)]

// 72. Edit Distance
// https://leetcode.com/problems/edit-distance/
// https://leetcode.cn/problems/edit-distance/
//
// Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
//
// You have the following three operations permitted on a word:
//
// Insert a character
// Delete a character
// Replace a character
//
// Example 1:
//
// Input: word1 = "horse", word2 = "ros"
// Output: 3
// Explanation:
// horse -> rorse (replace 'h' with 'r')
// rorse -> rose (remove 'r')
// rose -> ros (remove 'e')
//
// Example 2:
//
// Input: word1 = "intention", word2 = "execution"
// Output: 5
// Explanation:
// intention -> inention (remove 't')
// inention -> enention (replace 'i' with 'e')
// enention -> exention (replace 'n' with 'x')
// exention -> exection (replace 'n' with 'c')
// exection -> execution (insert 'u')
//
// Constraints:
//
// - 0 <= word1.length, word2.length <= 500
// - word1 and word2 consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let len1 = word1.len();
        let len2 = word2.len();
        if len1 == 0 || len2 == 0 {
            return (len1 | len2) as i32;
        };

        let mut a = vec![vec![0i32; len2 + 1]; len1 + 1];
        for i in 0..=len1 {
            for j in 0..=len2 {
                a[i][j] = if i == 0 || j == 0 {
                    (j | i) as i32
                } else if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                    a[i - 1][j - 1]
                } else {
                    // insert, delete or replace
                    1 + a[i - 1][j - 1].min(a[i - 1][j].min(a[i][j - 1]))
                }
            }
        }
        a[len1][len2]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
    assert_eq!(
        Solution::min_distance("intention".to_string(), "execution".to_string()),
        5
    );
}
