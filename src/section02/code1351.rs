#![allow(dead_code)]

// 1351. Count Negative Numbers in a Sorted Matrix
// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
// https://leetcode.cn/problems/count-negative-numbers-in-a-sorted-matrix/
//
// Easy
//
// Given a m x n matrix grid which is sorted in non-increasing order both row-wise and column-wise,
// return the number of negative numbers in grid.
//
// Example 1:
//
// Input: grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
// Output: 8
// Explanation: There are 8 negatives number in the matrix.
//
// Example 2:
//
// Input: grid = [[3,2],[1,0]]
// Output: 0
//
// Constraints:
//
// -    m == grid.length
// -    n == grid[i].length
// -    1 <= m, n <= 100
// -    -100 <= grid[i][j] <= 100
//

struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter()
            .map(|x| x.iter().filter(|&&y| y < 0).count() as i32)
            .sum::<i32>()
    }

    pub fn count_negatives_2(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for row in grid.iter() {
            for &col in row.iter() {
                if col < 0 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[test]
fn test() {
    let grid = vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
    ];
    assert_eq!(Solution::count_negatives(grid), 8);

    let grid = vec![vec![3, 2], vec![1, 0]];
    assert_eq!(Solution::count_negatives(grid), 0);
}
