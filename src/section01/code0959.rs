#![allow(dead_code)]

// 959. Regions Cut By Slashes
// https://leetcode.com/problems/regions-cut-by-slashes/
// https://leetcode.cn/problems/regions-cut-by-slashes/
//
// An n x n grid is composed of 1 x 1 squares where each 1 x 1 square consists of a '/', '\', or blank space ' '.
// These characters divide the square into contiguous regions.
//
// Given the grid grid represented as a string array, return the number of regions.
//
// Note that backslash characters are escaped, so a '\' is represented as '\\'.
//
// Example 1:
//
// Input: grid = [" /","/ "]
// Output: 2
//
// Example 2:
//
// Input: grid = [" /","  "]
// Output: 1
//
// Example 3:
//
// Input: grid = ["/\\","\\/"]
// Output: 5
// Explanation: Recall that because \ characters are escaped, "\\/" refers to \/, and "/\\" refers to /\.
//
// Constraints:
//
// - n == grid.length == grid[i].length
// - 1 <= n <= 30
// - grid[i][j] is either '/', '\', or ' '.
//

struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        fn count(graph: &mut Vec<Vec<i32>>) -> i32 {
            let n = graph.len();
            let mut ret = 0;
            for i in 0..n {
                for j in 0..n {
                    if graph[i][j] == 1 {
                        continue;
                    }
                    dfs(graph, i, j);
                    ret += 1;
                }
            }
            ret
        }

        fn dfs(graph: &mut Vec<Vec<i32>>, i: usize, j: usize) {
            if graph[i][j] == 1 {
                return;
            }
            graph[i][j] = 1;
            if i > 0 {
                dfs(graph, i - 1, j);
            }
            if i + 1 < graph.len() {
                dfs(graph, i + 1, j);
            }
            if j > 0 {
                dfs(graph, i, j - 1);
            }
            if j + 1 < graph.len() {
                dfs(graph, i, j + 1);
            }
        }

        let mut graph = vec![];
        for g in grid {
            let mut v = vec![vec![]; 3];
            for c in g.chars() {
                let mut data = vec![vec![0; 3]; 3];
                if c == '/' {
                    for (i, item) in data.iter_mut().enumerate() {
                        item[2 - i] = 1;
                    }
                }
                if c == '\\' {
                    for (i, item) in data.iter_mut().enumerate() {
                        item[i] = 1;
                    }
                }
                for i in 0..3 {
                    for j in 0..3 {
                        v[i].push(data[i][j]);
                    }
                }
            }
            for it in v {
                graph.push(it);
            }
        }
        count(&mut graph)
    }
}

#[test]
fn test() {
    let grid = vec![" /".to_string(), "/ ".to_string()];
    assert_eq!(Solution::regions_by_slashes(grid), 2);

    let grid = vec![" /".to_string(), "  ".to_string()];
    assert_eq!(Solution::regions_by_slashes(grid), 1);

    let grid = vec!["/\\".to_string(), "\\/".to_string()];
    assert_eq!(Solution::regions_by_slashes(grid), 5);

    let grid = vec!["//".to_string(), "/ ".to_string()];
    assert_eq!(Solution::regions_by_slashes(grid), 3);
}
