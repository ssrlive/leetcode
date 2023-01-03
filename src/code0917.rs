#![allow(dead_code)]

// 917. Reverse Only Letters
// https://leetcode.com/problems/reverse-only-letters/
// https://leetcode.cn/problems/reverse-only-letters/
//
// Given a string s, reverse the string according to the following rules:
//
// All the characters that are not English letters remain in the same position.
// All the English letters (lowercase or uppercase) should be reversed.
// Return s after reversing it.
//
// Example 1:
//
// Input: s = "ab-cd"
// Output: "dc-ba"
//
// Example 2:
//
// Input: s = "a-bC-dEf-ghIj"
// Output: "j-Ih-gfE-dCba"
//
// Example 3:
//
// Input: s = "Test1ng-Leet=code-Q!"
// Output: "Qedo1ct-eeLg=ntse-T!"
//
// Constraints:
//
// - 1 <= s.length <= 100
// - s consists of characters with ASCII values in the range [33, 122].
// - s does not contain '\"' or '\\'.
//

struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        use std::collections::HashMap;
        let mut non_alpha: HashMap<usize, char> = HashMap::new();
        let mut only_alpha = String::from("");
        let mut result = String::from("");
        for (i, c) in s.chars().enumerate() {
            if c.is_alphabetic() {
                only_alpha.push(c);
            } else {
                non_alpha.insert(i, c);
            }
        }
        let mut x = only_alpha.chars();
        for i in 0..s.len() {
            if let Some(&x) = non_alpha.get(&i) {
                result.push(x);
            } else {
                result.push(x.next_back().unwrap());
            }
        }
        result
    }
}

#[test]
fn test() {
    let s = "ab-cd".to_string();
    let result = "dc-ba".to_string();
    assert_eq!(Solution::reverse_only_letters(s), result);

    let s = "a-bC-dEf-ghIj".to_string();
    let result = "j-Ih-gfE-dCba".to_string();
    assert_eq!(Solution::reverse_only_letters(s), result);

    let s = "Test1ng-Leet=code-Q!".to_string();
    let result = "Qedo1ct-eeLg=ntse-T!".to_string();
    assert_eq!(Solution::reverse_only_letters(s), result);
}
