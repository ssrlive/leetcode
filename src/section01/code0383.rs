#![allow(dead_code)]

// 383. Ransom Note
// https://leetcode.com/problems/ransom-note/
// https://leetcode.cn/problems/ransom-note/
//
// Given two stings ransomNote and magazine, return true if ransomNote can be constructed from magazine and false otherwise.
//
// Each letter in magazine can only be used once in ransomNote.
//
// Example 1:
//
// Input: ransomNote = "a", magazine = "b"
// Output: false
//
// Example 2:
//
// Input: ransomNote = "aa", magazine = "ab"
// Output: false
//
// Example 3:
//
// Input: ransomNote = "aa", magazine = "aab"
// Output: true
//
// Constraints:
//
// - 1 <= ransomNote.length, magazine.length <= 10^5
// - ransomNote and magazine consist of lowercase English letters.
//

struct Solution;

impl Solution {
    fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine = magazine.into_bytes();
        magazine.sort_unstable();
        for c in ransom_note.into_bytes() {
            match magazine.binary_search(&c) {
                Ok(i) => {
                    magazine.remove(i);
                }
                Err(_) => return false,
            }
        }
        true
    }
}

#[test]
fn test_can_construct() {
    assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
    assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
    assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
}
