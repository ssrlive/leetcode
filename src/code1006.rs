#![allow(dead_code)]

// 1006. Clumsy Factorial
// https://leetcode.com/problems/clumsy-factorial/
// https://leetcode.cn/problems/clumsy-factorial/
//
// The factorial of a positive integer n is the product of all positive integers less than or equal to n.
//
// For example, factorial(10) = 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1.
// We make a clumsy factorial using the integers in decreasing order by swapping out the multiply operations for a fixed
// rotation of operations with multiply '*', divide '/', add '+', and subtract '-' in this order.
//
// For example, clumsy(10) = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1.
// However, these operations are still applied using the usual order of operations of arithmetic. We do all multiplication and
// division steps before any addition or subtraction steps, and multiplication and division steps are processed left to right.
//
// Additionally, the division that we use is floor division such that 10 * 9 / 8 = 90 / 8 = 11.
//
// Given an integer n, return the clumsy factorial of n.
//
// Example 1:
//
// Input: n = 4
// Output: 7
// Explanation: 7 = 4 * 3 / 2 + 1
//
// Example 2:
//
// Input: n = 10
// Output: 12
// Explanation: 12 = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1
//
// Constraints:
//
// - 1 <= n <= 10^4
//

struct Solution;

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut stk = Vec::new();
        let mut arr = Vec::new();
        let mut temp = 1;
        let mut i = n;
        loop {
            if i < 1 {
                break;
            }
            if temp >= 4 {
                temp = 1;
            }
            if temp == 1 {
                if i - 1 != 0 {
                    stk.push(i * (i - 1));
                } else {
                    stk.push(i);
                }
                i -= 1;
            } else if temp == 2 {
                if !stk.is_empty() {
                    let t = stk.pop().unwrap();
                    stk.push(t / i);
                }
            } else {
                arr.push(i);
            }
            temp += 1;
            i -= 1;
        }
        let mut sum = stk[0];
        for &item in stk.iter().skip(1) {
            sum -= item;
        }
        for &item in &arr {
            sum += item;
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::clumsy(4), 7);
    assert_eq!(Solution::clumsy(10), 12);
}
