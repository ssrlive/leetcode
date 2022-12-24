#![allow(dead_code)]

// 97. Interleaving String
// https://leetcode.com/problems/interleaving-string/
// https://leetcode.cn/problems/interleaving-string/
//
// Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.
//
// An interleaving of two strings s and t is a configuration where s and t are divided into n and m substrings respectively, such that:
//
// s = s1 + s2 + ... + sn
// t = t1 + t2 + ... + tm
// |n - m| <= 1
// The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...
// Note: a + b is the concatenation of strings a and b.
//
// Example 1:
//
// Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
// Output: true
// Explanation: One way to obtain s3 is:
// Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
// Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" = "aadbbcbcac".
// Since s3 can be obtained by interleaving s1 and s2, we return true.
//
// Example 2:
//
// Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
// Output: false
// Explanation: Notice how it is impossible to interleave s2 with any other string to obtain s3.
//
// Example 3:
//
// Input: s1 = "", s2 = "", s3 = ""
// Output: true
//
// Constraints:
//
// - 0 <= s1.length, s2.length <= 100
// - 0 <= s3.length <= 200
// - s1, s2, and s3 consist of lowercase English letters.
//
// Follow up: Could you solve it using only O(s2.length) additional memory space?
//

struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if s3.len() != m + n {
            return false;
        }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp_prev = std::iter::once(true)
            .chain(s2.iter().zip(s3.iter()).scan(true, |ok, (b2, b3)| {
                *ok &= *b2 == *b3;
                Some(*ok)
            }))
            .collect::<Vec<_>>();
        let mut dp_curr = vec![false; n + 1];
        for i in 0..m {
            dp_curr[0] = dp_prev[0] && s1[i] == s3[i];
            for j in 1..=n {
                dp_curr[j] = (dp_prev[j] && s1[i] == s3[i + j]) || (dp_curr[j - 1] && s2[j - 1] == s3[i + j]);
            }
            std::mem::swap(&mut dp_curr, &mut dp_prev);
        }
        dp_prev[n]
    }
}

#[test]
fn test() {
    let cases = vec![
        ("aabcc", "dbbca", "aadbbcbcac", true),
        ("aabcc", "dbbca", "aadbbbaccc", false),
        ("", "", "", true),
        ("a", "b", "ab", true),
        ("a", "b", "ba", true),
        ("a", "b", "a", false),
    ];
    let cases = cases
        .into_iter()
        .map(|(s1, s2, s3, expected)| (s1.to_string(), s2.to_string(), s3.to_string(), expected))
        .collect::<Vec<_>>();
    for (s1, s2, s3, expected) in cases {
        assert_eq!(Solution::is_interleave(s1, s2, s3), expected);
    }
}
