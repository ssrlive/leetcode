#![allow(dead_code)]

// 301. Remove Invalid Parentheses
// https://leetcode.com/problems/remove-invalid-parentheses/
//
// Given a string s that contains parentheses and letters, remove the minimum number of invalid parentheses to make the input string valid.
//
// Return all the possible results. You may return the answer in any order.
//
// Example 1:
//
// Input: s = "()())()"
// Output: ["(())()","()()()"]
//
// Example 2:
//
// Input: s = "(a)())()"
// Output: ["(a())()","(a)()()"]
//
// Example 3:
//
// Input: s = ")("
// Output: [""]
//
// Constraints:
//
// - 1 <= s.length <= 25
// - s consists of lowercase English letters and parentheses '(' and ')'.
// - There will be at most 20 parentheses in s.
//

struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        fn is_valid(s: &str) -> bool {
            let mut count = 0;
            for c in s.chars() {
                match c {
                    '(' => count += 1,
                    ')' => {
                        count -= 1;
                        if count < 0 {
                            return false;
                        }
                    }
                    _ => (),
                }
            }
            count == 0
        }

        let mut level = std::collections::HashSet::new();
        level.insert(s);
        while !level.is_empty() {
            let mut valid = Vec::new();
            for s in level.iter() {
                if is_valid(s) {
                    valid.push(s.clone());
                }
            }
            if !valid.is_empty() {
                return valid;
            }
            let mut next_level = std::collections::HashSet::new();
            for s in level.iter() {
                for i in 0..s.len() {
                    let mut s = s.clone();
                    s.remove(i);
                    next_level.insert(s);
                }
            }
            level = next_level;
        }
        vec![]
    }
}

#[test]
fn test_remove_invalid_parentheses() {
    let s = "()())()".to_string();
    let result = Solution::remove_invalid_parentheses(s);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"()()()".to_string()));
    assert!(result.contains(&"(())()".to_string()));

    let s = "(a)())()".to_string();
    let result = Solution::remove_invalid_parentheses(s);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"(a)()()".to_string()));
    assert!(result.contains(&"(a())()".to_string()));

    let s = ")(".to_string();
    let result = Solution::remove_invalid_parentheses(s);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&"".to_string()));

    let s = "((()((s((((()".to_string();
    let result = Solution::remove_invalid_parentheses(s);
    assert_eq!(result.len(), 3);
    println!("{:?}", result);
    assert!(result.contains(&"(()s)".to_string()));
}
