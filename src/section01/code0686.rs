#![allow(dead_code)]

// 686. Repeated String Match
// https://leetcode.com/problems/repeated-string-match/
// https://leetcode.cn/problems/repeated-string-match/
//
// Given two strings a and b, return the minimum number of times you should repeat string a so that string b
// is a substring of it. If it is impossible for b​​​​​​ to be a substring of a after repeating it, return -1.
//
// Notice: string "abc" repeated 0 times is "", repeated 1 time is "abc" and repeated 2 times is "abcabc".
//
// Example 1:
//
// Input: a = "abcd", b = "cdabcdab"
// Output: 3
// Explanation: We return 3 because by repeating a three times "abcdabcdabcd", b is a substring of it.
//
// Example 2:
//
// Input: a = "a", b = "aa"
// Output: 2
//
// Constraints:
//
// - 1 <= a.length, b.length <= 10^4
// - a and b consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut c = String::new();
        for i in 0..(b.len() / a.len() + 3) {
            if c.contains(&b) {
                return i as i32;
            }
            c = format!("{c}{a}");
        }
        -1
    }
}

#[test]
fn test() {
    fn _repeated_string_match(a: &str, b: &str) -> i32 {
        Solution::repeated_string_match(a.to_string(), b.to_string())
    }

    assert_eq!(_repeated_string_match("abcd", "cdabcdab"), 3);
    assert_eq!(_repeated_string_match("a", "aa"), 2);
    assert_eq!(_repeated_string_match("a", "a"), 1);
    assert_eq!(_repeated_string_match("abc", "wxyz"), -1);
    assert_eq!(_repeated_string_match("abc", "cabcabca"), 4);
    assert_eq!(_repeated_string_match("abc", "abac"), -1);
    assert_eq!(_repeated_string_match("abc", "abab"), -1);
    assert_eq!(_repeated_string_match("abc", "ababc"), -1);
}
