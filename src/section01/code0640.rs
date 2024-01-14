#![allow(dead_code)]

// 640. Solve the Equation
// https://leetcode.com/problems/solve-the-equation/
// https://leetcode.cn/problems/solve-the-equation/
//
// Solve a given equation and return the value of 'x' in the form of a string "x=#value". The equation contains
// only '+', '-' operation, the variable 'x' and its coefficient. You should return "No solution" if there is
// no solution for the equation, or "Infinite solutions" if there are infinite solutions for the equation.
//
// If there is exactly one solution for the equation, we ensure that the value of 'x' is an integer.
//
// Example 1:
//
// Input: equation = "x+5-3+x=6+x-2"
// Output: "x=2"
//
// Example 2:
//
// Input: equation = "x=x"
// Output: "Infinite solutions"
//
// Example 3:
//
// Input: equation = "2x=x"
// Output: "x=0"
//
// Constraints:
//
// - 3 <= equation.length <= 1000
// - equation has exactly one '='.
// - equation consists of integers with an absolute value in the range [0, 100] without any leading zeros, and the variable 'x'.
//

struct Solution;

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let equations = equation.as_bytes();
        let len = equations.len();
        let (mut tmp, mut lr, mut a, mut b, mut sign, mut calc) = (0, 1, 0, 0, 1, 0);
        for (idx, &equation) in equations.iter().enumerate() {
            if (b'0'..=b'9').contains(&equation) {
                tmp = tmp * 10 + (equation - b'0') as i32;
                calc += 1;
            }
            if equation == b'x' {
                if tmp == 0 && calc == 0 {
                    a += sign * lr;
                }
                a += sign * lr * tmp;
                calc = 0;
                tmp = 0;
            }
            if equation == b'-' || equation == b'+' || equation == b'=' || idx == len - 1 {
                b += sign * lr * tmp;
                calc = 0;
                tmp = 0;
                if equation == b'-' {
                    sign = -1;
                } else if equation == b'+' {
                    sign = 1;
                } else if equation == b'=' {
                    lr = -1;
                    sign = 1;
                }
            }
        }
        if a != 0 {
            format!("x={}", -b / a)
        } else if b == 0 {
            "Infinite solutions".to_string()
        } else {
            "No solution".to_string()
        }
    }
}

#[test]
fn test() {
    let equation = "x+5-3+x=6+x-2".to_string();
    let res = "x=2".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "x=x".to_string();
    let res = "Infinite solutions".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "2x=x".to_string();
    let res = "x=0".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "2x+3x-6x=x+2".to_string();
    let res = "x=-1".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "x=x+2".to_string();
    let res = "No solution".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
}
