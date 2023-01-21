#![allow(dead_code)]

// 1263. Minimum Moves to Move a Box to Their Target Location
// https://leetcode.com/problems/minimum-moves-to-move-a-box-to-their-target-location/
// https://leetcode.cn/problems/minimum-moves-to-move-a-box-to-their-target-location/
//
// Hard
//
// A storekeeper is a game in which the player pushes boxes around in a warehouse trying to get them to target locations.
//
// The game is represented by an m x n grid of characters grid where each element is a wall, floor, or box.
//
// Your task is to move the box 'B' to the target position 'T' under the following rules:
//
//     The character 'S' represents the player. The player can move up, down, left, right in grid if it is a floor (empty cell).
//     The character '.' represents the floor which means a free cell to walk.
//     The character '#' represents the wall which means an obstacle (impossible to walk there).
//     There is only one box 'B' and one target cell 'T' in the grid.
//     The box can be moved to an adjacent free cell by standing next to the box and then moving in the direction of the box. This is a push.
//     The player cannot walk through the box.
//
// Return the minimum number of pushes to move the box to the target. If there is no way to reach the target, return -1.
//
// Example 1:
//
// Input: grid = [["#","#","#","#","#","#"],
//                ["#","T","#","#","#","#"],
//                ["#",".",".","B",".","#"],
//                ["#",".","#","#",".","#"],
//                ["#",".",".",".","S","#"],
//                ["#","#","#","#","#","#"]]
// Output: 3
// Explanation: We return only the number of times the box is pushed.
//
// Example 2:
//
// Input: grid = [["#","#","#","#","#","#"],
//                ["#","T","#","#","#","#"],
//                ["#",".",".","B",".","#"],
//                ["#","#","#","#",".","#"],
//                ["#",".",".",".","S","#"],
//                ["#","#","#","#","#","#"]]
// Output: -1
//
// Example 3:
//
// Input: grid = [["#","#","#","#","#","#"],
//                ["#","T",".",".","#","#"],
//                ["#",".","#","B",".","#"],
//                ["#",".",".",".",".","#"],
//                ["#",".",".",".","S","#"],
//                ["#","#","#","#","#","#"]]
// Output: 5
// Explanation: push the box down, left, left, up and up.
//
// Constraints:
//
// -    m == grid.length
// -    n == grid[i].length
// -    1 <= m, n <= 20
// -    grid contains only characters '.', '#', 'S', 'T', or 'B'.
// -    There is only one character 'S', 'B', and 'T' in the grid.
//

struct Solution;

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashSet};

        const UNREACHABLE: i32 = -1;
        const DIR: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

        const OBSTACLE: char = '#';
        const BOX: char = 'B';
        const PERSON: char = 'S';
        const TARGET: char = 'T';

        let mut visited = HashSet::new();
        let (mut pr, mut pc) = (0, 0);
        let (mut br, mut bc) = (0, 0);
        let (mut tr, mut tc) = (0, 0);

        for (r, item) in grid.iter().enumerate() {
            for (c, &item2) in item.iter().enumerate() {
                match item2 {
                    BOX => {
                        br = r;
                        bc = c;
                    }
                    PERSON => {
                        pr = r;
                        pc = c;
                    }
                    TARGET => {
                        tr = r;
                        tc = c;
                    }
                    _ => {}
                }
            }
        }

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), br, bc, pr, pc));
        visited.insert((br, bc, pr, pc));

        while let Some((Reverse(cost), br, bc, pr, pc)) = pq.pop() {
            if (tr, tc) == (br, bc) {
                return cost;
            }

            for (dr, dc) in DIR.iter().copied() {
                let rx = pr as isize + dr;
                let cx = pc as isize + dc;

                if rx < 0 || cx < 0 {
                    continue;
                }

                let pr_new = rx as usize;
                let pc_new = cx as usize;

                if pr_new >= grid.len() || pc_new >= grid[pr_new].len() {
                    continue;
                }

                if grid[pr_new][pc_new] == OBSTACLE {
                    continue;
                }

                let mut cost_new = cost;
                let mut br_new = br;
                let mut bc_new = bc;

                if (pr_new, pc_new) == (br_new, bc_new) {
                    let rx = br_new as isize + dr;
                    let cx = bc_new as isize + dc;

                    if rx < 0 || cx < 0 {
                        continue;
                    }

                    br_new = rx as usize;
                    bc_new = cx as usize;

                    if br_new >= grid.len() || bc_new >= grid[br_new].len() {
                        continue;
                    }

                    if grid[br_new][bc_new] == OBSTACLE {
                        continue;
                    }

                    cost_new += 1;
                }

                if !visited.insert((br_new, bc_new, pr_new, pc_new)) {
                    continue;
                }

                pq.push((Reverse(cost_new), br_new, bc_new, pr_new, pc_new));
            }
        }

        UNREACHABLE
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec!['#', '#', '#', '#', '#', '#'],
                vec!['#', 'T', '#', '#', '#', '#'],
                vec!['#', '.', '.', 'B', '.', '#'],
                vec!['#', '.', '#', '#', '.', '#'],
                vec!['#', '.', '.', '.', 'S', '#'],
                vec!['#', '#', '#', '#', '#', '#'],
            ],
            3,
        ),
        (
            vec![
                vec!['#', '#', '#', '#', '#', '#'],
                vec!['#', 'T', '#', '#', '#', '#'],
                vec!['#', '.', '.', 'B', '.', '#'],
                vec!['#', '#', '#', '#', '.', '#'],
                vec!['#', '.', '.', '.', 'S', '#'],
                vec!['#', '#', '#', '#', '#', '#'],
            ],
            -1,
        ),
        (
            vec![
                vec!['#', '#', '#', '#', '#', '#'],
                vec!['#', 'T', '.', '.', '#', '#'],
                vec!['#', '.', '#', 'B', '.', '#'],
                vec!['#', '.', '.', '.', '.', '#'],
                vec!['#', '.', '.', '.', 'S', '#'],
                vec!['#', '#', '#', '#', '#', '#'],
            ],
            5,
        ),
    ];
    for (grid, expected) in cases {
        assert_eq!(Solution::min_push_box(grid), expected);
    }
}
