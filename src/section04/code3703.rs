#![allow(dead_code)]

// 3703. Remove K-Balanced Substrings
// https://leetcode.com/problems/remove-k-balanced-substrings/
// https://leetcode.cn/problems/remove-k-balanced-substrings/
//
// Medium
//
// You are given a string s consisting of '(' and ')', and an integer k.
//
// A string is k-balanced if it is exactly k consecutive '(' followed by k consecutive ')', i.e., '(' * k + ')' * k.
//
// For example, if k = 3, k-balanced is "((()))".
//
// You must repeatedly remove all non-overlapping k-balanced substrings from s, and then join the remaining parts.
// Continue this process until no k-balanced substring exists.
//
// Return the final string after all possible removals.
//
// ​​​​​​​Example 1:
//
// Input: s = "(())", k = 1
//
// Output: ""
//
// Explanation:
//
// k-balanced substring is "()"
//
// Step	Current s	k-balanced	Result s
// 1	(())	(())	()
// 2	()	()	Empty
// Thus, the final string is "".
//
// Example 2:
//
// Input: s = "(()(", k = 1
//
// Output: "(("
//
// Explanation:
//
// k-balanced substring is "()"
//
// Step	Current s	k-balanced	Result s
// 1	(()(	(()(	((
// 2	((	-	((
// Thus, the final string is "((".
//
// Example 3:
//
// Input: s = "((()))()()()", k = 3
//
// Output: "()()()"
//
// Explanation:
//
// k-balanced substring is "((()))"
//
// Step	Current s	k-balanced	Result s
// 1	((()))()()()	((()))()()()	()()()
// 2	()()()	-	()()()
// Thus, the final string is "()()()".
//
// Constraints:
//
// 2 <= s.length <= 10^5
// s consists only of '(' and ')'.
// 1 <= k <= s.length / 2
//

struct Solution;

impl Solution {
    pub fn remove_substring(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = Vec::new();

        for c in s.chars() {
            if let Some(last) = stack.last_mut() {
                if last.0 == c {
                    // Same character, increment count
                    last.1 += 1;
                } else {
                    // Different character, push new entry
                    stack.push((c, 1));
                }
            } else {
                // Stack is empty, push new entry
                stack.push((c, 1));
            }

            // Check if the last two groups form a k-balanced substring
            let n = stack.len();
            if n >= 2 && stack[n - 2].0 == '(' && stack[n - 2].1 >= k && stack[n - 1].0 == ')' && stack[n - 1].1 == k {
                // Remove k opening parentheses
                stack[n - 2].1 -= k;

                // Remove the closing parentheses group
                stack.pop();

                // Remove the opening parentheses group if its count is now 0
                if let Some(last) = stack.last()
                    && last.1 == 0
                {
                    stack.pop();
                }
            }
        }

        // Reconstruct the result string from remaining groups
        let mut result = String::new();
        for (ch, count) in stack {
            result.push_str(&ch.to_string().repeat(count as usize));
        }

        result
    }
}

#[test]
fn test() {
    let s = "(())".to_string();
    let k = 1;
    let result = Solution::remove_substring(s, k);
    assert_eq!(result, "");

    let s = "(()(".to_string();
    let k = 1;
    let result = Solution::remove_substring(s, k);
    assert_eq!(result, "((");

    let s = "((()))()()()".to_string();
    let k = 3;
    let result = Solution::remove_substring(s, k);
    assert_eq!(result, "()()()");
}
