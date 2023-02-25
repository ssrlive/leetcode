#![allow(dead_code)]

/*

// 1880. Check if Word Equals Summation of Two Words
// https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/
// https://leetcode.cn/problems/check-if-word-equals-summation-of-two-words/
//
// Easy
//
// The letter value of a letter is its position in the alphabet starting from 0 (i.e. 'a' -> 0, 'b' -> 1, 'c' -> 2, etc.).

The numerical value of some string of lowercase English letters s is the concatenation of the letter values of each letter in s, which is then converted into an integer.

    For example, if s = "acb", we concatenate each letter's letter value, resulting in "021". After converting it, we get 21.

You are given three strings firstWord, secondWord, and targetWord, each consisting of lowercase English letters 'a' through 'j' inclusive.

Return true if the summation of the numerical values of firstWord and secondWord equals the numerical value of targetWord, or false otherwise.

Example 1:

Input: firstWord = "acb", secondWord = "cba", targetWord = "cdb"
Output: true
Explanation:
The numerical value of firstWord is "acb" -> "021" -> 21.
The numerical value of secondWord is "cba" -> "210" -> 210.
The numerical value of targetWord is "cdb" -> "231" -> 231.
We return true because 21 + 210 == 231.

Example 2:

Input: firstWord = "aaa", secondWord = "a", targetWord = "aab"
Output: false
Explanation:
The numerical value of firstWord is "aaa" -> "000" -> 0.
The numerical value of secondWord is "a" -> "0" -> 0.
The numerical value of targetWord is "aab" -> "001" -> 1.
We return false because 0 + 0 != 1.

Example 3:

Input: firstWord = "aaa", secondWord = "a", targetWord = "aaaa"
Output: true
Explanation:
The numerical value of firstWord is "aaa" -> "000" -> 0.
The numerical value of secondWord is "a" -> "0" -> 0.
The numerical value of targetWord is "aaaa" -> "0000" -> 0.
We return true because 0 + 0 == 0.

Constraints:

    1 <= firstWord.length, secondWord.length, targetWord.length <= 8
    firstWord, secondWord, and targetWord consist of lowercase English letters from 'a' to 'j' inclusive.
*/

struct Solution;

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let calc_num_str = |s: String| {
            s.as_bytes().iter().enumerate().fold(0, |acc, (ind, b)| -> u32 {
                acc + (b - b'a') as u32 * 10u32.pow((s.len() - ind - 1) as u32)
            })
        };
        calc_num_str(first_word) + calc_num_str(second_word) == calc_num_str(target_word)
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("acb", "cba", "cdb", true),
        ("aaa", "a", "aab", false),
        ("aaa", "a", "aaaa", true),
    ];
    for (f, s, t, ex) in cases {
        assert_eq!(Solution::is_sum_equal(f.to_string(), s.to_string(), t.to_string()), ex);
    }
}
