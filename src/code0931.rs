#![allow(dead_code)]

// 931. Minimum Falling Path Sum
// https://leetcode.com/problems/minimum-falling-path-sum/
// https://leetcode.cn/problems/minimum-falling-path-sum/
//
// Given an n x n array of integers matrix, return the minimum sum of any falling path through matrix.
//
// A falling path starts at any element in the first row and chooses the element in the next row that is either directly below or diagonally left/right. Specifically, the next element from position (row, col) will be (row + 1, col - 1), (row + 1, col), or (row + 1, col + 1).
//
// Example 1:
//
//
// Input: matrix = [[2,1,3],[6,5,4],[7,8,9]]
// Output: 13
// Explanation: There are two falling paths with a minimum sum as shown.
//
// Example 2:
//
// Input: matrix = [[-19,57],[-40,-5]]
// Output: -59
// Explanation: The falling path with a minimum sum is shown.
//
// Constraints:
//
// - n == matrix.length == matrix[i].length
// - 1 <= n <= 100
// - -100 <= matrix[i][j] <= 100
//

struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = Vec::new();
        for &num in &matrix[0] {
            dp.push(num);
        }
        for item in matrix.iter().skip(1) {
            let mut tmp = Vec::new();
            for j in 0..item.len() {
                let mut sum = dp[j] + item[j];
                if j > 0 {
                    sum = sum.min(dp[j - 1] + item[j]);
                }
                if j < item.len() - 1 {
                    sum = sum.min(dp[j + 1] + item[j]);
                }
                tmp.push(sum);
            }
            dp = tmp;
        }
        *dp.iter().min().unwrap()
    }
}

#[test]
fn test() {
    let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    assert_eq!(Solution::min_falling_path_sum(matrix), 13);

    let matrix = vec![vec![-19, 57], vec![-40, -5]];
    assert_eq!(Solution::min_falling_path_sum(matrix), -59);
}
