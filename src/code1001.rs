#![allow(dead_code)]

// 1001. Grid Illumination
// https://leetcode.com/problems/grid-illumination/
// https://leetcode.cn/problems/grid-illumination/
//
// There is a 2D grid of size n x n where each cell of this grid has a lamp that is initially turned off.
//
// You are given a 2D array of lamp positions lamps, where lamps[i] = [rowi, coli] indicates that the lamp at
// grid[rowi][coli] is turned on. Even if the same lamp is listed more than once, it is turned on.
//
// When a lamp is turned on, it illuminates its cell and all other cells in the same row, column, or diagonal.
//
// You are also given another 2D array queries, where queries[j] = [rowj, colj]. For the jth query, determine
// whether grid[rowj][colj] is illuminated or not. After answering the jth query, turn off the lamp at grid[rowj][colj]
// and its 8 adjacent lamps if they exist. A lamp is adjacent if its cell shares either a side or corner with grid[rowj][colj].
//
// Return an array of integers ans, where ans[j] should be 1 if the cell in the jth query was illuminated, or 0 if the lamp was not.
//
// Example 1:
//
// Input: n = 5, lamps = [[0,0],[4,4]], queries = [[1,1],[1,0]]
// Output: [1,0]
// Explanation: We have the initial grid with all lamps turned off. In the above picture we see the
// grid after turning on the lamp at grid[0][0] then turning on the lamp at grid[4][4].
// The 0th query asks if the lamp at grid[1][1] is illuminated or not (the blue square).
// It is illuminated, so set ans[0] = 1. Then, we turn off all lamps in the red square.
//
// The 1st query asks if the lamp at grid[1][0] is illuminated or not (the blue square).
// It is not illuminated, so set ans[1] = 0. Then, we turn off all lamps in the red rectangle.
//
// Example 2:
//
// Input: n = 5, lamps = [[0,0],[4,4]], queries = [[1,1],[1,1]]
// Output: [1,1]
//
// Example 3:
//
// Input: n = 5, lamps = [[0,0],[0,4]], queries = [[0,4],[0,1],[1,4]]
// Output: [1,1,0]
//
// Constraints:
//
// - 1 <= n <= 10^9
// - 0 <= lamps.length <= 20000
// - 0 <= queries.length <= 20000
// - lamps[i].length == 2
// - 0 <= rowi, coli < n
// - queries[j].length == 2
// - 0 <= rowj, colj < n
//

struct Solution;

impl Solution {
    pub fn grid_illumination(_n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};
        let mut res = vec![];
        let mut x = HashMap::new();
        let mut y = HashMap::new();
        let mut a_d = HashMap::new();
        let mut d_d = HashMap::new();
        let mut ls = HashMap::<i32, HashSet<i32>>::new();
        for l in lamps {
            let i = l[0];
            let j = l[1];
            if ls.entry(i).or_default().insert(j) {
                *x.entry(i).or_insert(0) += 1;
                *y.entry(j).or_insert(0) += 1;
                *a_d.entry(i + j).or_insert(0) += 1;
                *d_d.entry(i - j).or_insert(0) += 1;
            }
        }
        for q in queries {
            let i = q[0];
            let j = q[1];
            if *x.get(&i).unwrap_or(&0) != 0
                || *y.get(&j).unwrap_or(&0) != 0
                || *a_d.get(&(i + j)).unwrap_or(&0) != 0
                || *d_d.get(&(i - j)).unwrap_or(&0) != 0
            {
                res.push(1);
                for li in i - 1..=i + 1 {
                    for lj in j - 1..=j + 1 {
                        if let Some(s) = ls.get_mut(&li) {
                            if s.remove(&lj) {
                                *x.get_mut(&li).unwrap() -= 1;
                                *y.get_mut(&lj).unwrap() -= 1;
                                *a_d.get_mut(&(li + lj)).unwrap() -= 1;
                                *d_d.get_mut(&(li - lj)).unwrap() -= 1;
                            }
                        }
                    }
                }
            } else {
                res.push(0);
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            5,
            vec![vec![0, 0], vec![4, 4]],
            vec![vec![1, 1], vec![1, 0]],
            vec![1, 0],
        ),
        (
            5,
            vec![vec![0, 0], vec![4, 4]],
            vec![vec![1, 1], vec![1, 1]],
            vec![1, 1],
        ),
        (
            5,
            vec![vec![0, 0], vec![0, 4]],
            vec![vec![0, 4], vec![0, 1], vec![1, 4]],
            vec![1, 1, 0],
        ),
    ];
    for (n, lamps, queries, res) in cases {
        assert_eq!(Solution::grid_illumination(n, lamps, queries), res);
    }
}
