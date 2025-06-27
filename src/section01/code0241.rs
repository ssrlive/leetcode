#![allow(dead_code)]

// 241. Different Ways to Add Parentheses
// https://leetcode.com/problems/different-ways-to-add-parentheses/
// https://leetcode.cn/problems/different-ways-to-add-parentheses/
//
// Given a string expression of numbers and operators, return all possible results from computing all
// the different possible ways to group numbers and operators. You may return the answer in any order.
//
// The test cases are generated such that the output values fit in a 32-bit integer and
// the number of different results does not exceed 104.
//
// Example 1:
//
// Input: "2-1-1"
// Output: [0, 2]
// Explanation:
// ((2-1)-1) = 0
// (2-(1-1)) = 2
//
// Example 2:
//
// Input: "2*3-4*5"
// Output: [-34, -14, -10, -10, 10]
// Explanation:
// (2*(3-(4*5))) = -34
// ((2*3)-(4*5)) = -14
// ((2*(3-4))*5) = -10
// (2*((3-4)*5)) = -10
// (((2*3)-4)*5) = 10
//

struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut result = vec![];
        for (i, c) in expression.char_indices() {
            if c == '+' || c == '-' || c == '*' {
                let left = Self::diff_ways_to_compute(expression[..i].to_string());
                let right = Self::diff_ways_to_compute(expression[i + 1..].to_string());
                for l in left {
                    for r in right.clone() {
                        match c {
                            '+' => result.push(l + r),
                            '-' => result.push(l - r),
                            '*' => result.push(l * r),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        if result.is_empty() {
            result.push(expression.parse().unwrap());
        }
        result
    }
}

#[test]
fn test() {
    let mut v = Solution::diff_ways_to_compute("2-1-1".to_string());
    v.sort();
    assert_eq!(v, vec![0, 2]);

    let mut v = Solution::diff_ways_to_compute("2*3-4*5".to_string());
    v.sort();
    assert_eq!(v, vec![-34, -14, -10, -10, 10]);
}
