#![allow(dead_code)]

// 3121. Count the Number of Special Characters II
// https://leetcode.com/problems/count-the-number-of-special-characters-ii/
// https://leetcode.cn/problems/count-the-number-of-special-characters-ii/
//
// Medium
//
// You are given a string word. A letter c is called special if it appears both in lowercase and uppercase in word, and every lowercase occurrence of c appears before the first uppercase occurrence of c.
//
// Return the number of special letters in word.
//
// Example 1:
//
// Input: word = "aaAbcBC"
//
// Output: 3
//
// Explanation:
//
// The special characters are 'a', 'b', and 'c'.
//
// Example 2:
//
// Input: word = "abc"
//
// Output: 0
//
// Explanation:
//
// There are no special characters in word.
//
// Example 3:
//
// Input: word = "AbBCab"
//
// Output: 0
//
// Explanation:
//
// There are no special characters in word.
//
// Constraints:
//
// 1 <= word.length <= 2 * 10^5
// word consists of only lowercase and uppercase English letters.
//

struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let (mut val_l, mut val_u) = ([0; 26], [0; 26]);
        let word = word.as_bytes();

        for i in 0..word.len() {
            if word[i] >= 97 {
                val_l[(word[i] - 97) as usize] = i + 1;
            } else if val_u[(word[i] - 65) as usize] == 0 {
                val_u[(word[i] - 65) as usize] = i + 1;
            }
        }

        let mut res = 0;
        for i in 0..26 {
            if val_l[i] > 0 && val_l[i] < val_u[i] {
                res += 1;
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
    assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
    assert_eq!(Solution::number_of_special_chars("AbBCab".to_string()), 0);
}
