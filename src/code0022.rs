#![allow(dead_code)]

// 22. Generate Parentheses
// https://leetcode.com/problems/generate-parentheses/
// https://leetcode.cn/problems/generate-parentheses/
//
// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
//
// Example 1:
//
// Input: n = 3
// Output: ["((()))","(()())","(())()","()(())","()()()"]
//
// Example 2:
//
// Input: n = 1
// Output: ["()"]
//
// Constraints:
//
// 1 <= n <= 8
//

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn back_track(s: String, open: i32, close: i32) -> Vec<String> {
            let mut res = vec![];
            if open == 0 && close == 0 {
                return vec![s];
            }
            if open > 0 {
                res.append(&mut back_track(s.clone() + "(", open - 1, close + 1));
            }
            if close > 0 {
                res.append(&mut back_track(s + ")", open, close - 1));
            }
            res
        }
        back_track("".to_string(), n, 0)
    }
}

#[test]
fn test() {
    let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
    assert_eq!(Solution::generate_parenthesis(3), expected);

    assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
}
