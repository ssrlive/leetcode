#![allow(dead_code)]

// 744. Find Smallest Letter Greater Than Target
// https://leetcode.com/problems/find-smallest-letter-greater-than-target/
// https://leetcode.cn/problems/find-smallest-letter-greater-than-target/
//
// You are given an array of characters letters that is sorted in non-decreasing order, and a character target.
// There are at least two different characters in letters.
//
// Return the smallest character in letters that is lexicographically greater than target.
// If such a character does not exist, return the first character in letters.
//
// Example 1:
//
// Input: letters = ["c","f","j"], target = "a"
// Output: "c"
// Explanation: The smallest character that is lexicographically greater than 'a' in letters is 'c'.
//
// Example 2:
//
// Input: letters = ["c","f","j"], target = "c"
// Output: "f"
// Explanation: The smallest character that is lexicographically greater than 'c' in letters is 'f'.
//
// Example 3:
//
// Input: letters = ["x","x","y","y"], target = "z"
// Output: "x"
// Explanation: There are no characters in letters that is lexicographically greater than 'z' so we return letters[0].
//
// Constraints:
//
// - 2 <= letters.length <= 10^4
// - letters[i] is a lowercase English letter.
// - letters is sorted in non-decreasing order.
// - letters contains at least two different characters.
// - target is a lowercase English letter.
//

struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut ans = None;
        for &c in letters.iter() {
            if c > target {
                ans = Some(c);
                break;
            }
        }
        ans.unwrap_or(letters[0])
    }
}

#[test]
fn test() {
    assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
    assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
    assert_eq!(Solution::next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'), 'x');
}
