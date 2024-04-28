#![allow(dead_code)]

// 3128. Right Triangles
// https://leetcode.com/problems/right-triangles/
// https://leetcode.cn/problems/right-triangles/
//
// Medium
//
// You are given a 2D boolean matrix grid.
//
// Return an integer that is the number of right triangles that can be made with the 3 elements of grid such that all of them have a value of 1.
//
// Note:
//
// A collection of 3 elements of grid is a right triangle if one of its elements is in the same row with another element
// and in the same column with the third element. The 3 elements do not have to be next to each other.
//
// Example 1:
//
// Input: grid = [[0,1,0],[0,1,1],[0,1,0]]
//
// Output: 2
//
// Explanation:
//
// There are two right triangles.
//
// Example 2:
//
// Input: grid = [[1,0,0,0],[0,1,0,1],[1,0,0,0]]
//
// Output: 0
//
// Explanation:
//
// There are no right triangles.
//
// Example 3:
//
// Input: grid = [[1,0,1],[1,0,0],[1,0,0]]
//
// Output: 2
//
// Explanation:
//
// There are two right triangles.
//
// Constraints:
//
// 1 <= grid.length <= 1000
// 1 <= grid[i].length <= 1000
// 0 <= grid[i][j] <= 1
//

struct Solution;

impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let mut t = 0i64;
        let (mm, nn) = (grid.len(), grid[0].len());
        let mut uu = grid.to_vec();
        let mut ll = grid.to_vec();
        let mut dd = grid.to_vec();
        let mut rr = grid.to_vec();
        for i in 0..mm {
            let u = mm - 1 - i;
            for j in 0..nn {
                let v = nn - 1 - j;
                uu[i][j] += if 0 < i { uu[i - 1][j] } else { 0 };
                ll[i][j] += if 0 < j { ll[i][j - 1] } else { 0 };
                dd[u][j] += if u + 1 < mm { dd[u + 1][j] } else { 0 };
                rr[i][v] += if v + 1 < nn { rr[i][v + 1] } else { 0 };
            }
        }
        for i in 0..mm {
            for j in 0..nn {
                if grid[i][j] == 1 {
                    t += ((uu[i][j] - 1) * (ll[i][j] - 1)) as i64;
                    t += ((uu[i][j] - 1) * (rr[i][j] - 1)) as i64;
                    t += ((dd[i][j] - 1) * (ll[i][j] - 1)) as i64;
                    t += ((dd[i][j] - 1) * (rr[i][j] - 1)) as i64;
                }
            }
        }
        t
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]];
    assert_eq!(Solution::number_of_right_triangles(grid), 2);

    let grid = vec![vec![1, 0, 0, 0], vec![0, 1, 0, 1], vec![1, 0, 0, 0]];
    assert_eq!(Solution::number_of_right_triangles(grid), 0);

    let grid = vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]];
    assert_eq!(Solution::number_of_right_triangles(grid), 2);
}
