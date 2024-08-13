#![allow(dead_code)]

// 3248. Snake in Matrix
// https://leetcode.com/problems/snake-in-matrix/
// https://leetcode.cn/problems/snake-in-matrix/
//
// Easy
//
// There is a snake in an n x n matrix grid and can move in four possible directions.
// Each cell in the grid is identified by the position: grid[i][j] = (i * n) + j.
//
// The snake starts at cell 0 and follows a sequence of commands.
//
// You are given an integer n representing the size of the grid and an array of strings commands
// where each command[i] is either "UP", "RIGHT", "DOWN", and "LEFT".
// It's guaranteed that the snake will remain within the grid boundaries throughout its movement.
//
// Return the position of the final cell where the snake ends up after executing commands.
//
// Example 1:
//
// Input: n = 2, commands = ["RIGHT","DOWN"]
//
// Output: 3
//
// Example 2:
//
// Input: n = 3, commands = ["DOWN","RIGHT","UP"]
//
// Output: 1
//
// Constraints:
//
//     2 <= n <= 10
//     1 <= commands.length <= 100
//     commands consists only of "UP", "RIGHT", "DOWN", and "LEFT".
//     The input is generated such the snake will not move outside of the boundaries.
//

struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut res = 0;
        for c in commands {
            match c.as_str() {
                "RIGHT" => res += 1,
                "LEFT" => res -= 1,
                "UP" => res -= n,
                _ => res += n,
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::final_position_of_snake(2, vec!["RIGHT".to_string(), "DOWN".to_string()]),
        3
    );
    assert_eq!(
        Solution::final_position_of_snake(3, vec!["DOWN".to_string(), "RIGHT".to_string(), "UP".to_string()]),
        1
    );
}
