#![allow(dead_code)]

// 1446. Consecutive Characters
// https://leetcode.com/problems/consecutive-characters/
// https://leetcode.cn/problems/consecutive-characters/
//
// Easy
//
// The power of the string is the maximum length of a non-empty substring that contains only one unique character.
//
// Given a string s, return the power of s.
//
// Example 1:
//
// Input: s = "leetcode"
// Output: 2
// Explanation: The substring "ee" is of length 2 with the character 'e' only.
//
// Example 2:
//
// Input: s = "abbcccddddeeeeedcba"
// Output: 5
// Explanation: The substring "eeeee" is of length 5 with the character 'e' only.
//
// Constraints:
//
// -    1 <= s.length <= 500
// -    s consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max = 0;
        let mut count = 0;
        let mut prev = ' ';
        for c in s.chars() {
            if c == prev {
                count += 1;
            } else {
                count = 1;
                prev = c;
            }
            if count > max {
                max = count;
            }
        }
        max
    }
}

#[test]
fn test() {
    let cases = vec![
        ("leetcode", 2),
        ("abbcccddddeeeeedcba", 5),
        ("triplepillooooow", 5),
        ("hooraaaaaaaaaaay", 11),
        ("tourist", 1),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::max_power(s.to_string()), expected);
    }
}
