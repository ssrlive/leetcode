#![allow(dead_code)]

// 3240. Minimum Number of Flips to Make Binary Grid Palindromic II
// https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-ii/
// https://leetcode.cn/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-ii/
//
// Medium
//
// You are given an m x n binary matrix grid.
//
// A row or column is considered palindromic if its values read the same forward and backward.
//
// You can flip any number of cells in grid from 0 to 1, or from 1 to 0.
//
// Return the minimum number of cells that need to be flipped to make all rows and columns palindromic,
// and the total number of 1's in grid divisible by 4.
//
// Example 1:
//
// Input: grid = [[1,0,0],[0,1,0],[0,0,1]]
//
// Output: 3
//
// Explanation:
//
// Example 2:
//
// Input: grid = [[0,1],[0,1],[0,0]]
//
// Output: 2
//
// Explanation:
//
// Example 3:
//
// Input: grid = [[1],[1]]
//
// Output: 2
//
// Explanation:
//
// Constraints:
//
//     m == grid.length
//     n == grid[i].length
//     1 <= m * n <= 2 * 10^5
//     0 <= grid[i][j] <= 1
//

struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        // Handle symmetrical cells, they don't contribute to the count of ones we should care about since they come in pack of 4s
        let mut ri = 0;
        let mut rj = grid.len() - 1;
        let mut change_count = 0;

        while ri < rj {
            let mut ci = 0;
            let mut cj = grid[0].len() - 1;

            while ci < cj {
                let tl = grid[ri][ci];
                let tr = grid[ri][cj];

                let bl = grid[rj][ci];
                let br = grid[rj][cj];

                let count1 = (tl + tr + bl + br) % 4;

                if count1 > 0 {
                    change_count += match count1 {
                        2 => 2,
                        _ => 1,
                    };
                }

                ci += 1;
                cj -= 1;
            }

            ri += 1;
            rj -= 1;
        }

        // Count number of flips in central row / column - required changes to make valid palindrom
        let mut flip_special = 0;
        // Count of pairs of existing ones - need to balance them (make count of pairs even) to make sure they divisible by 4
        let mut one_pairs = 0;

        if grid.len() % 2 == 1 {
            let row = &grid[grid.len() / 2];
            let mut i = 0;
            let mut j = grid[0].len() - 1;

            while i < j {
                let a = row[i];
                let b = row[j];
                if a != b {
                    flip_special += 1;
                } else if a == 1 {
                    one_pairs += 1;
                }

                i += 1;
                j -= 1;
            }
        }

        if grid[0].len() % 2 == 1 {
            let mut i = 0;
            let mut j = grid.len() - 1;
            let col = grid[0].len() / 2;

            while i < j {
                if grid[i][col] != grid[j][col] {
                    flip_special += 1;
                } else if grid[i][col] == 1 {
                    one_pairs += 1;
                }
                i += 1;
                j -= 1;
            }
        }

        // zero the middle
        if grid.len() % 2 == 1 && grid[0].len() % 2 == 1 && grid[grid.len() / 2][grid[0].len() / 2] == 1 {
            change_count += 1;
        }

        // if there was flip we can flip it to 1s instead of 0s increasing number of one_pairs by one without additional cost
        if one_pairs % 2 == 1 && flip_special == 0 {
            // otherwise we have to flip that pair of ones to zeros
            change_count += 2;
        }

        change_count + flip_special
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    let expected = 3;
    assert_eq!(Solution::min_flips(grid), expected);

    let grid = vec![vec![0, 1], vec![0, 1], vec![0, 0]];
    let expected = 2;
    assert_eq!(Solution::min_flips(grid), expected);

    let grid = vec![vec![1], vec![1]];
    let expected = 2;
    assert_eq!(Solution::min_flips(grid), expected);
}
