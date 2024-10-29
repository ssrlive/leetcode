#![allow(dead_code)]

// 3330. Find the Original Typed String I
// https://leetcode.com/problems/find-the-original-typed-string-i/
// https://leetcode.cn/problems/find-the-original-typed-string-i/
//
// Easy
//
// Alice is attempting to type a specific string on her computer. However, she tends to be clumsy
// and may press a key for too long, resulting in a character being typed multiple times.
//
// Although Alice tried to focus on her typing, she is aware that she may still have done this at most once.
//
// You are given a string word, which represents the final output displayed on Alice's screen.
//
// Return the total number of possible original strings that Alice might have intended to type.
//
// Example 1:
//
// Input: word = "abbcccc"
//
// Output: 5
//
// Explanation:
//
// The possible strings are: "abbcccc", "abbccc", "abbcc", "abbc", and "abcccc".
//
// Example 2:
//
// Input: word = "abcd"
//
// Output: 1
//
// Explanation:
//
// The only possible string is "abcd".
//
// Example 3:
//
// Input: word = "aaaa"
//
// Output: 4
//
// Constraints:
//
//    1 <= word.length <= 100
//    word consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut res = 1;
        let word: Vec<_> = word.chars().collect();
        for w in word.windows(2) {
            if w[0] == w[1] {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::possible_string_count("abbcccc".to_string()), 5);
    assert_eq!(Solution::possible_string_count("abcd".to_string()), 1);
    assert_eq!(Solution::possible_string_count("aaaa".to_string()), 4);
}
