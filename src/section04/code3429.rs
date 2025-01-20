#![allow(dead_code)]

// 3429. Paint House IV
// https://leetcode.com/problems/paint-house-iv/
// https://leetcode.cn/problems/paint-house-iv/
//
// Medium
//
// You are given an even integer n representing the number of houses arranged in a straight line, and a 2D array
// cost of size n x 3, where cost[i][j] represents the cost of painting house i with color j + 1.
//
// The houses will look beautiful if they satisfy the following conditions:
//
// - No two adjacent houses are painted the same color.
// - Houses equidistant from the ends of the row are not painted the same color. For example, if n = 6,
//   houses at positions (0, 5), (1, 4), and (2, 3) are considered equidistant.
//
// Return the minimum cost to paint the houses such that they look beautiful.
//
// Example 1:
//
// Input: n = 4, cost = [[3,5,7],[6,2,9],[4,8,1],[7,3,5]]
//
// Output: 9
//
// Explanation:
//
// The optimal painting sequence is [1, 2, 3, 2] with corresponding costs [3, 2, 1, 3]. This satisfies the following conditions:
//
//     No adjacent houses have the same color.
//     Houses at positions 0 and 3 (equidistant from the ends) are not painted the same color (1 != 2).
//     Houses at positions 1 and 2 (equidistant from the ends) are not painted the same color (2 != 3).
//
// The minimum cost to paint the houses so that they look beautiful is 3 + 2 + 1 + 3 = 9.
//
// Example 2:
//
// Input: n = 6, cost = [[2,4,6],[5,3,8],[7,1,9],[4,6,2],[3,5,7],[8,2,4]]
//
// Output: 18
//
// Explanation:
//
// The optimal painting sequence is [1, 3, 2, 3, 1, 2] with corresponding costs [2, 8, 1, 2, 3, 2]. This satisfies the following conditions:
//
//     No adjacent houses have the same color.
//     Houses at positions 0 and 5 (equidistant from the ends) are not painted the same color (1 != 2).
//     Houses at positions 1 and 4 (equidistant from the ends) are not painted the same color (3 != 1).
//     Houses at positions 2 and 3 (equidistant from the ends) are not painted the same color (2 != 3).
//
// The minimum cost to paint the houses so that they look beautiful is 2 + 8 + 1 + 2 + 3 + 2 = 18.
//
// Constraints:
//
//     2 <= n <= 10^5
//     n is even.
//     cost.length == n
//     cost[i].length == 3
//     0 <= cost[i][j] <= 10^5
//

struct Solution;

impl Solution {
    pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        fn sol(c: &[Vec<i32>], l: i32, r: i32, i: i32, dp: &mut [Vec<Vec<i64>>]) -> i64 {
            let n = c.len() as i32;
            if i >= n / 2 {
                return 0;
            }

            if dp[i as usize][l as usize][r as usize] != -1 {
                return dp[i as usize][l as usize][r as usize];
            }

            let mut lftc = Vec::new();
            let mut rgtc = Vec::new();
            for j in 0..3 {
                if l != j {
                    lftc.push(j);
                }
                if r != j {
                    rgtc.push(j);
                }
            }
            let mut ans = i64::MAX;
            for h in lftc {
                for j in rgtc.iter() {
                    if h == *j {
                        continue;
                    }
                    let cl = c[i as usize][h as usize];
                    let cr = c[(n - i - 1) as usize][*j as usize];
                    ans = ans.min(cl as i64 + cr as i64 + sol(c, h, *j, i + 1, dp));
                }
            }
            dp[i as usize][l as usize][r as usize] = ans;
            ans
        }

        let mut dp = vec![vec![vec![-1; 4]; 4]; (n / 2 + 1) as usize];
        sol(&cost, 3, 3, 0, &mut dp)
    }
}

#[test]
fn test() {
    let n = 4;
    let cost = vec![vec![3, 5, 7], vec![6, 2, 9], vec![4, 8, 1], vec![7, 3, 5]];
    let res = 9;
    assert_eq!(Solution::min_cost(n, cost), res);

    let n = 6;
    let cost = vec![
        vec![2, 4, 6],
        vec![5, 3, 8],
        vec![7, 1, 9],
        vec![4, 6, 2],
        vec![3, 5, 7],
        vec![8, 2, 4],
    ];
    let res = 18;
    assert_eq!(Solution::min_cost(n, cost), res);
}
