#![allow(dead_code)]

// 1039. Minimum Score Triangulation of Polygon
// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/
// https://leetcode.cn/problems/minimum-score-triangulation-of-polygon/
//
// You have a convex n-sided polygon where each vertex has an integer value. You are given an integer array
// values where values[i] is the value of the ith vertex (i.e., clockwise order).
//
// You will triangulate the polygon into n - 2 triangles. For each triangle, the value of that triangle is
// the product of the values of its vertices, and the total score of the triangulation is the sum of these values
// over all n - 2 triangles in the triangulation.
//
// Return the smallest possible total score that you can achieve with some triangulation of the polygon.
//
// Example 1:
//
// Input: values = [1,2,3]
// Output: 6
// Explanation: The polygon is already triangulated, and the score of the only triangle is 6.
//
// Example 2:
//
// Input: values = [3,7,4,5]
// Output: 144
// Explanation: There are two triangulations, with possible scores: 3*7*5 + 4*5*7 = 245, or 3*4*5 + 3*4*7 = 144.
// The minimum score is 144.
//
// Example 3:
//
// Input: values = [1,3,1,4,1,5]
// Output: 13
// Explanation: The minimum score triangulation has score 1*1*3 + 1*1*4 + 1*1*5 + 1*1*1 = 13.
//
// Constraints:
//
// - n == values.length
// - 3 <= n <= 50
// - 1 <= values[i] <= 100
//

struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![0; n]; n];
        for j in 2..n {
            for i in (0..j - 1).rev() {
                dp[i][j] = i32::MAX;
                for k in i + 1..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + values[i] * values[j] * values[k]);
                }
            }
        }
        dp[0][n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_score_triangulation(vec![1, 2, 3]), 6);
    assert_eq!(Solution::min_score_triangulation(vec![3, 7, 4, 5]), 144);
    assert_eq!(Solution::min_score_triangulation(vec![1, 3, 1, 4, 1, 5]), 13);
}
