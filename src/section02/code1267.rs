#![allow(dead_code)]

// 1267. Count Servers that Communicate
// https://leetcode.com/problems/count-servers-that-communicate/
// https://leetcode.cn/problems/count-servers-that-communicate/
//
// Medium
//
// You are given a map of a server center, represented as a m * n integer matrix grid,
// where 1 means that on that cell there is a server and 0 means that it is no server.
// Two servers are said to communicate if they are on the same row or on the same column.
//
// Return the number of servers that communicate with any other server.
//
// Example 1:
//
// Input: grid = [[1,0],[0,1]]
// Output: 0
// Explanation: No servers can communicate with others.
//
// Example 2:
//
// Input: grid = [[1,0],[1,1]]
// Output: 3
// Explanation: All three servers can communicate with at least one other server.
//
// Example 3:
//
// Input: grid = [[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]]
// Output: 4
// Explanation: The two servers in the first row can communicate with each other.
// The two servers in the third column can communicate with each other.
// The server at right bottom corner can't communicate with any other server.
//
// Constraints:
//
// -    m == grid.length
// -    n == grid[i].length
// -    1 <= m <= 250
// -    1 <= n <= 250
// -    grid[i][j] == 0 or 1
//

struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut c = vec![0; grid.len()];
        let mut r = vec![0; grid[0].len()];
        for i in 0..grid.len() {
            for (j, item) in r.iter_mut().enumerate() {
                if grid[i][j] == 1 {
                    c[i] += 1;
                    *item += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..grid.len() {
            for (j, &item) in r.iter().enumerate() {
                if grid[i][j] == 1 && (c[i] > 1 || item > 1) {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let grid = vec![vec![1, 0], vec![0, 1]];
    assert_eq!(Solution::count_servers(grid), 0);

    let grid = vec![vec![1, 0], vec![1, 1]];
    assert_eq!(Solution::count_servers(grid), 3);

    let grid = vec![vec![1, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]];
    assert_eq!(Solution::count_servers(grid), 4);
}
