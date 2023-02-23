#![allow(dead_code)]

/*

// 1805. Number of Different Integers in a String
// https://leetcode.com/problems/number-of-different-integers-in-a-string/
// https://leetcode.cn/problems/number-of-different-integers-in-a-string/
//
// Easy
//
// You are given a string word that consists of digits and lowercase English letters.

You will replace every non-digit character with a space. For example, "a123bc34d8ef34" will become " 123  34 8  34". Notice that you are left with some integers that are separated by at least one space: "123", "34", "8", and "34".

Return the number of different integers after performing the replacement operations on word.

Two integers are considered different if their decimal representations without any leading zeros are different.

Example 1:

Input: word = "a123bc34d8ef34"
Output: 3
Explanation: The three different integers are "123", "34", and "8". Notice that "34" is only counted once.

Example 2:

Input: word = "leet1234code234"
Output: 2

Example 3:

Input: word = "a1b01c001"
Output: 1
Explanation: The three integers "1", "01", and "001" all represent the same integer because
the leading zeros are ignored when comparing their decimal values.

Constraints:

    1 <= word.length <= 1000
    word consists of digits and lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        use std::collections::HashSet;
        word.split(char::is_alphabetic)
            .filter(|s| !s.is_empty())
            .map(|x| x.trim_start_matches('0'))
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("a123bc34d8ef34", 3),
        ("leet1234code234", 2),
        ("a1b01c001", 1),
    ];
    for (word, expected) in cases {
        assert_eq!(Solution::num_different_integers(word.to_string()), expected);
    }
}
