#![allow(dead_code)]

/*

// 1813. Sentence Similarity III
// https://leetcode.com/problems/sentence-similarity-iii/
// https://leetcode.cn/problems/sentence-similarity-iii/
//
// Medium
//
// A sentence is a list of words that are separated by a single space with no leading or trailing spaces. For example, "Hello World", "HELLO", "hello world hello world" are all sentences. Words consist of only uppercase and lowercase English letters.

Two sentences sentence1 and sentence2 are similar if it is possible to insert an arbitrary sentence (possibly empty) inside one of these sentences such that the two sentences become equal. For example, sentence1 = "Hello my name is Jane" and sentence2 = "Hello Jane" can be made equal by inserting "my name is" between "Hello" and "Jane" in sentence2.

Given two sentences sentence1 and sentence2, return true if sentence1 and sentence2 are similar. Otherwise, return false.

Example 1:

Input: sentence1 = "My name is Haley", sentence2 = "My Haley"
Output: true
Explanation: sentence2 can be turned to sentence1 by inserting "name is" between "My" and "Haley".

Example 2:

Input: sentence1 = "of", sentence2 = "A lot of words"
Output: false
Explanation: No single sentence can be inserted inside one of the sentences to make it equal to the other.

Example 3:

Input: sentence1 = "Eating right now", sentence2 = "Eating"
Output: true
Explanation: sentence2 can be turned to sentence1 by inserting "right now" at the end of the sentence.

Constraints:

    1 <= sentence1.length, sentence2.length <= 100
    sentence1 and sentence2 consist of lowercase and uppercase English letters and spaces.
    The words in sentence1 and sentence2 are separated by a single space.
*/

struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        use std::collections::*;
        let mut s1 = sentence1
            .split(' ')
            .map(|v| v.to_string())
            .collect::<VecDeque<String>>();
        let mut s2 = sentence2
            .split(' ')
            .map(|v| v.to_string())
            .collect::<VecDeque<String>>();
        if s1.len() < s2.len() {
            std::mem::swap(&mut s1, &mut s2);
        }
        while !s2.is_empty() {
            if s1[0] == s2[0] {
                s1.pop_front();
                s2.pop_front();
            } else {
                break;
            }
        }
        while !s2.is_empty() {
            if s1[s1.len() - 1] == s2[s2.len() - 1] {
                s1.pop_back();
                s2.pop_back();
            } else {
                break;
            }
        }
        s2.is_empty()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("My name is Haley", "My Haley", true),
        ("of", "A lot of words", false),
        ("Eating right now", "Eating", true),
        ("Luky", "Lucccky", false),
    ];
    for (s1, s2, expected) in cases {
        assert_eq!(
            Solution::are_sentences_similar(s1.to_string(), s2.to_string()),
            expected
        );
    }
}
