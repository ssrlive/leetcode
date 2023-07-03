#![allow(dead_code)]

/*

// 1771. Maximize Palindrome Length From Subsequences
Hard
459
8
Companies

You are given two strings, word1 and word2. You want to construct a string in the following manner:

    Choose some non-empty subsequence subsequence1 from word1.
    Choose some non-empty subsequence subsequence2 from word2.
    Concatenate the subsequences: subsequence1 + subsequence2, to make the string.

Return the length of the longest palindrome that can be constructed in the described manner. If no palindromes can be constructed, return 0.

A subsequence of a string s is a string that can be made by deleting some (possibly none) characters from s without changing the order of the remaining characters.

A palindrome is a string that reads the same forward as well as backward.

Example 1:

Input: word1 = "cacb", word2 = "cbba"
Output: 5
Explanation: Choose "ab" from word1 and "cba" from word2 to make "abcba", which is a palindrome.

Example 2:

Input: word1 = "ab", word2 = "ab"
Output: 3
Explanation: Choose "ab" from word1 and "a" from word2 to make "aba", which is a palindrome.

Example 3:

Input: word1 = "aa", word2 = "bb"
Output: 0
Explanation: You cannot construct a palindrome from the described method, so return 0.

Constraints:

    1 <= word1.length, word2.length <= 1000
    word1 and word2 consist of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn longest_palindrome(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![-1; 2002]; 2002];
        let s = word1.clone() + &word2;
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        Solution::lcs(&s, 0, n - 1, &mut dp);
        let mut mp = std::collections::HashMap::new();
        for (i, c) in word1.chars().enumerate() {
            mp.entry(c).or_insert(i as i32);
        }
        let mut ans = 0;
        let mut m = std::collections::HashMap::new();
        let word2 = word2.chars().rev().collect::<Vec<_>>();
        for (i, c) in word2.iter().enumerate() {
            if mp.contains_key(c) && !m.contains_key(&c) {
                m.insert(c, true);
                let id1 = mp[c] + 1;
                let id2 = n as i32 - i as i32 - 2;
                if id1 >= 0 && id2 >= 0 {
                    ans = ans.max(2 + dp[id1 as usize][id2 as usize]);
                }
            }
        }
        ans
    }

    fn lcs(s: &[char], i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if i > j {
            dp[i][j] = 0;
            return 0;
        }
        if i == j {
            dp[i][j] = 1;
            return 1;
        }
        if dp[i][j] != -1 {
            return dp[i][j];
        }
        if s[i] == s[j] {
            let v = 2 + Solution::lcs(s, i + 1, j - 1, dp);
            dp[i][j] = v;
            return v;
        }
        let v1 = Solution::lcs(s, i + 1, j, dp);
        let v2 = Solution::lcs(s, i, j - 1, dp);
        dp[i][j] = v1.max(v2);
        dp[i][j]
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("cacb", "cbba", 5),
        ("ab", "ab", 3),
        ("aa", "bb", 0),
        ("a", "a", 2),
    ];
    for (word1, word2, expected) in cases {
        assert_eq!(Solution::longest_palindrome(word1.to_string(), word2.to_string()), expected);
    }
}
