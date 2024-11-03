#![allow(dead_code)]

// 3341. Find Minimum Time to Reach Last Room I
// https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i/
// https://leetcode.cn/problems/find-minimum-time-to-reach-last-room-i/
//
// Medium
//
// There is a dungeon with n x m rooms arranged as a grid.
//
// You are given a 2D array moveTime of size n x m, where moveTime[i][j] represents the minimum time in seconds
// when you can start moving to that room. You start from the room (0, 0) at time t = 0 and can move to an adjacent room.
// Moving between adjacent rooms takes exactly one second.
//
// Return the minimum time to reach the room (n - 1, m - 1).
//
// Two rooms are adjacent if they share a common wall, either horizontally or vertically.
//
// Example 1:
//
// Input: moveTime = [[0,4],[4,4]]
//
// Output: 6
//
// Explanation:
//
// The minimum time required is 6 seconds.
//
//     At time t == 4, move from room (0, 0) to room (1, 0) in one second.
//     At time t == 5, move from room (1, 0) to room (1, 1) in one second.
//
// Example 2:
//
// Input: moveTime = [[0,0,0],[0,0,0]]
//
// Output: 3
//
// Explanation:
//
// The minimum time required is 3 seconds.
//
//     At time t == 0, move from room (0, 0) to room (1, 0) in one second.
//     At time t == 1, move from room (1, 0) to room (1, 1) in one second.
//     At time t == 2, move from room (1, 1) to room (1, 2) in one second.
//
// Example 3:
//
// Input: moveTime = [[0,1],[1,2]]
//
// Output: 3
//
// Constraints:
//
//     2 <= n == moveTime.length <= 50
//     2 <= m == moveTime[i].length <= 50
//     0 <= moveTime[i][j] <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let rows = move_time.len() as i32;
        let cols = move_time[0].len() as i32;
        let mut min_heap = std::collections::BinaryHeap::new();
        let mut time = vec![vec![i32::MAX; cols as usize]; rows as usize];

        min_heap.push(std::cmp::Reverse((0, 0, 0))); // time, x, y
        time[0][0] = 0;

        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some(std::cmp::Reverse((current_time, x, y))) = min_heap.pop() {
            if x == rows - 1 && y == cols - 1 {
                return current_time;
            }

            for &(dx, dy) in &directions {
                let new_x = x + dx;
                let new_y = y + dy;

                if new_x >= 0 && new_x < rows && new_y >= 0 && new_y < cols {
                    let wait_time = std::cmp::max(move_time[new_x as usize][new_y as usize] - current_time, 0);
                    let new_time = current_time + 1 + wait_time;

                    if new_time < time[new_x as usize][new_y as usize] {
                        time[new_x as usize][new_y as usize] = new_time;
                        min_heap.push(std::cmp::Reverse((new_time, new_x, new_y)));
                    }
                }
            }
        }

        -1 // unreachable
    }
}

#[test]
fn test() {
    let move_time = vec![vec![0, 4], vec![4, 4]];
    let res = 6;
    assert_eq!(Solution::min_time_to_reach(move_time), res);

    let move_time = vec![vec![0, 0, 0], vec![0, 0, 0]];
    let res = 3;
    assert_eq!(Solution::min_time_to_reach(move_time), res);

    let move_time = vec![vec![0, 1], vec![1, 2]];
    let res = 3;
    assert_eq!(Solution::min_time_to_reach(move_time), res);

    let move_time = vec![vec![15, 58], vec![67, 4]];
    let res = 60;
    assert_eq!(Solution::min_time_to_reach(move_time), res);
}
