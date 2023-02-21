#![allow(dead_code)]

/*

// 1790. Check if One String Swap Can Make Strings Equal
Easy
826
45
Companies

You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.

Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.

Example 1:

Input: s1 = "bank", s2 = "kanb"
Output: true
Explanation: For example, swap the first character with the last character of s2 to make "bank".

Example 2:

Input: s1 = "attack", s2 = "defend"
Output: false
Explanation: It is impossible to make them equal with one string swap.

Example 3:

Input: s1 = "kelb", s2 = "kelb"
Output: true
Explanation: The two strings are already equal, so no string swap operation is required.

Constraints:

    1 <= s1.length, s2.length <= 100
    s1.length == s2.length
    s1 and s2 consist of only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let f = |(c1, c2): &(char, char)| c1 != c2;
        let diff = s1.chars().zip(s2.chars()).filter(f).take(3).collect::<Vec<_>>();
        diff.is_empty() || (diff.len() == 2 && diff[0].0 == diff[1].1 && diff[0].1 == diff[1].0)
    }
}

#[test]
fn test() {
    let cases = vec![
        ("bank", "kanb", true),
        ("attack", "defend", false),
        ("kelb", "kelb", true),
        ("abcd", "dcba", false),
        ("aa", "aa", true),
        ("ab", "ba", true),
        ("ab", "ab", true),
        ("abcd", "badc", false),
    ];
    for (s1, s2, expected) in cases {
        assert_eq!(Solution::are_almost_equal(s1.to_string(), s2.to_string()), expected);
    }
}
