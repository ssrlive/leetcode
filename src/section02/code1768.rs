#![allow(dead_code)]

/*

// 1768. Merge Strings Alternately
Easy
920
17
Companies

You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.

Return the merged string.

Example 1:

Input: word1 = "abc", word2 = "pqr"
Output: "apbqcr"
Explanation: The merged string will be merged as so:
word1:  a   b   c
word2:    p   q   r
merged: a p b q c r

Example 2:

Input: word1 = "ab", word2 = "pqrs"
Output: "apbqrs"
Explanation: Notice that as word2 is longer, "rs" is appended to the end.
word1:  a   b
word2:    p   q   r   s
merged: a p b q   r   s

Example 3:

Input: word1 = "abcd", word2 = "pq"
Output: "apbqcd"
Explanation: Notice that as word1 is longer, "cd" is appended to the end.
word1:  a   b   c   d
word2:    p   q
merged: a p b q c   d

Constraints:

    1 <= word1.length, word2.length <= 100
    word1 and word2 consist of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let len1: usize = word1.len();
        let len2: usize = word2.len();
        let chs1: Vec<char> = word1.chars().collect();
        let chs2: Vec<char> = word2.chars().collect();
        let mut idx1: usize = 0;
        let mut idx2: usize = 0;
        let mut ans: String = "".to_owned();
        while idx1 < len1 || idx2 < len2 {
            if idx1 < len1 {
                ans.push(chs1[idx1]);
                idx1 += 1;
            }
            if idx2 < len2 {
                ans.push(chs2[idx2]);
                idx2 += 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![("abc", "pqr", "apbqcr"), ("ab", "pqrs", "apbqrs"), ("abcd", "pq", "apbqcd")];
    for (word1, word2, expected) in cases {
        assert_eq!(Solution::merge_alternately(word1.to_owned(), word2.to_owned()), expected);
    }
}
