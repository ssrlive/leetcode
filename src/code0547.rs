#![allow(dead_code)]

// 547. Number of Provinces
// https://leetcode.com/problems/number-of-provinces/
// https://leetcode.cn/problems/number-of-provinces/
//
// There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.
//
// A province is a group of directly or indirectly connected cities and no other cities outside of the group.
//
// You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the ith city and the jth city are directly connected, and isConnected[i][j] = 0 otherwise.
//
// Return the total number of provinces.
//
// Example 1:
//
// Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
// Output: 2
//
// Example 2:
//
// Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
// Output: 3
//
// Constraints:
//
// - 1 <= n <= 200
// - n == isConnected.length
// - n == isConnected[i].length
// - isConnected[i][j] is 1 or 0.
// - isConnected[i][i] == 1
// - isConnected[i][j] == isConnected[j][i]
//

struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; is_connected.len()];
        let mut count = 0;
        for i in 0..is_connected.len() {
            if !visited[i] {
                Self::dfs(&is_connected, &mut visited, i);
                count += 1;
            }
        }
        count
    }
    fn dfs(is_connected: &[Vec<i32>], visited: &mut Vec<bool>, i: usize) {
        for j in 0..is_connected.len() {
            if is_connected[i][j] == 1 && !visited[j] {
                visited[j] = true;
                Self::dfs(is_connected, visited, j);
            }
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]], 2),
        (vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]], 3),
    ];
    for (is_connected, expected) in cases {
        assert_eq!(Solution::find_circle_num(is_connected), expected);
    }
}
