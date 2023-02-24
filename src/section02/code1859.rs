#![allow(dead_code)]

/*

// 1859. Sorting the Sentence
// https://leetcode.com/problems/sorting-the-sentence/
// https://leetcode.cn/problems/sorting-the-sentence/
//
// Easy
//
// A sentence is a list of words that are separated by a single space with no leading or trailing spaces. Each word consists of lowercase and uppercase English letters.

A sentence can be shuffled by appending the 1-indexed word position to each word then rearranging the words in the sentence.

    For example, the sentence "This is a sentence" can be shuffled as "sentence4 a3 is2 This1" or "is2 sentence4 This1 a3".

Given a shuffled sentence s containing no more than 9 words, reconstruct and return the original sentence.

Example 1:

Input: s = "is2 sentence4 This1 a3"
Output: "This is a sentence"
Explanation: Sort the words in s to their original positions "This1 is2 a3 sentence4", then remove the numbers.

Example 2:

Input: s = "Myself2 Me1 I4 and3"
Output: "Me Myself and I"
Explanation: Sort the words in s to their original positions "Me1 Myself2 and3 I4", then remove the numbers.

Constraints:

    2 <= s.length <= 200
    s consists of lowercase and uppercase English letters, spaces, and digits from 1 to 9.
    The number of words in s is between 1 and 9.
    The words in s are separated by a single space.
    s contains no leading or trailing spaces.
*/

struct Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words: Vec<_> = s.split_ascii_whitespace().collect();
        words.sort_by_key(|w| w.chars().last().unwrap());
        words
            .iter()
            .map(|w| {
                w.chars()
                    .map(|mut c| {
                        if c.is_ascii_digit() {
                            c = ' ';
                        }
                        c
                    })
                    .collect::<String>()
            })
            .collect::<String>()
            .trim()
            .to_owned()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("is2 sentence4 This1 a3", "This is a sentence"),
        ("Myself2 Me1 I4 and3", "Me Myself and I"),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::sort_sentence(s.to_owned()), expected);
    }
}
