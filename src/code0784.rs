#![allow(dead_code)]

// 784. Letter Case Permutation
// https://leetcode.com/problems/letter-case-permutation/
// https://leetcode.cn/problems/letter-case-permutation/
//
// Given a string s, you can transform every letter individually to be lowercase or uppercase to create another string.
//
// Return a list of all possible strings we could create. Return the output in any order.
//
// Example 1:
//
// Input: s = "a1b2"
// Output: ["a1b2","a1B2","A1b2","A1B2"]
//
// Example 2:
//
// Input: s = "3z4"
// Output: ["3z4","3Z4"]
//
// Constraints:
//
// - 1 <= s.length <= 12
// - s consists of lowercase English letters, uppercase English letters, and digits.
//

struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn backtrack(s: &[u8], i: usize, path: &mut Vec<u8>, result: &mut Vec<String>) -> Option<()> {
            if i == s.len() {
                result.push(String::from_utf8(path.clone()).ok()?);
                return Some(());
            }

            if s[i].is_ascii_alphabetic() {
                path.push(s[i].to_ascii_lowercase());
                backtrack(s, i + 1, path, result)?;
                path.pop();

                path.push(s[i].to_ascii_uppercase());
                backtrack(s, i + 1, path, result)?;
                path.pop();
            } else {
                path.push(s[i]);
                backtrack(s, i + 1, path, result)?;
                path.pop();
            }
            Some(())
        }

        let mut result = vec![];
        let mut path = vec![];
        backtrack(s.as_bytes(), 0, &mut path, &mut result);
        result
    }
}

#[test]
fn test() {
    let s = "a1b2".to_string();
    let result = Solution::letter_case_permutation(s);
    assert_eq!(result.len(), 4);
    assert!(result.contains(&"a1b2".to_string()));
    assert!(result.contains(&"a1B2".to_string()));
    assert!(result.contains(&"A1b2".to_string()));
    assert!(result.contains(&"A1B2".to_string()));

    let s = "3z4".to_string();
    let result = Solution::letter_case_permutation(s);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"3z4".to_string()));
    assert!(result.contains(&"3Z4".to_string()));
}
