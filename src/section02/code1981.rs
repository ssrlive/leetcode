#![allow(dead_code)]

/*

// 1981. Minimize the Difference Between Target and Chosen Elements
// https://leetcode.com/problems/minimize-the-difference-between-target-and-chosen-elements/
// https://leetcode.cn/problems/minimize-the-difference-between-target-and-chosen-elements/
//
// Medium
//
// You are given an m x n integer matrix mat and an integer target.

Choose one integer from each row in the matrix such that the absolute difference between target and the sum of the chosen elements is minimized.

Return the minimum absolute difference.

The absolute difference between two numbers a and b is the absolute value of a - b.

Example 1:

Input: mat = [[1,2,3],[4,5,6],[7,8,9]], target = 13
Output: 0
Explanation: One possible choice is to:
- Choose 1 from the first row.
- Choose 5 from the second row.
- Choose 7 from the third row.
The sum of the chosen elements is 13, which equals the target, so the absolute difference is 0.

Example 2:

Input: mat = [[1],[2],[3]], target = 100
Output: 94
Explanation: The best possible choice is to:
- Choose 1 from the first row.
- Choose 2 from the second row.
- Choose 3 from the third row.
The sum of the chosen elements is 6, and the absolute difference is 94.

Example 3:

Input: mat = [[1,2,9,8,7]], target = 6
Output: 1
Explanation: The best choice is to choose 7 from the first row.
The absolute difference is 1.

Constraints:

    m == mat.length
    n == mat[i].length
    1 <= m, n <= 70
    1 <= mat[i][j] <= 70
    1 <= target <= 800
*/

struct Solution;

impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let max = 70 * 70;
        let mut dp = vec![vec![false; max + 1_usize]; n + 1];
        dp[0][0] = true;
        for i in 0..n {
            for j in 0..m {
                for k in 0..max + 1 {
                    if dp[i][k] {
                        dp[i + 1][k + mat[i][j] as usize] = true;
                    }
                }
            }
        }
        let result = dp[n]
            .iter()
            .enumerate()
            .filter(|x| *x.1)
            .map(|x| (target - x.0 as i32).abs())
            .min()
            .unwrap();
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13, 0),
        (vec![vec![1], vec![2], vec![3]], 100, 94),
        (vec![vec![1, 2, 9, 8, 7]], 6, 1),
    ];
    for (mat, target, expect) in cases {
        assert_eq!(Solution::minimize_the_difference(mat, target), expect);
    }
}
