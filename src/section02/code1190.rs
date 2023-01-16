#![allow(dead_code)]

// 1190. Reverse Substrings Between Each Pair of Parentheses
// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/
// https://leetcode.cn/problems/reverse-substrings-between-each-pair-of-parentheses/
//
// You are given a string s that consists of lower case English letters and brackets.
//
// Reverse the strings in each pair of matching parentheses, starting from the innermost one.
//
// Your result should not contain any brackets.
//
// Example 1:
//
// Input: s = "(abcd)"
// Output: "dcba"
//
// Example 2:
//
// Input: s = "(u(love)i)"
// Output: "iloveu"
// Explanation: The substring "love" is reversed first, then the whole string is reversed.
//
// Example 3:
//
// Input: s = "(ed(et(oc))el)"
// Output: "leetcode"
// Explanation: First, we reverse the substring "oc", then "etco", and finally, the whole string.
//
// Constraints:
//
// - 1 <= s.length <= 2000
// - s only contains lower case English characters and parentheses.
// - It is guaranteed that all parentheses are balanced.
//

struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![vec![]];
        for c in s.chars() {
            if c == '(' {
                stack.push(vec![]);
            } else if c == ')' {
                let mut arr = stack.pop().unwrap();
                arr.reverse();
                if stack.is_empty() {
                    stack.push(arr);
                } else {
                    let li = stack.len() - 1;
                    for cc in arr {
                        stack[li].push(cc);
                    }
                }
            } else {
                let li = stack.len() - 1;
                stack[li].push(c);
            }
        }

        let mut result = vec![];
        for arr in stack {
            for c in arr {
                result.push(c);
            }
        }

        result.iter().map(|v| v.to_string()).collect::<String>()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_parentheses("(abcd)".to_string()), "dcba");
    assert_eq!(Solution::reverse_parentheses("(u(love)i)".to_string()), "iloveu");
    assert_eq!(Solution::reverse_parentheses("(ed(et(oc))el)".to_string()), "leetcode");
    assert_eq!(
        Solution::reverse_parentheses("a(bcdefghijkl(mno)p)q".to_string()),
        "apmnolkjihgfedcbq"
    );
}
