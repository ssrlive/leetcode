#![allow(dead_code)]

// 17. Letter Combinations of a Phone Number
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// https://leetcode.cn/problems/letter-combinations-of-a-phone-number/
//
// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
//
// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
//
// Example 1:
//
// Input: digits = "23"
// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
//
// Example 2:
//
// Input: digits = ""
// Output: []
//
// Example 3:
//
// Input: digits = "2"
// Output: ["a","b","c"]
//
// Constraints:
//
// - 0 <= digits.length <= 4
// - digits[i] is a digit in the range ['2', '9'].
//

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        digits.chars().fold(
            match digits.is_empty() {
                true => Vec::new(),
                false => vec![String::new()],
            },
            |acc, digit| {
                let letters = match digit {
                    '2' => 'a'..='c',
                    '3' => 'd'..='f',
                    '4' => 'g'..='i',
                    '5' => 'j'..='l',
                    '6' => 'm'..='o',
                    '7' => 'p'..='s',
                    '8' => 't'..='v',
                    '9' => 'w'..='z',
                    _ => return Vec::new(),
                };
                acc.iter()
                    .flat_map(|x| {
                        std::iter::repeat(x)
                            .zip(letters.clone())
                            .map(|(x, y)| x.chars().chain(std::iter::once(y)).collect::<String>())
                    })
                    .collect::<Vec<String>>()
            },
        )
    }
}

#[test]
fn test() {
    let cases = vec![
        ("23", vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]),
        ("", Vec::<&str>::new()),
        ("2", vec!["a", "b", "c"]),
    ];
    for (digits, expected) in cases {
        assert_eq!(Solution::letter_combinations(digits.to_string()), expected);
    }
}
