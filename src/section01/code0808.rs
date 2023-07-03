#![allow(dead_code)]

// 808. Soup Servings
// https://leetcode.com/problems/soup-servings/
// https://leetcode.cn/problems/soup-servings/
//
// There are two types of soup: type A and type B. Initially, we have n ml of each type of soup. There are four kinds of operations:
//
// Serve 100 ml of soup A and 0 ml of soup B,
// Serve 75 ml of soup A and 25 ml of soup B,
// Serve 50 ml of soup A and 50 ml of soup B, and
// Serve 25 ml of soup A and 75 ml of soup B.
// When we serve some soup, we give it to someone, and we no longer have it. Each turn, we will choose from the four operations with an equal probability 0.25.
// If the remaining volume of soup is not enough to complete the operation, we will serve as much as possible. We stop once we no longer have some quantity of both types of soup.
//
// Note that we do not have an operation where all 100 ml's of soup B are used first.
//
// Return the probability that soup A will be empty first, plus half the probability that A and B become empty at the same time. Answers within 10-5 of the actual answer will be accepted.
//
// Example 1:
//
// Input: n = 50
// Output: 0.62500
// Explanation: If we choose the first two operations, A will become empty first.
// For the third operation, A and B will become empty at the same time.
// For the fourth operation, B will become empty first.
// So the total probability of A becoming empty first plus half the probability that A and B become empty at the same time, is 0.25 * (1 + 1 + 0.5 + 0) = 0.625.
//
// Example 2:
//
// Input: n = 100
// Output: 0.71875
//
// Constraints:
//
// - 0 <= n <= 10^9
//

struct Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        fn step(a: i32, b: i32, memo: &mut Vec<Vec<f64>>) -> f64 {
            if a <= 0 {
                return if b <= 0 { 0.5 } else { 1.0 };
            }
            if b <= 0 {
                return 0.0;
            }
            if memo[a as usize][b as usize] > 0.0 {
                return memo[a as usize][b as usize];
            }
            let val = step(a - 100, b, memo) + step(a - 75, b - 25, memo) + step(a - 50, b - 50, memo) + step(a - 25, b - 75, memo);
            memo[a as usize][b as usize] = 0.25 * val;
            memo[a as usize][b as usize]
        }

        if n > 4750 {
            return 1.0;
        }
        let mut memo = vec![vec![0.0; n as usize + 1]; n as usize + 1];
        step(n, n, &mut memo)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::soup_servings(50), 0.625);
    assert_eq!(Solution::soup_servings(100), 0.71875);
}
