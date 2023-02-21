#![allow(dead_code)]

/*

// 1781. Sum of Beauty of All Substrings
Medium
552
124
Companies

The beauty of a string is the difference in frequencies between the most frequent and least frequent characters.

    For example, the beauty of "abaacc" is 3 - 1 = 2.

Given a string s, return the sum of beauty of all of its substrings.

Example 1:

Input: s = "aabcb"
Output: 5
Explanation: The substrings with non-zero beauty are ["aab","aabc","aabcb","abcb","bcb"], each with beauty equal to 1.

Example 2:

Input: s = "aabcbaa"
Output: 17

Constraints:

    1 <= s.length <= 500
    s consists of only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        fn beauty(s: &str) -> i32 {
            let mut counts = vec![0; 26];
            s.chars().for_each(|c| counts[c as usize - 'a' as usize] += 1);
            let (min, max) = counts
                .into_iter()
                .fold((i32::MAX, i32::MIN), |(min, max), value| match value {
                    0 => (min, max),
                    _ => (min.min(value), max.max(value)),
                });
            max - min
        }

        let mut sum = 0;
        for i in 0..s.len() - 1 {
            for j in i + 1..s.len() {
                sum += beauty(&s[i..=j]);
            }
        }
        sum
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("aabcb", 5),
        ("aabcbaa", 17),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::beauty_sum(s.to_string()), expected);
    }
}
