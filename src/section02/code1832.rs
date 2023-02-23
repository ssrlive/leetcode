#![allow(dead_code)]

/*

// 1832. Check if the Sentence Is Pangram
// https://leetcode.com/problems/check-if-the-sentence-is-pangram/
// https://leetcode.cn/problems/check-if-the-sentence-is-pangram/
//
// Easy
//
// A pangram is a sentence where every letter of the English alphabet appears at least once.

Given a string sentence containing only lowercase English letters, return true if sentence is a pangram, or false otherwise.

Example 1:

Input: sentence = "thequickbrownfoxjumpsoverthelazydog"
Output: true
Explanation: sentence contains at least one of every letter of the English alphabet.

Example 2:

Input: sentence = "leetcode"
Output: false

Constraints:

    1 <= sentence.length <= 1000
    sentence consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        const N_LETTERS: usize = (b'z' - b'a' + 1) as _;
        sentence
            .bytes()
            .scan(0_u32, |bitset, b| {
                *bitset |= 1 << (b - b'a');
                Some(*bitset)
            })
            .any(|bitset| bitset == (1 << N_LETTERS) - 1)
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("thequickbrownfoxjumpsoverthelazydog", true),
        ("leetcode", false),
    ];
    for (sentence, expected) in cases {
        assert_eq!(Solution::check_if_pangram(sentence.to_string()), expected);
    }
}
