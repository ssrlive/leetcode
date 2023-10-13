#![allow(dead_code)]

/*

// 1941. Check if All Characters Have Equal Number of Occurrences
// https://leetcode.com/problems/check-if-all-characters-have-equal-number-of-occurrences/
// https://leetcode.cn/problems/check-if-all-characters-have-equal-number-of-occurrences/
//
// Easy
//
// Given a string s, return true if s is a good string, or false otherwise.

A string s is good if all the characters that appear in s have the same number of occurrences (i.e., the same frequency).

Example 1:

Input: s = "abacbc"
Output: true
Explanation: The characters that appear in s are 'a', 'b', and 'c'. All characters occur 2 times in s.

Example 2:

Input: s = "aaabb"
Output: false
Explanation: The characters that appear in s are 'a' and 'b'.
'a' occurs 3 times while 'b' occurs 2 times, which is not the same number of times.

Constraints:

    1 <= s.length <= 1000
    s consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut counts = vec![0; 26];
        let mut max = 0;
        for x in s.chars() {
            let cnt = &mut counts[x as usize - 'a' as usize];
            *cnt += 1;
            max = max.max(*cnt);
        }
        counts.into_iter().filter(|x| *x > 0).all(|x| x == max)
    }
}

#[test]
fn test() {
    assert!(Solution::are_occurrences_equal("abacbc".to_string()));
    assert!(!Solution::are_occurrences_equal("aaabb".to_string()));
}
