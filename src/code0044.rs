#![allow(dead_code)]

// 44. Wildcard Matching
// https://leetcode.com/problems/wildcard-matching/
// https://leetcode.cn/problems/wildcard-matching/
//
// Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*' where:
//
// '?' Matches any single character.
// '*' Matches any sequence of characters (including the empty sequence).
// The matching should cover the entire input string (not partial).
//
// Example 1:
//
// Input: s = "aa", p = "a"
// Output: false
// Explanation: "a" does not match the entire string "aa".
//
// Example 2:
//
// Input: s = "aa", p = "*"
// Output: true
// Explanation: '*' matches any sequence.
//
// Example 3:
//
// Input: s = "cb", p = "?a"
// Output: false
// Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
//
// Constraints:
//
// - 0 <= s.length, p.length <= 2000
// - s contains only lowercase English letters.
// - p contains only lowercase English letters, '?' or '*'.
//

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        enum Pattern {
            SingleAny,
            ManyOrNone,
            Single(u8),
            Empty,
        }
        type DpCacheMatrix = Vec<Vec<Option<bool>>>;

        // a patternsize+1 x candidatesize+1 cache
        let dp: &mut DpCacheMatrix = &mut vec![vec!(None; s.len() + 1); p.len() + 1];

        fn is_match_bytes(s: &[u8], p: &[u8], dp: &mut DpCacheMatrix) -> bool {
            if let Some(b) = dp[p.len()][s.len()] {
                return b;
            };

            let res = match parse(p) {
                Pattern::Empty => s.is_empty(),
                Pattern::ManyOrNone if s.is_empty() => is_match_bytes(s, &p[1..], dp),
                Pattern::ManyOrNone => {
                    is_match_single(s, Pattern::SingleAny, p, dp)
                        || is_match_bytes(&s[1..], p, dp)
                        || is_match_bytes(s, &p[1..], dp)
                }
                single => is_match_single(s, single, p, dp),
            };

            dp[p.len()][s.len()] = Some(res);

            res
        }

        fn is_match_single(s: &[u8], token: Pattern, p: &[u8], dp: &mut DpCacheMatrix) -> bool {
            if let Some(b) = dp[p.len()][s.len()] {
                return b;
            };

            match s.split_first() {
                Some((head_s, tail_s)) => match token {
                    Pattern::Single(c) if *head_s == c => is_match_bytes(tail_s, &p[1..], dp),
                    Pattern::SingleAny => is_match_bytes(tail_s, &p[1..], dp),
                    _ => false,
                },
                None => p.is_empty(),
            }
        }

        fn parse(pattern: &[u8]) -> Pattern {
            match pattern.first() {
                Some(b'?') => Pattern::SingleAny,
                Some(b'*') => Pattern::ManyOrNone,
                Some(c) => Pattern::Single(*c),
                _ => Pattern::Empty,
            }
        }

        return is_match_bytes(s.as_bytes(), p.as_bytes(), dp);
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
    assert_eq!(Solution::is_match("cb".to_string(), "?a".to_string()), false);
    assert_eq!(Solution::is_match("adceb".to_string(), "*a*b".to_string()), true);
    assert_eq!(Solution::is_match("acdcb".to_string(), "a*c?b".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "*".to_string()), true);
    assert_eq!(Solution::is_match("".to_string(), "******".to_string()), true);
    assert_eq!(Solution::is_match("".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a*".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a**".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a***".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a****".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a*****".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a******".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a*******".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a********".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a*********".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a**********".to_string()), false);
    assert_eq!(Solution::is_match("".to_string(), "a***********".to_string()), false);
}
