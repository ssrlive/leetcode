#![allow(dead_code)]

// 3557. Find Maximum Number of Non Intersecting Substrings
// https://leetcode.com/problems/find-maximum-number-of-non-intersecting-substrings/
// https://leetcode.cn/problems/find-maximum-number-of-non-intersecting-substrings/
//
// Medium
//
// You are given a string word.
//
// Return the maximum number of non-intersecting of word that are at least four characters long and start and end with the same letter.
//
// Example 1:
//
// Input: word = "abcdeafdef"
//
// Output: 2
//
// Explanation:
//
// The two substrings are "abcdea" and "fdef".
//
// Example 2:
//
// Input: word = "bcdaaaab"
//
// Output: 1
//
// Explanation:
//
// The only substring is "aaaa". Note that we cannot also choose "bcdaaaab" since it intersects with the other substring.
//
// Constraints:
//
//     1 <= word.length <= 2 * 10^5
//     word consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn max_substrings(word: String) -> i32 {
        let mut len = 0;
        let mut arr = [-1; 26];
        for (i, c) in word.chars().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            if arr[idx] != -1 && i as i32 - arr[idx] + 1 >= 4 {
                len += 1;
                arr.fill(-1);
            } else if arr[idx] == -1 {
                arr[idx] = i as i32;
            }
        }
        len
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_substrings("abcdeafdef".to_string()), 2);
    assert_eq!(Solution::max_substrings("bcdaaaab".to_string()), 1);
}
