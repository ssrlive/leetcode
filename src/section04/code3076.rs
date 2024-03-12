#![allow(dead_code)]

// 3076. Shortest Uncommon Substring in an Array
// https://leetcode.com/problems/shortest-uncommon-substring-in-an-array/
// https://leetcode.cn/problems/shortest-uncommon-substring-in-an-array/
//
// Medium
//
// You are given an array arr of size n consisting of non-empty strings.
//
// Find a string array answer of size n such that:
//
// answer[i] is the shortest substring of arr[i] that does not occur as a substring in any other string in arr.
// If multiple such substrings exist, answer[i] should be the lexicographically smallest.
// And if no such substring exists, answer[i] should be an empty string.
//
// Return the array answer.
//
// Example 1:
//
// Input: arr = ["cab","ad","bad","c"]
// Output: ["ab","","ba",""]
// Explanation: We have the following:
// - For the string "cab", the shortest substring that does not occur in any other string is either "ca" or "ab",
//   we choose the lexicographically smaller substring, which is "ab".
// - For the string "ad", there is no substring that does not occur in any other string.
// - For the string "bad", the shortest substring that does not occur in any other string is "ba".
// - For the string "c", there is no substring that does not occur in any other string.
//
// Example 2:
//
// Input: arr = ["abc","bcd","abcd"]
// Output: ["","","abcd"]
// Explanation: We have the following:
// - For the string "abc", there is no substring that does not occur in any other string.
// - For the string "bcd", there is no substring that does not occur in any other string.
// - For the string "abcd", the shortest substring that does not occur in any other string is "abcd".
//
// Constraints:
//
// n == arr.length
// 2 <= n <= 100
// 1 <= arr[i].length <= 20
// arr[i] consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let is_unique_substring = |index, substring| arr.iter().enumerate().all(|(i, s)| !s.contains(&substring) || i == index);

        for (index, string) in arr.iter().enumerate() {
            let chars: Vec<char> = string.chars().collect();
            let mut unique_substrings = vec![];

            for length in 1..=chars.len() {
                for start in 0..=chars.len() - length {
                    let substring: String = chars[start..start + length].iter().collect();
                    if is_unique_substring(index, substring.clone()) {
                        unique_substrings.push(substring);
                    }
                }
                if !unique_substrings.is_empty() {
                    break;
                }
            }

            if let Some(first) = unique_substrings.iter().min().cloned() {
                result.push(first);
            } else {
                result.push(String::from(""));
            }
        }

        result
    }
}

#[test]
fn test() {
    let arr = vec!["cab", "ad", "bad", "c"];
    let expected = vec!["ab", "", "ba", ""];
    assert_eq!(Solution::shortest_substrings(arr.iter().map(|s| s.to_string()).collect()), expected);

    let arr = vec!["abc", "bcd", "abcd"];
    let expected = vec!["", "", "abcd"];
    assert_eq!(Solution::shortest_substrings(arr.iter().map(|s| s.to_string()).collect()), expected);
}
