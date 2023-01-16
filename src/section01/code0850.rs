#![allow(dead_code)]

// 850. Rectangle Area II
// https://leetcode.com/problems/rectangle-area-ii/
// https://leetcode.cn/problems/rectangle-area-ii/
//
// You are given a 2D array of axis-aligned rectangles. Each rectangle[i] = [xi1, yi1, xi2, yi2] denotes the ith rectangle
// where (xi1, yi1) are the coordinates of the bottom-left corner, and (xi2, yi2) are the coordinates of the top-right corner.
//
// Calculate the total area covered by all rectangles in the plane. Any area covered by two or more rectangles should only be counted once.
//
// Return the total area. Since the answer may be too large, return it modulo 109 + 7.
//
// Example 1:
// Input: rectangles = [[0,0,2,2],[1,0,2,3],[1,0,3,1]]
// Output: 6
// Explanation: A total area of 6 is covered by all three rectangles, as illustrated in the picture.
// From (1,1) to (2,2), the green and red rectangles overlap.
// From (1,0) to (2,3), all three rectangles overlap.
//
// Example 2:
//
// Input: rectangles = [[0,0,1000000000,1000000000]]
// Output: 49
// Explanation: The answer is 1018 modulo (109 + 7), which is 49.
//
// Constraints:
//
// - 1 <= rectangles.length <= 200
// - rectanges[i].length == 4
// - 0 <= xi1, yi1, xi2, yi2 <= 10^9
//

struct Solution;

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        fn _rectangle_area(rectangles: Vec<Vec<i64>>) -> Option<i64> {
            use std::collections::HashMap;
            let mut x = vec![];
            let mut y = vec![];
            for r in rectangles.iter() {
                x.push(r[0]);
                x.push(r[2]);
                y.push(r[1]);
                y.push(r[3]);
            }
            x.sort_unstable();
            y.sort_unstable();
            x.dedup();
            y.dedup();
            let mut x_map = HashMap::new();
            let mut y_map = HashMap::new();
            for (i, &v) in x.iter().enumerate() {
                x_map.insert(v, i);
            }
            for (i, &v) in y.iter().enumerate() {
                y_map.insert(v, i);
            }
            let mut grid = vec![vec![0; y.len()]; x.len()];
            for r in rectangles.iter() {
                let x1 = *x_map.get(&r[0])?;
                let x2 = *x_map.get(&r[2])?;
                let y1 = *y_map.get(&r[1])?;
                let y2 = *y_map.get(&r[3])?;
                for item in grid.iter_mut().take(x2).skip(x1) {
                    for point in item.iter_mut().take(y2).skip(y1) {
                        *point = 1;
                    }
                }
            }
            let mut ans = 0;
            for i in 0..x.len() {
                for j in 0..y.len() {
                    if grid[i][j] == 1 {
                        ans += (x[i + 1] - x[i]) * (y[j + 1] - y[j]);
                        ans %= 1_000_000_007;
                    }
                }
            }
            Some(ans)
        }

        let f = |r: &Vec<i32>| -> Vec<i64> { r.iter().map(|&v| v as i64).collect() };
        let rectangles = rectangles.iter().map(f).collect();
        _rectangle_area(rectangles).unwrap_or_default() as i32
    }
}

#[test]
fn test() {
    let rectangles = vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]];
    assert_eq!(Solution::rectangle_area(rectangles), 6);
    let rectangles = vec![vec![0, 0, 1000000000, 1000000000]];
    assert_eq!(Solution::rectangle_area(rectangles), 49);
}
