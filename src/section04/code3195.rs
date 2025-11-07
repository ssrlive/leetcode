#![allow(dead_code)]

// 3195. Find the Minimum Area to Cover All Ones I
// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones/
// https://leetcode.cn/problems/find-the-minimum-area-to-cover-all-ones/
//
// Medium
//
// You are given a 2D binary array grid. Find a rectangle with horizontal and vertical sides with the smallest area,
// such that all the 1's in grid lie inside this rectangle.
//
// Return the minimum possible area of the rectangle.
//
// Example 1:
//
// Input: grid = [[0,1,0],[1,0,1]]
//
// Output: 6
//
// Explanation:
//
// The smallest rectangle has a height of 2 and a width of 3, so it has an area of 2 * 3 = 6.
//
// Example 2:
//
// Input: grid = [[1,0],[0,0]]
//
// Output: 1
//
// Explanation:
//
// The smallest rectangle has both height and width 1, so its area is 1 * 1 = 1.
//
// Constraints:
//
//     1 <= grid.length, grid[i].length <= 1000
//     grid[i][j] is either 0 or 1.
//     The input is generated such that there is at least one 1 in grid.
//

struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut y1, mut y2, mut x1, mut x2) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    y1 = y1.min(y as i32);
                    y2 = y2.max(y as i32);
                    x1 = x1.min(x as i32);
                    x2 = x2.max(x as i32);
                }
            }
        }

        (y2 - y1 + 1) * (x2 - x1 + 1)
    }
}

#[test]
fn test() {
    let grid = vec![vec![0, 1, 0], vec![1, 0, 1]];
    assert_eq!(Solution::minimum_area(grid), 6);

    let grid = vec![vec![1, 0], vec![0, 0]];
    assert_eq!(Solution::minimum_area(grid), 1);
}
