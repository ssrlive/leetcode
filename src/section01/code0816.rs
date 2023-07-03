#![allow(dead_code)]

// 816. Ambiguous Coordinates
// https://leetcode.com/problems/ambiguous-coordinates/
// https://leetcode.cn/problems/ambiguous-coordinates/
//
// We had some 2-dimensional coordinates, like "(1, 3)" or "(2, 0.5)". Then, we removed all commas, decimal points, and spaces and ended up with the string s.
//
// For example, "(1, 3)" becomes s = "(13)" and "(2, 0.5)" becomes s = "(205)".
//
// Return a list of strings representing all possibilities for what our original coordinates could have been.
//
// Our original representation never had extraneous zeroes, so we never started with numbers like "00", "0.0", "0.00", "1.0", "001", "00.01",
// or any other number that can be represented with fewer digits. Also,
// a decimal point within a number never occurs without at least one digit occurring before it, so we never started with numbers like ".1".
//
// The final answer list can be returned in any order. All coordinates in the final answer have exactly one space between them (occurring after the comma.)
//
// Example 1:
//
// Input: s = "(123)"
// Output: ["(1, 2.3)","(1, 23)","(1.2, 3)","(12, 3)"]
//
// Example 2:
//
// Input: s = "(0123)"
// Output: ["(0, 1.23)","(0, 12.3)","(0, 123)","(0.1, 2.3)","(0.1, 23)","(0.12, 3)"]
// Explanation: 0.0, 00, 0001 or 00.01 are not allowed.
//
// Example 3:
//
// Input: s = "(00011)"
// Output: ["(0, 0.011)","(0.001, 1)"]
//
// Constraints:
//
// - 4 <= s.length <= 12
// - s[0] == '(' and s[s.length - 1] == ')'.
// - The rest of s are digits.
//

struct Solution;

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let s = s.chars().skip(1).take(s.len() - 2).collect::<Vec<_>>();
        let candidates = |v: &[char]| -> Vec<String> {
            (0..v.len())
                .filter_map(|i| {
                    let s: String = if i == 0 {
                        v.iter().collect()
                    } else {
                        v[..i].iter().chain(std::iter::once(&'.')).chain(v[i..].iter()).collect()
                    };
                    if (s != "0" && s.starts_with('0') && !s.starts_with("0.")) || (s.contains('.') && s.ends_with('0')) {
                        None
                    } else {
                        Some(s)
                    }
                })
                .collect()
        };
        let mut answer = Vec::new();
        for i in 1..s.len() {
            for x in &candidates(&s[..i]) {
                for y in &candidates(&s[i..]) {
                    answer.push(format!("({x}, {y})"));
                }
            }
        }
        answer
    }
}

#[test]
fn test() {
    let s = "(123)".to_string();
    let mut result = Solution::ambiguous_coordinates(s);
    result.sort();
    assert_eq!(result, vec!["(1, 2.3)", "(1, 23)", "(1.2, 3)", "(12, 3)"]);

    let s = "(0123)".to_string();
    let mut result = Solution::ambiguous_coordinates(s);
    result.sort();
    assert_eq!(
        result,
        vec!["(0, 1.23)", "(0, 12.3)", "(0, 123)", "(0.1, 2.3)", "(0.1, 23)", "(0.12, 3)"]
    );

    let s = "(00011)".to_string();
    let mut result = Solution::ambiguous_coordinates(s);
    result.sort();
    assert_eq!(result, vec!["(0, 0.011)", "(0.001, 1)"]);

    let s = "(100)".to_string();
    let result = Solution::ambiguous_coordinates(s);
    assert_eq!(result, vec!["(10, 0)"]);
}
