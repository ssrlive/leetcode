#![allow(dead_code)]

// 3568. Minimum Moves to Clean the Classroom
// https://leetcode.com/problems/minimum-moves-to-clean-the-classroom/
// https://leetcode.cn/problems/minimum-moves-to-clean-the-classroom/
//
// Medium
//
// You are given an m x n grid classroom where a student volunteer is tasked with cleaning up litter scattered around the room.
// Each cell in the grid is one of the following:
//
//     'S': Starting position of the student
//     'L': Litter that must be collected (once collected, the cell becomes empty)
//     'R': Reset area that restores the student's energy to full capacity, regardless of their current energy level (can be used multiple times)
//     'X': Obstacle the student cannot pass through
//     '.': Empty space
//
// You are also given an integer energy, representing the student's maximum energy capacity. The student starts with this energy from the starting position 'S'.
//
// Each move to an adjacent cell (up, down, left, or right) costs 1 unit of energy. If the energy reaches 0, the student can only
// continue if they are on a reset area 'R', which resets the energy to its maximum capacity energy.
//
// Return the minimum number of moves required to collect all litter items, or -1 if it's impossible.
//
// Example 1:
//
// Input: classroom = ["S.", "XL"], energy = 2
//
// Output: 2
//
// Explanation:
//
//     The student starts at cell (0, 0) with 2 units of energy.
//     Since cell (1, 0) contains an obstacle 'X', the student cannot move directly downward.
//     A valid sequence of moves to collect all litter is as follows:
//         Move 1: From (0, 0) → (0, 1) with 1 unit of energy and 1 unit remaining.
//         Move 2: From (0, 1) → (1, 1) to collect the litter 'L'.
//     The student collects all the litter using 2 moves. Thus, the output is 2.
//
// Example 2:
//
// Input: classroom = ["LS", "RL"], energy = 4
//
// Output: 3
//
// Explanation:
//
//     The student starts at cell (0, 1) with 4 units of energy.
//     A valid sequence of moves to collect all litter is as follows:
//         Move 1: From (0, 1) → (0, 0) to collect the first litter 'L' with 1 unit of energy used and 3 units remaining.
//         Move 2: From (0, 0) → (1, 0) to 'R' to reset and restore energy back to 4.
//         Move 3: From (1, 0) → (1, 1) to collect the second litter 'L'.
//     The student collects all the litter using 3 moves. Thus, the output is 3.
//
// Example 3:
//
// Input: classroom = ["L.S", "RXL"], energy = 3
//
// Output: -1
//
// Explanation:
//
// No valid path collects all 'L'.
//
// Constraints:
//
//     1 <= m == classroom.length <= 20
//     1 <= n == classroom[i].length <= 20
//     classroom[i][j] is one of 'S', 'L', 'R', 'X', or '.'
//     1 <= energy <= 50
//     There is exactly one 'S' in the grid.
//     There are at most 10 'L' cells in the grid.
//

struct Solution;

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        const DR: [i32; 4] = [-1, 1, 0, 0];
        const DC: [i32; 4] = [0, 0, -1, 1];

        let row_cnt = classroom.len();
        let col_cnt = classroom[0].len();

        let mut start_r = -1;
        let mut start_c = -1;
        let mut litter_item_coords = Vec::new();
        let mut litter_coord_to_idx_map = std::collections::BTreeMap::new();
        for (i, classroom_i) in classroom.iter().enumerate().take(row_cnt) {
            for j in 0..col_cnt {
                if classroom_i.as_bytes()[j] == b'S' {
                    start_r = i as i32;
                    start_c = j as i32;
                } else if classroom_i.as_bytes()[j] == b'L' {
                    litter_coord_to_idx_map.insert((i as i32, j as i32), litter_item_coords.len());
                    litter_item_coords.push((i as i32, j as i32));
                }
            }
        }
        let num_litter = litter_item_coords.len();
        if num_litter == 0 {
            return 0;
        }
        let target_mask = (1 << num_litter) - 1;
        let mut dist = vec![vec![vec![vec![-1; 1 << num_litter]; energy as usize + 1]; col_cnt]; row_cnt];
        let mut q = std::collections::VecDeque::new();
        dist[start_r as usize][start_c as usize][energy as usize][0] = 0;
        q.push_back((start_r, start_c, energy, 0));
        while let Some((r, c, current_e, mask)) = q.pop_front() {
            let moves = dist[r as usize][c as usize][current_e as usize][mask];
            if mask == target_mask {
                return moves;
            }
            for i in 0..4 {
                let nr = r + DR[i];
                let nc = c + DC[i];
                if nr >= 0
                    && nr < row_cnt as i32
                    && nc >= 0
                    && nc < col_cnt as i32
                    && classroom[nr as usize].as_bytes()[nc as usize] != b'X'
                    && current_e > 0
                {
                    let energy_after_move = current_e - 1;
                    let next_total_moves = moves + 1;
                    let mut energy_at_new_cell = energy_after_move;
                    let mut new_mask_after_move = mask;
                    let destination_cell_char = classroom[nr as usize].as_bytes()[nc as usize];
                    if destination_cell_char == b'L' {
                        let litter_idx = litter_coord_to_idx_map[&(nr, nc)];
                        new_mask_after_move |= 1 << litter_idx;
                    }
                    if destination_cell_char == b'R' {
                        energy_at_new_cell = energy;
                    }
                    if dist[nr as usize][nc as usize][energy_at_new_cell as usize][new_mask_after_move] == -1
                        || next_total_moves < dist[nr as usize][nc as usize][energy_at_new_cell as usize][new_mask_after_move]
                    {
                        dist[nr as usize][nc as usize][energy_at_new_cell as usize][new_mask_after_move] = next_total_moves;
                        q.push_back((nr, nc, energy_at_new_cell, new_mask_after_move));
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let classroom = vec!["S.".to_string(), "XL".to_string()];
    let energy = 2;
    assert_eq!(Solution::min_moves(classroom, energy), 2);

    let classroom = vec!["LS".to_string(), "RL".to_string()];
    let energy = 4;
    assert_eq!(Solution::min_moves(classroom, energy), 3);

    let classroom = vec!["L.S".to_string(), "RXL".to_string()];
    let energy = 3;
    assert_eq!(Solution::min_moves(classroom, energy), -1);
}
