#![allow(dead_code)]

// 344. Reverse String
// https://leetcode.com/problems/reverse-string/
// https://leetcode.cn/problems/reverse-string/
//
// Write a function that reverses a string. The input string is given as an array of characters s.
//
// You must do this by modifying the input array in-place with O(1) extra memory.
//
// Example 1:
//
// Input: s = ["h","e","l","l","o"]
// Output: ["o","l","l","e","h"]
//
// Example 2:
//
// Input: s = ["H","a","n","n","a","h"]
// Output: ["h","a","n","n","a","H"]
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s[i] is a printable ascii character.
//
// Follow up: Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
//

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[test]
fn test_reverse_string() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut s);
    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reverse_string(&mut s);
    assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}
