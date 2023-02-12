#![allow(dead_code)]

/*

// 1585. Check If String Is Transformable With Substring Sort Operations
Hard
387
7
Companies

Given two strings s and t, transform string s into string t using the following operation any number of times:

    Choose a non-empty substring in s and sort it in place so the characters are in ascending order.
        For example, applying the operation on the underlined substring in "14234" results in "12344".

Return true if it is possible to transform s into t. Otherwise, return false.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: s = "84532", t = "34852"
Output: true
Explanation: You can transform s into t using the following sort operations:
"84532" (from index 2 to 3) -> "84352"
"84352" (from index 0 to 2) -> "34852"

Example 2:

Input: s = "34521", t = "23415"
Output: true
Explanation: You can transform s into t using the following sort operations:
"34521" -> "23451"
"23451" -> "23415"

Example 3:

Input: s = "12345", t = "12435"
Output: false

Constraints:

    s.length == t.length
    1 <= s.length <= 105
    s and t consist of only digits.
*/

struct Solution;

impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let s = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        let t = t
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        let mut pos = vec![vec![]; 10];
        for i in 0..s.len() {
            pos[s[i]].push(i);
        }
        for &item_t in &t {
            let c = item_t;
            if pos[c].is_empty() {
                return false;
            }
            let p = pos[c][0];
            for item in pos.iter().take(c) {
                if !item.is_empty() && item[0] < p {
                    return false;
                }
            }
            pos[c].remove(0);
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        ("84532", "34852", true),
        ("34521", "23415", true),
        ("12345", "12435", false),
    ];
    for (s, t, expected) in cases {
        assert_eq!(Solution::is_transformable(s.to_string(), t.to_string()), expected);
    }
}
