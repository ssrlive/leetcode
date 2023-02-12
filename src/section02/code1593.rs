#![allow(dead_code)]

/*

// 1593. Split a String Into the Max Number of Unique Substrings
Medium
699
26
Companies

Given a string s, return the maximum number of unique substrings that the given string can be split into.

You can split string s into any list of non-empty substrings, where the concatenation of the substrings forms the original string. However, you must split the substrings such that all of them are unique.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: s = "ababccc"
Output: 5
Explanation: One way to split maximally is ['a', 'b', 'ab', 'c', 'cc']. Splitting like ['a', 'b', 'a', 'b', 'c', 'cc'] is not valid as you have 'a' and 'b' multiple times.

Example 2:

Input: s = "aba"
Output: 2
Explanation: One way to split maximally is ['a', 'ba'].

Example 3:

Input: s = "aa"
Output: 1
Explanation: It is impossible to split the string any further.

Constraints:

    1 <= s.length <= 16
    s contains only lower case English letters.
*/

struct Solution;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        use std::collections::HashSet;

        fn dfs(s: &str, start: usize, set: &mut HashSet<String>, max: &mut i32) {
            if start == s.len() {
                *max = std::cmp::max(*max, set.len() as i32);
                return;
            }
            for i in start..s.len() {
                let sub = &s[start..=i];
                if set.contains(sub) {
                    continue;
                }
                set.insert(sub.to_string());
                dfs(s, i + 1, set, max);
                set.remove(sub);
            }
        }

        let mut set = HashSet::new();
        let mut max = 0;
        dfs(&s, 0, &mut set, &mut max);
        max
    }
}

#[test]
fn test() {
    let s = "ababccc".to_string();
    let res = 5;
    assert_eq!(Solution::max_unique_split(s), res);
    let s = "aba".to_string();
    let res = 2;
    assert_eq!(Solution::max_unique_split(s), res);
    let s = "aa".to_string();
    let res = 1;
    assert_eq!(Solution::max_unique_split(s), res);
}
