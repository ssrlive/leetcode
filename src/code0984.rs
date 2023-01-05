#![allow(dead_code)]

// 984. String Without AAA or BBB
// https://leetcode.com/problems/string-without-aaa-or-bbb/
// https://leetcode.cn/problems/string-without-aaa-or-bbb/
//
// Given two integers a and b, return any string s such that:
//
// s has length a + b and contains exactly a 'a' letters, and exactly b 'b' letters,
// The substring 'aaa' does not occur in s, and
// The substring 'bbb' does not occur in s.
//
// Example 1:
//
// Input: a = 1, b = 2
// Output: "abb"
// Explanation: "abb", "bab" and "bba" are all correct answers.
//
// Example 2:
//
// Input: a = 4, b = 1
// Output: "aabaa"
//
// Constraints:
//
// - 0 <= a, b <= 100
// - It is guaranteed such an s exists for the given a and b.
//

struct Solution;

impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let (mut a, mut b) = (a, b);
        let mut result = vec![];
        if a < b {
            while 0 < a && 0 < b {
                result.push("ba".to_string());
                a -= 1;
                b -= 1;
            }

            if 2 <= b {
                b -= 2;
                result.push("bb".to_string());
            }

            for i in 0..b {
                result[i as usize] = "bba".to_string();
            }
            result.into_iter().collect::<String>()
        } else {
            while 0 < a && 0 < b {
                result.push("ab".to_string());
                a -= 1;
                b -= 1;
            }

            if 2 <= a {
                a -= 2;
                result.push("aa".to_string());
            }

            for i in 0..a {
                result[i as usize] = "aab".to_string();
            }
            result.into_iter().collect::<String>()
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (1, 2, "abb"),
        (4, 1, "aaaab"),
        (1, 4, "abbbb"),
        (2, 2, "aabb"),
        (2, 3, "aabbb"),
        (3, 2, "aaabb"),
        (3, 3, "aaabbb"),
        (3, 4, "aaabbbb"),
    ];

    for (a, b, expected) in cases {
        let result = Solution::str_without3a3b(a, b);
        let mut result = result.chars().collect::<Vec<char>>();
        result.sort();
        let result = result.into_iter().collect::<String>();
        assert_eq!(result, expected);
    }
}
