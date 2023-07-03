#![allow(dead_code)]

/*

// 2232. Minimize Result by Adding Parentheses to Expression
// https://leetcode.com/problems/minimize-result-by-adding-parentheses-to-expression/
// https://leetcode.cn/problems/minimize-result-by-adding-parentheses-to-expression/
//
// Medium
//
// You are given a 0-indexed string expression of the form "<num1>+<num2>" where <num1> and <num2> represent positive integers.

Add a pair of parentheses to expression such that after the addition of parentheses, expression is a valid mathematical expression and evaluates to the smallest possible value. The left parenthesis must be added to the left of '+' and the right parenthesis must be added to the right of '+'.

Return expression after adding a pair of parentheses such that expression evaluates to the smallest possible value. If there are multiple answers that yield the same result, return any of them.

The input has been generated such that the original value of expression, and the value of expression after adding any pair of parentheses that meets the requirements fits within a signed 32-bit integer.

Example 1:

Input: expression = "247+38"
Output: "2(47+38)"
Explanation: The expression evaluates to 2 * (47 + 38) = 2 * 85 = 170.
Note that "2(4)7+38" is invalid because the right parenthesis must be to the right of the '+'.
It can be shown that 170 is the smallest possible value.

Example 2:

Input: expression = "12+34"
Output: "1(2+3)4"
Explanation: The expression evaluates to 1 * (2 + 3) * 4 = 1 * 5 * 4 = 20.

Example 3:

Input: expression = "999+999"
Output: "(999+999)"
Explanation: The expression evaluates to 999 + 999 = 1998.

Constraints:

    3 <= expression.length <= 10
    expression consists of digits from '1' to '9' and '+'.
    expression starts and ends with digits.
    expression contains exactly one '+'.
    The original value of expression, and the value of expression after adding any pair of parentheses that meets the requirements fits within a signed 32-bit integer.
*/

struct Solution;

impl Solution {
    pub fn minimize_result(expression: String) -> String {
        let mut plus = false;
        let mut a = String::new();
        let mut b = String::new();
        for ch in expression.chars() {
            if ch == '+' {
                plus = true;
            }
            if !plus {
                a.push(ch);
            } else {
                b.push(ch);
            }
        }
        let n = a.len();
        let m = b.len();
        let mut ans = String::new();
        let mut maxi = i32::MAX;
        for i in 0..n {
            for j in 1..m {
                let left = if i == 0 { 1 } else { a[0..i].parse::<i32>().unwrap() };
                let middle = a[i..n].parse::<i32>().unwrap() + b[0..j + 1].parse::<i32>().unwrap();
                let right = if j == m - 1 { 1 } else { b[j + 1..m].parse::<i32>().unwrap() };
                if left * middle * right < maxi {
                    maxi = left * middle * right;
                    ans = a[0..i].to_string() + "(" + &a[i..n] + &b[0..j + 1] + ")" + &b[j + 1..m];
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("247+38", "2(47+38)"),
        ("12+34", "1(2+3)4"),
        ("999+999", "(999+999)"),
    ];
    for (expression, expected) in cases {
        assert_eq!(Solution::minimize_result(expression.to_string()), expected);
    }
}
