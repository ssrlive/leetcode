#![allow(dead_code)]

// 3302. Find the Lexicographically Smallest Valid Sequence
// https://leetcode.com/problems/find-the-lexicographically-smallest-valid-sequence/
// https://leetcode.cn/problems/find-the-lexicographically-smallest-valid-sequence/
//
// Medium
//
// You are given two strings word1 and word2.
//
// A string x is called almost equal to y if you can change at most one character in x to make it identical to y.
//
// A sequence of indices seq is called valid if:
//
//     The indices are sorted in ascending order.
//     Concatenating the characters at these indices in word1 in the same order results in a string that is almost equal to word2.
//
// Return an array of size word2.length representing the lexicographically smallest
// valid sequence of indices. If no such sequence of indices exists, return an empty array.
//
// Note that the answer must represent the lexicographically smallest array, not the corresponding string formed by those indices.
//
// Example 1:
//
// Input: word1 = "vbcca", word2 = "abc"
//
// Output: [0,1,2]
//
// Explanation:
//
// The lexicographically smallest valid sequence of indices is [0, 1, 2]:
//
//     Change word1[0] to 'a'.
//     word1[1] is already 'b'.
//     word1[2] is already 'c'.
//
// Example 2:
//
// Input: word1 = "bacdc", word2 = "abc"
//
// Output: [1,2,4]
//
// Explanation:
//
// The lexicographically smallest valid sequence of indices is [1, 2, 4]:
//
//     word1[1] is already 'a'.
//     Change word1[2] to 'b'.
//     word1[4] is already 'c'.
//
// Example 3:
//
// Input: word1 = "aaaaaa", word2 = "aaabc"
//
// Output: []
//
// Explanation:
//
// There is no valid sequence of indices.
//
// Example 4:
//
// Input: word1 = "abc", word2 = "ab"
//
// Output: [0,1]
//
// Constraints:
//
//     1 <= word2.length < word1.length <= 3 * 10^5
//     word1 and word2 consist only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let mut s1 = std::collections::HashSet::new();
        let mut sol = vec![];
        if Self::start(0, 0, 1, word1.as_bytes(), word2.as_bytes(), &mut sol, &mut s1) {
            sol.reverse();
            if sol.len() == word2.len() {
                return sol;
            }
        }
        vec![]
    }

    fn start(
        i: usize,
        j: usize,
        flag: i32,
        word1: &[u8],
        word2: &[u8],
        sol: &mut Vec<i32>,
        s1: &mut std::collections::HashSet<(usize, (usize, i32))>,
    ) -> bool {
        if j >= word2.len() {
            return true;
        }
        if i >= word1.len() {
            return false;
        }
        if s1.contains(&(i, (j, flag))) {
            return false;
        }

        let mut f1 = false;

        if word1[i] == word2[j] {
            f1 = Self::start(i + 1, j + 1, flag, word1, word2, sol, s1);
            if f1 {
                sol.push(i as i32);
            }
            return f1;
        } else {
            if flag > 0 {
                f1 = Self::start(i + 1, j + 1, 0, word1, word2, sol, s1);
                if f1 {
                    sol.push(i as i32);
                    return true;
                }
            }

            let mut i = i;
            while i < word1.len() && word1[i] != word2[j] {
                i += 1;
            }
            if i < word1.len() {
                f1 = Self::start(i + 1, j + 1, flag, word1, word2, sol, s1);
                if f1 {
                    sol.push(i as i32);
                    return true;
                }
            }
        }

        s1.insert((i, (j, flag)));
        f1
    }
}

#[test]
fn test() {
    let word1 = "vbcca".to_string();
    let word2 = "abc".to_string();
    let output = vec![0, 1, 2];
    assert_eq!(Solution::valid_sequence(word1, word2), output);

    let word1 = "bacdc".to_string();
    let word2 = "abc".to_string();
    let output = vec![1, 2, 4];
    assert_eq!(Solution::valid_sequence(word1, word2), output);

    let word1 = "aaaaaa".to_string();
    let word2 = "aaabc".to_string();
    let output = vec![];
    assert_eq!(Solution::valid_sequence(word1, word2), output);

    let word1 = "abc".to_string();
    let word2 = "ab".to_string();
    let output = vec![0, 1];
    assert_eq!(Solution::valid_sequence(word1, word2), output);
}
