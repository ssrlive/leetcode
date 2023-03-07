#![allow(dead_code)]

/*

// 2375. Construct Smallest Number From DI String
// https://leetcode.com/problems/construct-smallest-number-from-di-string/
// https://leetcode.cn/problems/construct-smallest-number-from-di-string/
//
// Medium
//
// You are given a 0-indexed string pattern of length n consisting of the characters 'I' meaning increasing and 'D' meaning decreasing.

A 0-indexed string num of length n + 1 is created using the following conditions:

    num consists of the digits '1' to '9', where each digit is used at most once.
    If pattern[i] == 'I', then num[i] < num[i + 1].
    If pattern[i] == 'D', then num[i] > num[i + 1].

Return the lexicographically smallest possible string num that meets the conditions.

Example 1:

Input: pattern = "IIIDIDDD"
Output: "123549876"
Explanation:
At indices 0, 1, 2, and 4 we must have that num[i] < num[i+1].
At indices 3, 5, 6, and 7 we must have that num[i] > num[i+1].
Some possible values of num are "245639871", "135749862", and "123849765".
It can be proven that "123549876" is the smallest possible num that meets the conditions.
Note that "123414321" is not possible because the digit '1' is used more than once.

Example 2:

Input: pattern = "DDD"
Output: "4321"
Explanation:
Some possible values of num are "9876", "7321", and "8742".
It can be proven that "4321" is the smallest possible num that meets the conditions.

Constraints:

    1 <= pattern.length <= 8
    pattern consists of only the letters 'I' and 'D'.
*/

struct Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        // use std::iter::FromIterator;
        let pattern = pattern.chars().collect::<Vec<char>>();
        let mut s = String::from("123456789").chars().collect::<Vec<char>>();
        let (mut i, n) = (0, pattern.len());
        while i < n {
            let mut count = 0;
            while i + count < n && pattern[i + count] == 'D' {
                count += 1;
            }
            for k in 0..n {
                if i + k >= i + count - k {
                    break;
                }
                s.swap(i + k, i + count - k);
            }
            i += count + 1;
        }
        String::from_iter(s[0..=n].iter())
    }
}

#[test]
fn test() {
    let cases = vec![
        ("IIIDIDDD", "123549876"),
        ("DDD", "4321"),
        ("IIII", "12345"),
        ("DDDD", "54321"),
    ];
    for (pattern, expected) in cases {
        assert_eq!(Solution::smallest_number(pattern.to_string()), expected.to_string());
    }
}
