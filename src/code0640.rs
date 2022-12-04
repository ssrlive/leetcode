#![allow(dead_code)]

// 640. Solve the Equation
// https://leetcode.com/problems/solve-the-equation/
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
        let mut nox = 0;
        let mut curr = 0;
        let mut i = 0;
        let mut j = 0;
        let n = equation.len();
        let equation = equation.as_bytes();
        while i < n {
            if equation[i] == b'=' {
                j = i;
                break;
            } else if equation[i] == b'x' && i == 0 {
                nox += 1;
            } else if equation[i] == b'x' && equation[i - 1] == b'-' {
                nox -= 1;
            } else if equation[i] == b'x' && equation[i - 1] == b'+' {
                nox += 1;
            } else if equation[i] >= b'0' && equation[i] <= b'9' && i == 0 {
                let mut s = String::new();
                while i < n && equation[i] >= b'0' && equation[i] <= b'9' {
                    s.push(equation[i] as char);
                    i += 1;
                }
                if i < n && equation[i] == b'x' {
                    nox += s.parse::<i32>().unwrap();
                } else {
                    i -= 1;
                    curr += s.parse::<i32>().unwrap();
                }
            } else if equation[i] >= b'0' && equation[i] <= b'9' && equation[i - 1] == b'-' {
                let mut s = String::new();
                while i < n && equation[i] >= b'0' && equation[i] <= b'9' {
                    s.push(equation[i] as char);
                    i += 1;
                }
                if i < n && equation[i] == b'x' {
                    nox -= s.parse::<i32>().unwrap();
                } else {
                    i -= 1;
                    curr -= s.parse::<i32>().unwrap();
                }
            } else if equation[i] >= b'0' && equation[i] <= b'9' && equation[i - 1] == b'+' {
                let mut s = String::new();
                while i < n && equation[i] >= b'0' && equation[i] <= b'9' {
                    s.push(equation[i] as char);
                    i += 1;
                }
                if i < n && equation[i] == b'x' {
                    nox += s.parse::<i32>().unwrap();
                } else {
                    i -= 1;
                    curr += s.parse::<i32>().unwrap();
                }
            }
            i += 1;
        }
        i = j + 1;
        while i < n {
            if equation[i] == b'x' && equation[i - 1] == b'=' {
                nox -= 1;
            } else if equation[i] == b'x' && equation[i - 1] == b'-' {
                nox += 1;
            } else if equation[i] == b'x' && equation[i - 1] == b'+' {
                nox -= 1;
            } else if equation[i] >= b'0' && equation[i] <= b'9' && equation[i - 1] == b'=' {
                let mut s = String::new();
                while i < n && equation[i] >= b'0' && equation[i] <= b'9' {
                    s.push(equation[i] as char);
                    i += 1;
                }
                if i < n && equation[i] == b'x' {
                    nox -= s.parse::<i32>().unwrap();
                } else {
                    i -= 1;
                    curr -= s.parse::<i32>().unwrap();
                }
            } else if equation[i] >= b'0' && equation[i] <= b'9' && equation[i - 1] == b'-' {
                let mut s = String::new();
                while i < n && equation[i] >= b'0' && equation[i] <= b'9' {
                    s.push(equation[i] as char);
                    i += 1;
                }
                if i < n && equation[i] == b'x' {
                    nox += s.parse::<i32>().unwrap();
                } else {
                    i -= 1;
                    curr += s.parse::<i32>().unwrap();
                }
            } else if equation[i] >= b'0' && equation[i] <= b'9' && equation[i - 1] == b'+' {
                let mut s = String::new();
                while i < n && equation[i] >= b'0' && equation[i] <= b'9' {
                    s.push(equation[i] as char);
                    i += 1;
                }
                if i < n && equation[i] == b'x' {
                    nox -= s.parse::<i32>().unwrap();
                } else {
                    i -= 1;
                    curr -= s.parse::<i32>().unwrap();
                }
            }
            i += 1;
        }
        if nox == 0 && curr == 0 {
            return "Infinite solutions".to_string();
        } else if nox == 0 {
            return "No solution".to_string();
        }
        let res = -(curr / nox);
        let mut ans = res.to_string();
        ans.insert(0, 'x');
        ans.insert(1, '=');
        ans
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
