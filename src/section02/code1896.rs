#![allow(dead_code)]

/*

// 1896. Minimum Cost to Change the Final Value of Expression
// https://leetcode.com/problems/minimum-cost-to-change-the-final-value-of-expression/
// https://leetcode.cn/problems/minimum-cost-to-change-the-final-value-of-expression/
//
// Hard
//
// You are given a valid boolean expression as a string expression consisting of the characters '1','0','&' (bitwise AND operator),'|' (bitwise OR operator),'(', and ')'.

    For example, "()1|1" and "(1)&()" are not valid while "1", "(((1))|(0))", and "1|(0&(1))" are valid expressions.

Return the minimum cost to change the final value of the expression.

    For example, if expression = "1|1|(0&0)&1", its value is 1|1|(0&0)&1 = 1|1|0&1 = 1|0&1 = 1&1 = 1. We want to apply operations so that the new expression evaluates to 0.

The cost of changing the final value of an expression is the number of operations performed on the expression. The types of operations are described as follows:

    Turn a '1' into a '0'.
    Turn a '0' into a '1'.
    Turn a '&' into a '|'.
    Turn a '|' into a '&'.

Note: '&' does not take precedence over '|' in the order of calculation. Evaluate parentheses first, then in left-to-right order.

Example 1:

Input: expression = "1&(0|1)"
Output: 1
Explanation: We can turn "1&(0|1)" into "1&(0&1)" by changing the '|' to a '&' using 1 operation.
The new expression evaluates to 0.

Example 2:

Input: expression = "(0&0)&(0&0&0)"
Output: 3
Explanation: We can turn "(0&0)&(0&0&0)" into "(0|1)|(0&0&0)" using 3 operations.
The new expression evaluates to 1.

Example 3:

Input: expression = "(0|(1|0&1))"
Output: 1
Explanation: We can turn "(0|(1|0&1))" into "(0|(0|0&1))" using 1 operation.
The new expression evaluates to 0.

Constraints:

    1 <= expression.length <= 10^5
    expression only contains '1','0','&','|','(', and ')'
    All parentheses are properly matched.
    There will be no empty parentheses (i.e: "()" is not a substring of expression).
*/

struct Solution;

impl Solution {
    pub fn min_operations_to_flip(expression: String) -> i32 {
        let expression = expression.as_bytes();
        let mut s = std::collections::VecDeque::new();
        let (mut val1, mut val2, mut op): (char, char, char);
        let (mut cost1, mut cost2): (i32, i32);
        let mut p: (char, i32);
        for &expression_i in expression.iter() {
            if expression_i == b'(' || expression_i == b'&' || expression_i == b'|' {
                s.push_back((expression_i as char, 0));
            } else {
                if expression_i == b')' {
                    p = s.pop_back().unwrap();
                    s.pop_back();
                } else {
                    p = (expression_i as char, 1);
                }
                while !s.is_empty() && (s.back().unwrap().0 == '&' || s.back().unwrap().0 == '|') {
                    op = s.pop_back().unwrap().0;
                    val2 = p.0;
                    cost2 = p.1;
                    let (val1_, cost1_) = s.pop_back().unwrap();
                    val1 = val1_;
                    cost1 = cost1_;

                    if op == '&' && val1 == '1' && val2 == '1' {
                        p = ('1', cost1.min(cost2))
                    }
                    if op == '&' && val1 == '1' && val2 == '0' {
                        p = ('0', 1)
                    }
                    if op == '&' && val1 == '0' && val2 == '1' {
                        p = ('0', 1)
                    }
                    if op == '&' && val1 == '0' && val2 == '0' {
                        p = ('0', (1 + cost1).min(1 + cost2))
                    }

                    if op == '|' && val1 == '1' && val2 == '1' {
                        p = ('1', (1 + cost1).min(1 + cost2))
                    }
                    if op == '|' && val1 == '1' && val2 == '0' {
                        p = ('1', 1)
                    }
                    if op == '|' && val1 == '0' && val2 == '1' {
                        p = ('1', 1)
                    }
                    if op == '|' && val1 == '0' && val2 == '0' {
                        p = ('0', cost1.min(cost2))
                    }
                }
                s.push_back(p);
            }
        }
        s.pop_back().unwrap().1
    }
}

#[test]
fn test() {
    let cases = vec![
        ("1&(0|1)".to_string(), 1),
        ("(0&0)&(0&0&0)".to_string(), 3),
        ("(0|(1|0&1))".to_string(), 1),
    ];
    for (expression, expected) in cases {
        assert_eq!(Solution::min_operations_to_flip(expression), expected);
    }
}
