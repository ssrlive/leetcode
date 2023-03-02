#![allow(dead_code)]

/*

// 2047. Number of Valid Words in a Sentence
// https://leetcode.com/problems/number-of-valid-words-in-a-sentence/
// https://leetcode.cn/problems/number-of-valid-words-in-a-sentence/
//
// Easy
//
// A sentence consists of lowercase letters ('a' to 'z'), digits ('0' to '9'), hyphens ('-'),
// punctuation marks ('!', '.', and ','), and spaces (' ') only.
// Each sentence can be broken down into one or more tokens separated by one or more spaces ' '.

A token is a valid word if all three of the following are true:

    It only contains lowercase letters, hyphens, and/or punctuation (no digits).
    There is at most one hyphen '-'. If present, it must be surrounded by lowercase characters ("a-b" is valid, but "-ab" and "ab-" are not valid).
    There is at most one punctuation mark. If present, it must be at the end of the token ("ab,", "cd!", and "." are valid, but "a!b" and "c.," are not valid).

Examples of valid words include "a-b.", "afad", "ba-c", "a!", and "!".

Given a string sentence, return the number of valid words in sentence.

Example 1:

Input: sentence = "cat and  dog"
Output: 3
Explanation: The valid words in the sentence are "cat", "and", and "dog".

Example 2:

Input: sentence = "!this  1-s b8d!"
Output: 0
Explanation: There are no valid words in the sentence.
"!this" is invalid because it starts with a punctuation mark.
"1-s" and "b8d" are invalid because they contain digits.

Example 3:

Input: sentence = "alice and  bob are playing stone-game10"
Output: 5
Explanation: The valid words in the sentence are "alice", "and", "bob", "are", and "playing".
"stone-game10" is invalid because it contains digits.

Constraints:

    1 <= sentence.length <= 1000
    sentence only contains lowercase English letters, digits, ' ', '-', '!', '.', and ','.
    There will be at least 1 token.
*/

struct Solution;

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let mut res = 0;
        'outer: for word in sentence.split_ascii_whitespace() {
            let l = word.len();
            let mut find_hyphen = false;
            let mut find_punct = false;
            let mut check_next_alpha = false;

            for (ind, c) in word.chars().enumerate() {
                let is_hyphen = c == '-';
                let is_punct = "!.,".contains(c);

                if c.is_ascii_digit()
                    || (is_hyphen && (find_hyphen || ind == 0 || ind == l - 1))
                    || (is_punct && (find_punct || ind != l - 1))
                    || (check_next_alpha && !c.is_alphabetic())
                {
                    continue 'outer;
                } else if is_hyphen {
                    find_hyphen = true;
                    check_next_alpha = true;
                } else if is_punct {
                    find_punct = true;
                } else if check_next_alpha {
                    check_next_alpha = false;
                }
            }
            res += 1;
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        ("cat and  dog", 3),
        ("!this  1-s b8d!", 0),
        ("alice and  bob are playing stone-game10", 5),
    ];
    for (sentence, expected) in cases {
        assert_eq!(Solution::count_valid_words(sentence.to_string()), expected);
    }
}
