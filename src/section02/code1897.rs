#![allow(dead_code)]

/*

// 1897. Redistribute Characters to Make All Strings Equal
// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/
// https://leetcode.cn/problems/redistribute-characters-to-make-all-strings-equal/
//
// Easy
//
// You are given an array of strings words (0-indexed).

In one operation, pick two distinct indices i and j, where words[i] is a non-empty string, and move any character from words[i] to any position in words[j].

Return true if you can make every string in words equal using any number of operations, and false otherwise.

Example 1:

Input: words = ["abc","aabc","bc"]
Output: true
Explanation: Move the first 'a' in words[1] to the front of words[2],
to make words[1] = "abc" and words[2] = "abc".
All the strings are now equal to "abc", so return true.

Example 2:

Input: words = ["ab","a"]
Output: false
Explanation: It is impossible to make all the strings equal using the operation.

Constraints:

    1 <= words.length <= 100
    1 <= words[i].length <= 100
    words[i] consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let l = words.len();
        words
            .into_iter()
            .flat_map(|s| s.into_bytes())
            .fold([0; 26], |mut counter, b| {
                counter[(b - b'a') as usize] += 1;
                counter
            })
            .iter()
            .all(|x: &usize| (*x).is_multiple_of(l))
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["abc", "aabc", "bc"], true),
        (vec!["ab", "a"], false),
        (vec!["abc", "aabc", "bc", "abc", "aabc", "bc"], true),
    ];
    for (words, expected) in cases {
        let words = words.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::make_equal(words), expected);
    }
}
