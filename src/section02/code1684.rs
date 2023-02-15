#![allow(dead_code)]

/*

// 1684. Count the Number of Consistent Strings
// https://leetcode.com/problems/count-the-number-of-consistent-strings/
// https://leetcode.cn/problems/count-the-number-of-consistent-strings/
//
// Easy
//
// You are given a string allowed consisting of distinct characters and an array of strings words. A string is consistent if all characters in the string appear in the string allowed.

Return the number of consistent strings in the array words.

Example 1:

Input: allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
Output: 2
Explanation: Strings "aaab" and "baa" are consistent since they only contain characters 'a' and 'b'.

Example 2:

Input: allowed = "abc", words = ["a","b","c","ab","ac","bc","abc"]
Output: 7
Explanation: All strings are consistent.

Example 3:

Input: allowed = "cad", words = ["cc","acd","b","ba","bac","bad","ac","d"]
Output: 4
Explanation: Strings "cc", "acd", "ac", and "d" are consistent.

Constraints:

    1 <= words.length <= 10^4
    1 <= allowed.length <= 26
    1 <= words[i].length <= 10
    The characters in allowed are distinct.
    words[i] and allowed contain only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut count = 0;
        for word in words {
            let mut flag = true;
            for c in word.chars() {
                if !allowed.contains(c) {
                    flag = false;
                    break;
                }
            }
            if flag {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    let cases = vec![
        ("ab", vec!["ad", "bd", "aaab", "baa", "badab"], 2),
        ("abc", vec!["a", "b", "c", "ab", "ac", "bc", "abc"], 7),
        ("cad", vec!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"], 4),
    ];
    for (a, w, ex) in cases {
        let w: Vec<String> = w.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::count_consistent_strings(a.to_string(), w), ex);
    }
}
