#![allow(dead_code)]

// 520. Detect Capital
// https://leetcode.com/problems/detect-capital/
// https://leetcode.cn/problems/detect-capital/
//
// We define the usage of capitals in a word to be right when one of the following cases holds:
//
// - All letters in this word are capitals, like "USA".
// - All letters in this word are not capitals, like "leetcode".
// - Only the first letter in this word is capital, like "Google".
//
// Given a string word, return true if the usage of capitals in it is right.
//
// Example 1:
//
// Input: word = "USA"
// Output: true
//
// Example 2:
//
// Input: word = "FlaG"
// Output: false
//
// Constraints:
//
// - 1 <= word.length <= 100
// - word consists of lowercase and uppercase English letters.
//

struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        fn _detect_capital_use(word: String) -> Option<bool> {
            let mut chars = word.chars();
            let first = chars.next()?;
            let is_upper = first.is_uppercase();
            let mut is_lower = first.is_lowercase();
            let mut is_mixed = false;

            for c in chars {
                if c.is_uppercase() {
                    if is_lower {
                        return Some(false);
                    }
                    is_mixed = true;
                } else {
                    if is_upper && is_mixed {
                        return Some(false);
                    }
                    is_lower = true;
                }
            }

            Some(true)
        }

        _detect_capital_use(word).unwrap_or(false)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
    assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
    assert_eq!(Solution::detect_capital_use("leetcode".to_string()), true);
    assert_eq!(Solution::detect_capital_use("Google".to_string()), true);
    assert_eq!(Solution::detect_capital_use("g".to_string()), true);
    assert_eq!(Solution::detect_capital_use("G".to_string()), true);
}
