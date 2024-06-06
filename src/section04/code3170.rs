#![allow(dead_code)]

// 3170. Lexicographically Minimum String After Removing Stars
// https://leetcode.com/problems/lexicographically-minimum-string-after-removing-stars/
// https://leetcode.cn/problems/lexicographically-minimum-string-after-removing-stars/
//
// Medium
//
// You are given a string s. It may contain any number of '*' characters. Your task is to remove all '*' characters.
//
// While there is a '*', do the following operation:
//
// Delete the leftmost '*' and the smallest non-'*' character to its left. If there are several smallest characters, you can delete any of them.
// Return the lexicographically smallest resulting string after removing all '*' characters.
//
// Example 1:
//
// Input: s = "aaba*"
//
// Output: "aab"
//
// Explanation:
//
// We should delete one of the 'a' characters with '*'. If we choose s[3], s becomes the lexicographically smallest.
//
// Example 2:
//
// Input: s = "abc"
//
// Output: "abc"
//
// Explanation:
//
// There is no '*' in the string.
//
// Constraints:
//
// 1 <= s.length <= 10^5
// s consists only of lowercase English letters and '*'.
// The input is generated such that it is possible to delete all '*' characters.
//

struct Solution;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
        let mut chars: Vec<char> = s.chars().collect();
        let mut to_replace = Vec::new();

        for (i, c) in chars.iter().enumerate() {
            if *c == '*' {
                for p in pos.iter_mut() {
                    if let Some(last) = p.pop() {
                        to_replace.push(last);
                        break;
                    }
                }
            } else {
                let index = (*c as usize) - ('a' as usize);
                pos[index].push(i);
            }
        }

        for i in to_replace {
            chars[i] = '*';
        }

        chars.into_iter().filter(|&c| c != '*').collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::clear_stars("aaba*".to_string()), "aab");
    assert_eq!(Solution::clear_stars("abc".to_string()), "abc");
    assert_eq!(Solution::clear_stars("a*".to_string()), "");
}
