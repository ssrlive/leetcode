#![allow(dead_code)]

// 3216. Lexicographically Smallest String After a Swap
// https://leetcode.com/problems/lexicographically-smallest-string-after-a-swap/
// https://leetcode.cn/problems/lexicographically-smallest-string-after-a-swap/
//
// Easy
//
// Given a string s containing only digits, return the lexicographically smallest string
// that can be obtained after swapping adjacent digits in s with the same parity at most once.
//
// Digits have the same parity if both are odd or both are even. For example, 5 and 9,
// as well as 2 and 4, have the same parity, while 6 and 9 do not.
//
// Example 1:
//
// Input: s = "45320"
//
// Output: "43520"
//
// Explanation:
//
// s[1] == '5' and s[2] == '3' both have the same parity, and swapping them results in the lexicographically smallest string.
//
// Example 2:
//
// Input: s = "001"
//
// Output: "001"
//
// Explanation:
//
// There is no need to perform a swap because s is already the lexicographically smallest.
//
// Constraints:
//
//     2 <= s.length <= 100
//     s consists only of digits.
//

struct Solution;

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut s = s.as_bytes().to_vec();

        for i in 0..s.len() - 1 {
            if (s[i] % 2 == s[i + 1] % 2) && (s[i] > s[i + 1]) {
                s.swap(i, i + 1);
                break;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_smallest_string("45320".to_string()), "43520");
    assert_eq!(Solution::get_smallest_string("001".to_string()), "001");
}
