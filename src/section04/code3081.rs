#![allow(dead_code)]

// 3081. Replace Question Marks in String to Minimize Its Value
// https://leetcode.com/problems/replace-question-marks-in-string-to-minimize-its-value/
// https://leetcode.cn/problems/replace-question-marks-in-string-to-minimize-its-value/
//
// Medium
//
// You are given a string s. s[i] is either a lowercase English letter or '?'.
//
// For a string t having length m containing only lowercase English letters, we define the function cost(i)
// for an index i as the number of characters equal to t[i] that appeared before it, i.e. in the range [0, i - 1].
//
// The value of t is the sum of cost(i) for all indices i.
//
// For example, for the string t = "aab":
//
//     cost(0) = 0
//     cost(1) = 1
//     cost(2) = 0
//     Hence, the value of "aab" is 0 + 1 + 0 = 1.
//
// Your task is to replace all occurrences of '?' in s with any lowercase English letter so that the value of s is minimized.
//
// Return a string denoting the modified string with replaced occurrences of '?'.
// If there are multiple strings resulting in the minimum value, return the lexicographically smallest one.
//
// Example 1:
//
// Input: s = "???"
//
// Output: "abc"
//
// Explanation: In this example, we can replace the occurrences of '?' to make s equal to "abc".
//
// For "abc", cost(0) = 0, cost(1) = 0, and cost(2) = 0.
//
// The value of "abc" is 0.
//
// Some other modifications of s that have a value of 0 are "cba", "abz", and, "hey".
//
// Among all of them, we choose the lexicographically smallest.
//
// Example 2:
//
// Input: s = "a?a?"
//
// Output: "abac"
//
// Explanation: In this example, the occurrences of '?' can be replaced to make s equal to "abac".
//
// For "abac", cost(0) = 0, cost(1) = 0, cost(2) = 1, and cost(3) = 0.
//
// The value of "abac" is 1.
//
// Constraints:
//
//     1 <= s.length <= 10^5
//     s[i] is either a lowercase English letter or '?'.
//

struct Solution;

impl Solution {
    pub fn minimize_string_value(s: String) -> String {
        let s = s.as_bytes();
        let (mut cnts, mut cnt) = ([0; 26], 0);
        for i in 0..s.len() {
            if s[i] == 63 {
                cnt += 1;
            } else {
                cnts[s[i] as usize - 97] += 1;
            }
        }

        let mut data = std::collections::BinaryHeap::with_capacity(26);
        for (i, &cnts_i) in cnts.iter().enumerate() {
            data.push((-cnts_i, -(i as i32)));
        }

        let (mut values_insert, mut pos) = (Vec::with_capacity(cnt as usize), 0);
        for _ in 0..cnt {
            let (v, p) = data.pop().unwrap();
            values_insert.push(-p);
            data.push((v - 1, p));
        }

        values_insert.sort_unstable();
        let mut res = vec![0; s.len()];
        for i in 0..s.len() {
            if s[i] == 63 {
                res[i] = values_insert[pos] as u8 + 97;
                pos += 1;
            } else {
                res[i] = s[i];
            }
        }

        unsafe { String::from_utf8_unchecked(res) }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimize_string_value("???".to_string()), "abc");
    assert_eq!(Solution::minimize_string_value("a?a?".to_string()), "abac");
}
