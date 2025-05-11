#![allow(dead_code)]

// 3548. Equal Sum Grid Partition II
// https://leetcode.com/problems/equal-sum-grid-partition-ii
// https://leetcode.cn/problems/equal-sum-grid-partition-ii
//
// Hard
//
// You are given an m x n matrix grid of positive integers. Your task is to determine if it is possible
// to make either one horizontal or one vertical cut on the grid such that:
//
//     Each of the two resulting sections formed by the cut is non-empty.
//     The sum of elements in both sections is equal, or can be made equal by discounting at most one single cell in total (from either section).
//     If a cell is discounted, the rest of the section must remain connected.
//
// Return true if such a partition exists; otherwise, return false.
//
// Note: A section is connected if every cell in it can be reached from any other cell by moving up, down, left, or right through other cells in the section.
//
// Example 1:
//
// Input: grid = [[1,4],[2,3]]
//
// Output: true
//
// Explanation:
//
//     A horizontal cut after the first row gives sums 1 + 4 = 5 and 2 + 3 = 5, which are equal. Thus, the answer is true.
//
// Example 2:
//
// Input: grid = [[1,2],[3,4]]
//
// Output: true
//
// Explanation:
//
//     A vertical cut after the first column gives sums 1 + 3 = 4 and 2 + 4 = 6.
//     By discounting 2 from the right section (6 - 2 = 4), both sections have equal sums and remain connected. Thus, the answer is true.
//
// Example 3:
//
// Input: grid = [[1,2,4],[2,3,5]]
//
// Output: false
//
// Explanation:
//
// - A horizontal cut after the first row gives 1 + 2 + 4 = 7 and 2 + 3 + 5 = 10.
// - By discounting 3 from the bottom section (10 - 3 = 7), both sections have equal sums, but they do not remain connected
//   as it splits the bottom section into two parts ([2] and [5]). Thus, the answer is false.
//
// Example 4:
//
// Input: grid = [[4,1,8],[3,2,6]]
//
// Output: false
//
// Explanation:
//
// No valid cut exists, so the answer is false.
//
// Constraints:
//
//     1 <= m == grid.length <= 10^5
//     1 <= n == grid[i].length <= 10^5
//     2 <= m * n <= 10^5
//     1 <= grid[i][j] <= 10^5
//

struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let grid = grid
            .iter()
            .map(|row| row.iter().map(|&x| x as i64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let n = grid.len() as i64;
        let m = grid[0].len() as i64;
        let mut total_row_sum = 0;
        let mut prefix_row_wise = vec![0; n as usize];
        let mut prefix_col_wise = vec![0; m as usize];
        let maxv = 100_000;
        let mut min_row = vec![None; maxv as usize + 1];
        let mut max_row = vec![None; maxv as usize + 1];
        let mut min_col = vec![None; maxv as usize + 1];
        let mut max_col = vec![None; maxv as usize + 1];
        for i in 0..n {
            for j in 0..m {
                let v = grid[i as usize][j as usize];
                let val = v;
                prefix_row_wise[i as usize] += val;
                prefix_col_wise[j as usize] += val;

                if min_row[v as usize].is_none() {
                    min_row[v as usize] = Some(i);
                    max_row[v as usize] = Some(i);
                    min_col[v as usize] = Some(j);
                    max_col[v as usize] = Some(j);
                } else {
                    if min_row[v as usize].unwrap() > i {
                        min_row[v as usize] = Some(i);
                    }
                    if max_row[v as usize].unwrap() < i {
                        max_row[v as usize] = Some(i);
                    }
                    if min_col[v as usize].unwrap() > j {
                        min_col[v as usize] = Some(j);
                    }
                    if max_col[v as usize].unwrap() < j {
                        max_col[v as usize] = Some(j);
                    }
                }
            }
        }
        for r in &prefix_row_wise {
            total_row_sum += *r;
        }
        let total_col_sum = total_row_sum;
        let mut current_row_upper_sum = 0;
        for i in 0..n - 1 {
            current_row_upper_sum += prefix_row_wise[i as usize];
            let lower_segment_sum = total_row_sum - current_row_upper_sum;
            if current_row_upper_sum == lower_segment_sum {
                return true;
            }
            if current_row_upper_sum > lower_segment_sum {
                let diff = current_row_upper_sum - lower_segment_sum;
                if diff <= maxv && min_row[diff as usize].is_some() {
                    if i == 0 || m == 1 {
                        if diff == grid[0][0] || diff == grid[0][m as usize - 1] {
                            return true;
                        }
                    } else if min_row[diff as usize].unwrap() <= i {
                        return true;
                    }
                }
            } else {
                let diff = lower_segment_sum - current_row_upper_sum;
                if diff <= maxv && max_row[diff as usize].is_some() {
                    if i == n - 2 || m == 1 {
                        if diff == grid[i as usize + 1][0] || diff == grid[i as usize + 1][m as usize - 1] {
                            return true;
                        }
                    } else if max_row[diff as usize].unwrap() > i {
                        return true;
                    }
                }
            }
        }
        let mut current_col_left_sum = 0;
        for j in 0..m - 1 {
            current_col_left_sum += prefix_col_wise[j as usize];
            let right_segment_sum = total_col_sum - current_col_left_sum;
            if current_col_left_sum == right_segment_sum {
                return true;
            }
            if current_col_left_sum > right_segment_sum {
                let diff = current_col_left_sum - right_segment_sum;
                if diff <= maxv && min_col[diff as usize].is_some() {
                    if j == 0 || n == 1 {
                        if diff == grid[0][0] || diff == grid[n as usize - 1][0] {
                            return true;
                        }
                    } else if min_col[diff as usize].unwrap() <= j {
                        return true;
                    }
                }
            } else {
                let diff = right_segment_sum - current_col_left_sum;
                if diff <= maxv && max_col[diff as usize].is_some() {
                    if j == m - 2 || n == 1 {
                        if diff == grid[0][j as usize + 1] || diff == grid[n as usize - 1][j as usize + 1] {
                            return true;
                        }
                    } else if max_col[diff as usize].unwrap() > j {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::can_partition_grid(vec![vec![1, 4], vec![2, 3]]));
    assert!(Solution::can_partition_grid(vec![vec![1, 2], vec![3, 4]]));
    assert!(!Solution::can_partition_grid(vec![vec![1, 2, 4], vec![2, 3, 5]]));
    assert!(!Solution::can_partition_grid(vec![vec![4, 1, 8], vec![3, 2, 6]]));
}
