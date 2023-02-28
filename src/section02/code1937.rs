#![allow(dead_code)]

/*

// 1937. Maximum Number of Points with Cost
// https://leetcode.com/problems/maximum-number-of-points-with-cost/
// https://leetcode.cn/problems/maximum-number-of-points-with-cost/
//
// Medium
//
// You are given an m x n integer matrix points (0-indexed). Starting with 0 points, you want to maximize the number of points you can get from the matrix.

To gain points, you must pick one cell in each row. Picking the cell at coordinates (r, c) will add points[r][c] to your score.

However, you will lose points if you pick a cell too far from the cell that you picked in the previous row. For every two adjacent rows r and r + 1 (where 0 <= r < m - 1), picking cells at coordinates (r, c1) and (r + 1, c2) will subtract abs(c1 - c2) from your score.

Return the maximum number of points you can achieve.

abs(x) is defined as:

    x for x >= 0.
    -x for x < 0.

Example 1:

Input: points = [[1,2,3],[1,5,1],[3,1,1]]
Output: 9
Explanation:
The blue cells denote the optimal cells to pick, which have coordinates (0, 2), (1, 1), and (2, 0).
You add 3 + 5 + 3 = 11 to your score.
However, you must subtract abs(2 - 1) + abs(1 - 0) = 2 from your score.
Your final score is 11 - 2 = 9.

Example 2:

Input: points = [[1,5],[2,3],[4,2]]
Output: 11
Explanation:
The blue cells denote the optimal cells to pick, which have coordinates (0, 1), (1, 1), and (2, 0).
You add 5 + 3 + 4 = 12 to your score.
However, you must subtract abs(1 - 1) + abs(1 - 0) = 1 from your score.
Your final score is 12 - 1 = 11.

Constraints:

    m == points.length
    n == points[r].length
    1 <= m, n <= 10^5
    1 <= m * n <= 10^5
    0 <= points[r][c] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let col_len = points[0].len();
        let mut ans: Vec<i64> = points[0].iter().map(|&num| i64::from(num)).collect();
        for row in points.iter().skip(1) {
            let mut dp: Vec<i64> = ans.clone();
            let mut left = ans[0];
            let mut right = ans[col_len - 1];
            for i in 1..col_len {
                left = (left - 1).max(ans[i]);
                dp[i] = dp[i].max(left);

                let j = col_len - 1 - i;
                right = (right - 1).max(ans[j]);
                dp[j] = dp[j].max(right);
            }
            for i in 0..col_len {
                ans[i] = dp[i] + i64::from(row[i]);
            }
        }
        *ans.iter().max().unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]], 9),
        (vec![vec![1, 5], vec![2, 3], vec![4, 2]], 11),
    ];
    for (points, expect) in cases {
        assert_eq!(Solution::max_points(points), expect);
    }
}
