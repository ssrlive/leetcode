#![allow(dead_code)]

// 913. Cat and Mouse
// https://leetcode.com/problems/cat-and-mouse/
// https://leetcode.cn/problems/cat-and-mouse/
//
// A game on an undirected graph is played by two players, Mouse and Cat, who alternate turns.
//
// The graph is given as follows: graph[a] is a list of all nodes b such that ab is an edge of the graph.
//
// The mouse starts at node 1 and goes first, the cat starts at node 2 and goes second, and there is a hole at node 0.
//
// During each player's turn, they must travel along one edge of the graph that meets where they are.
// For example, if the Mouse is at node 1, it must travel to any node in graph[1].
//
// Additionally, it is not allowed for the Cat to travel to the Hole (node 0.)
//
// Then, the game can end in three ways:
//
// If ever the Cat occupies the same node as the Mouse, the Cat wins.
// If ever the Mouse reaches the Hole, the Mouse wins.
// If ever a position is repeated (i.e., the players are in the same position as a previous turn,
//     and it is the same player's turn to move), the game is a draw.
// Given a graph, and assuming both players play optimally, return
//
// 1 if the mouse wins the game,
// 2 if the cat wins the game, or
// 0 if the game is a draw.
//
// Example 1:
//
// Input: graph = [[2,5],[3],[0,4,5],[1,4,5],[2,3],[0,2,3]]
// Output: 0
//
// Example 2:
//
// Input: graph = [[1,3],[0],[3],[0,2]]
// Output: 1
//
// Constraints:
//
// - 3 <= graph.length <= 50
// - 1 <= graph[i].length < graph.length
// - 0 <= graph[i][j] < graph.length
// - graph[i][j] != i
// - graph[i] is unique.
// - The mouse and the cat can always move.
//

struct Solution;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let len = graph.len();
        let mut dp = vec![vec![vec![0; len]; len]; 2];
        let mut outdegree = vec![vec![vec![0; len]; len]; 2];
        let mut q = VecDeque::new();

        let (dp0, dp1) = dp.split_at_mut(1);
        dp0[0][0]
            .iter_mut()
            .zip(dp1[0][0].iter_mut())
            .enumerate()
            .for_each(|(j, (val0, val1))| {
                *val0 = 1;
                *val1 = 1;
                q.push_back((0, 0, j));
                q.push_back((1, 0, j));
            });

        let (dp0, dp1) = dp.split_at_mut(1);
        dp0[0]
            .iter_mut()
            .zip(dp1[0].iter_mut())
            .enumerate()
            .skip(1)
            .for_each(|(j, (dp0_row, dp1_row))| {
                dp0_row[j] = 2;
                dp1_row[j] = 2;
                q.push_back((0, j, j));
                q.push_back((1, j, j));
            });

        for i in 0..len {
            for j in 1..len {
                outdegree[0][i][j] = graph[i].len() as i32;
                outdegree[1][i][j] = graph[j].len() as i32;
            }
        }
        for &v in &graph[0] {
            outdegree[1].iter_mut().for_each(|row| row[v as usize] -= 1);
        }
        while let Some((turn, mouse, cat)) = q.pop_front() {
            if turn == 0 && mouse == 1 && cat == 2 {
                break;
            }
            if turn == 0 {
                for &v in &graph[cat] {
                    let v = v as usize;
                    if v == 0 {
                        continue;
                    }
                    if dp[1][mouse][v] > 0 {
                        continue;
                    }
                    if dp[turn][mouse][cat] == 2 {
                        dp[1][mouse][v] = 2;
                        q.push_back((1, mouse, v));
                        continue;
                    }
                    outdegree[1][mouse][v] -= 1;
                    if outdegree[1][mouse][v] == 0 {
                        dp[1][mouse][v] = 1;
                        q.push_back((1, mouse, v));
                    }
                }
            } else {
                for &v in &graph[mouse] {
                    let v = v as usize;
                    if dp[0][v][cat] > 0 {
                        continue;
                    }
                    if dp[turn][mouse][cat] == 1 {
                        dp[0][v][cat] = 1;
                        q.push_back((0, v, cat));
                        continue;
                    }
                    outdegree[0][v][cat] -= 1;
                    if outdegree[0][v][cat] == 0 {
                        dp[0][v][cat] = 2;
                        q.push_back((0, v, cat));
                    }
                }
            }
        }
        dp[0][1][2]
    }
}

#[test]
fn test() {
    let graph = vec![vec![2, 5], vec![3], vec![0, 4, 5], vec![1, 4, 5], vec![2, 3], vec![0, 2, 3]];
    let res = 0;
    assert_eq!(Solution::cat_mouse_game(graph), res);

    let graph = vec![vec![1, 3], vec![0], vec![3], vec![0, 2]];
    let res = 1;
    assert_eq!(Solution::cat_mouse_game(graph), res);

    let graph = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
    let res = 1;
    assert_eq!(Solution::cat_mouse_game(graph), res);

    let graph = vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]];
    let res = 1;
    assert_eq!(Solution::cat_mouse_game(graph), res);
}
