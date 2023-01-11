#![allow(dead_code)]

// 1036. Escape a Large Maze
// https://leetcode.com/problems/escape-a-large-maze/
// https://leetcode.cn/problems/escape-a-large-maze/
//
// There is a 1 million by 1 million grid on an XY-plane, and the coordinates of each grid square are (x, y).
//
// We start at the source = [sx, sy] square and want to reach the target = [tx, ty] square. There is also an array
// of blocked squares, where each blocked[i] = [xi, yi] represents a blocked square with coordinates (xi, yi).
//
// Each move, we can walk one square north, east, south, or west if the square is not in the array of blocked squares.
// We are also not allowed to walk outside of the grid.
//
// Return true if and only if it is possible to reach the target square from the source square through a sequence of valid moves.
//
// Example 1:
//
// Input: blocked = [[0,1],[1,0]], source = [0,0], target = [0,2]
// Output: false
// Explanation: The target square is inaccessible starting from the source square because we cannot move.
// We cannot move north or east because those squares are blocked.
// We cannot move south or west because we cannot go outside of the grid.
//
// Example 2:
//
// Input: blocked = [], source = [0,0], target = [999999,999999]
// Output: true
// Explanation: Because there are no blocked cells, it is possible to reach the target square.
//
// Constraints:
//
// - 0 <= blocked.length <= 200
// - blocked[i].length == 2
// - 0 <= xi, yi < 10^6
// - source.length == target.length == 2
// - 0 <= sx, sy, tx, ty < 10^6
// - source != target
// - It is guaranteed that source and target are not blocked.
//

struct Solution;

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        use std::collections::{HashSet, VecDeque};
        fn can_reach(source: &[i32], target: &[i32], num_blocked: i32, block: &HashSet<(i32, i32)>) -> bool {
            let dx = vec![0, 0, 1, -1];
            let dy = vec![1, -1, 0, 0];
            let max_area = num_blocked * (num_blocked - 1) / 2;
            let mut curr_area = 1;
            let mut visited = HashSet::new();
            let mut q = VecDeque::new();
            q.push_back((source[0], source[1]));
            visited.insert((source[0], source[1]));
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                for i in 0..4 {
                    let new_x = x + dx[i];
                    let new_y = y + dy[i];
                    if new_x == target[0] && new_y == target[1] {
                        return true;
                    }
                    if curr_area > max_area {
                        return true;
                    }
                    if new_x >= 0
                        && new_x < 1e6 as i32
                        && new_y >= 0
                        && new_y < 1e6 as i32
                        && !block.contains(&(new_x, new_y))
                        && !visited.contains(&(new_x, new_y))
                    {
                        curr_area += 1;
                        visited.insert((new_x, new_y));
                        q.push_back((new_x, new_y));
                    }
                }
            }
            false
        }

        let mut block = HashSet::new();
        let mut count = 0;
        for item in &blocked {
            count += 1;
            block.insert((item[0], item[1]));
        }
        can_reach(&source, &target, count, &block) && can_reach(&target, &source, count, &block)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 1], vec![1, 0]], vec![0, 0], vec![0, 2], false),
        (vec![], vec![0, 0], vec![999999, 999999], true),
    ];
    for (blocked, source, target, expected) in cases {
        assert_eq!(Solution::is_escape_possible(blocked, source, target), expected);
    }
}
