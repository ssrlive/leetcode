#![allow(dead_code)]

/*

// 1878. Get Biggest Three Rhombus Sums in a Grid
// https://leetcode.com/problems/get-biggest-three-rhombus-sums-in-a-grid/
// https://leetcode.cn/problems/get-biggest-three-rhombus-sums-in-a-grid/
//
// Medium
//
// You are given an m x n integer matrix grid​​​.

A rhombus sum is the sum of the elements that form the border of a regular rhombus shape in grid​​​. The rhombus must have the shape of a square rotated 45 degrees with each of the corners centered in a grid cell. Below is an image of four valid rhombus shapes with the corresponding colored cells that should be included in each rhombus sum:

Note that the rhombus can have an area of 0, which is depicted by the purple rhombus in the bottom right corner.

Return the biggest three distinct rhombus sums in the grid in descending order. If there are less than three distinct values, return all of them.

Example 1:

Input: grid = [[3,4,5,1,3],[3,3,4,2,3],[20,30,200,40,10],[1,5,5,4,1],[4,3,2,2,5]]
Output: [228,216,211]
Explanation: The rhombus shapes for the three biggest distinct rhombus sums are depicted above.
- Blue: 20 + 3 + 200 + 5 = 228
- Red: 200 + 2 + 10 + 4 = 216
- Green: 5 + 200 + 4 + 2 = 211

Example 2:

Input: grid = [[1,2,3],[4,5,6],[7,8,9]]
Output: [20,9,8]
Explanation: The rhombus shapes for the three biggest distinct rhombus sums are depicted above.
- Blue: 4 + 2 + 6 + 8 = 20
- Red: 9 (area 0 rhombus in the bottom right corner)
- Green: 8 (area 0 rhombus in the bottom middle)

Example 3:

Input: grid = [[7,7,7]]
Output: [7]
Explanation: All three possible rhombus sums are the same, so return [7].

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 50
    1 <= grid[i][j] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ld = grid.clone();
        let mut rd = grid.clone();
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                let pi = i as i32 - 1;
                let prevj = j as i32 - 1;
                if pi >= 0 && prevj >= 0 {
                    ld[i][j] += ld[pi as usize][prevj as usize];
                }
                let prevj = j + 1;
                if pi >= 0 && prevj < n {
                    rd[i][j] += rd[pi as usize][prevj];
                }
            }
        }
        let mut cnt = std::collections::BTreeMap::new();
        for i in 0..m {
            for j in 0..n {
                *cnt.entry(grid[i][j]).or_insert(0) += 1;
                for k in 1..=m.max(n) {
                    let left = j as i32 - k as i32;
                    let right = j as i32 + k as i32;
                    let bot = i as i32 + 2 * k as i32;
                    if left < 0 || right >= n as i32 || bot >= m as i32 {
                        continue;
                    }
                    let sum = rd[i + k][left as usize] - rd[i][j] + ld[i + k][right as usize] - ld[i][j]
                        + ld[bot as usize][j]
                        - ld[i + k][left as usize]
                        + rd[bot as usize][j]
                        - rd[i + k][right as usize]
                        + grid[i][j]
                        - grid[bot as usize][j];
                    *cnt.entry(sum).or_insert(0) += 1;
                }
                while cnt.len() > 3 {
                    let k = *cnt.iter().next().unwrap().0;
                    cnt.remove(&k);
                }
            }
        }
        let mut ans = vec![];
        for (&k, _) in cnt.iter().rev() {
            ans.push(k);
            if ans.len() >= 3 {
                return ans;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let grid = vec![
        vec![3, 4, 5, 1, 3],
        vec![3, 3, 4, 2, 3],
        vec![20, 30, 200, 40, 10],
        vec![1, 5, 5, 4, 1],
        vec![4, 3, 2, 2, 5],
    ];
    let result = vec![228, 216, 211];
    assert_eq!(Solution::get_biggest_three(grid), result);

    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result = vec![20, 9, 8];
    assert_eq!(Solution::get_biggest_three(grid), result);

    let grid = vec![vec![7, 7, 7]];
    let result = vec![7];
    assert_eq!(Solution::get_biggest_three(grid), result);
}
