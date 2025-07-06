#![allow(dead_code)]

// 1209. Remove All Adjacent Duplicates in String II
// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string-ii/
// https://leetcode.cn/problems/remove-all-adjacent-duplicates-in-string-ii/
//
// Medium
//
// You are given a string s and an integer k, a k duplicate removal consists of choosing k adjacent and equal letters from s and removing them, causing the left and the right side of the deleted substring to concatenate together.
//
// We repeatedly make k duplicate removals on s until we no longer can.
//
// Return the final string after all such duplicate removals have been made. It is guaranteed that the answer is unique.
//
// Example 1:
//
// Input: s = "abcd", k = 2
// Output: "abcd"
// Explanation: There's nothing to delete.
//
// Example 2:
//
// Input: s = "deeedbbcccbdaa", k = 3
// Output: "aa"
// Explanation:
// First delete "eee" and "ccc", get "ddbbbdaa"
// Then delete "bbb", get "dddaa"
// Finally delete "ddd", get "aa"
//
// Example 3:
//
// Input: s = "pbbcggttciiippooaais", k = 2
// Output: "ps"
//
// Constraints:
//
// -    1 <= s.length <= 10^5
// -    2 <= k <= 10^4
// -    s only contains lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if let Some((last_c, last_count)) = stack.last_mut()
                && *last_c == c
            {
                *last_count += 1;
                if *last_count == k {
                    stack.pop();
                }
                continue;
            }
            stack.push((c, 1));
        }
        stack.into_iter().map(|(c, count)| c.to_string().repeat(count as usize)).collect()
    }
}

#[test]
fn test() {
    let cases = vec![("abcd", 2, "abcd"), ("deeedbbcccbdaa", 3, "aa"), ("pbbcggttciiippooaais", 2, "ps")];
    for (s, k, expected) in cases {
        let s = s.to_string();
        assert_eq!(Solution::remove_duplicates(s, k), expected);
    }
}
