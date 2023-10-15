#![allow(dead_code)]

// 2746. Decremental String Concatenation
// https://leetcode.com/problems/decremental-string-concatenation/
// https://leetcode.cn/problems/decremental-string-concatenation/
//
// Medium
//
// You are given a 0-indexed array words containing n strings.
//
// Let's define a join operation join(x, y) between two strings x and y as concatenating them into xy.
// However, if the last character of x is equal to the first character of y, one of them is deleted.
//
// For example join("ab", "ba") = "aba" and join("ab", "cde") = "abcde".
//
// You are to perform n - 1 join operations. Let str0 = words[0]. Starting from i = 1 up to i = n - 1,
// for the ith operation, you can do one of the following:
//
//     Make stri = join(stri - 1, words[i])
//     Make stri = join(words[i], stri - 1)
//
// Your task is to minimize the length of strn - 1.
//
// Return an integer denoting the minimum possible length of strn - 1.
//
// Example 1:
//
// Input: words = ["aa","ab","bc"]
// Output: 4
// Explanation: In this example, we can perform join operations in the following order to minimize the length of str2:
// str0 = "aa"
// str1 = join(str0, "ab") = "aab"
// str2 = join(str1, "bc") = "aabc"
// It can be shown that the minimum possible length of str2 is 4.
//
// Example 2:
//
// Input: words = ["ab","b"]
// Output: 2
// Explanation: In this example, str0 = "ab", there are two ways to get str1:
// join(str0, "b") = "ab" or join("b", str0) = "bab".
// The first string, "ab", has the minimum length. Hence, the answer is 2.
//
// Example 3:
//
// Input: words = ["aaa","c","aba"]
// Output: 6
// Explanation: In this example, we can perform join operations in the following order to minimize the length of str2:
// str0 = "aaa"
// str1 = join(str0, "c") = "aaac"
// str2 = join("aba", str1) = "abaaac"
// It can be shown that the minimum possible length of str2 is 6.
//
// Constraints:
//
//     1 <= words.length <= 1000
//     1 <= words[i].length <= 50
//     Each character in words[i] is an English lowercase letter
//

struct Solution;

impl Solution {
    pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
        let mut dp = vec![vec![vec![-1; 26]; 26]; words.len()];
        let word = words[0].as_bytes();
        let l = (word[0] - b'a') as usize;
        let r = (word[word.len() - 1] - b'a') as usize;
        Self::dfs(&words, &mut dp, 1, l, r, word.len() as i32)
    }

    fn dfs(words: &[String], dp: &mut [Vec<Vec<i32>>], i: usize, left: usize, right: usize, len: i32) -> i32 {
        if i == words.len() {
            return len;
        }

        if dp[i][left][right] == -1 {
            let word = words[i].as_bytes();
            let l = (word[0] - b'a') as usize;
            let r = (word[word.len() - 1] - b'a') as usize;
            let len_left = if r == left {
                len + word.len() as i32 - 1
            } else {
                len + word.len() as i32
            };
            let len_right = if l == right {
                len + word.len() as i32 - 1
            } else {
                len + word.len() as i32
            };
            dp[i][left][right] = Self::dfs(words, dp, i + 1, l, right, len_left).min(Self::dfs(words, dp, i + 1, left, r, len_right)) - len;
        }
        dp[i][left][right] + len
    }
}

#[test]
fn test() {
    let words = vec!["aa", "ab", "bc"].into_iter().map(|s| s.to_string()).collect();
    let res = 4;
    assert_eq!(Solution::minimize_concatenated_length(words), res);

    let words = vec!["ab", "b"].into_iter().map(|s| s.to_string()).collect();
    let res = 2;
    assert_eq!(Solution::minimize_concatenated_length(words), res);

    let words = vec!["aaa", "c", "aba"].into_iter().map(|s| s.to_string()).collect();
    let res = 6;
    assert_eq!(Solution::minimize_concatenated_length(words), res);
}
