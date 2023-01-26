#![allow(dead_code)]

// 1349. Maximum Students Taking Exam
// https://leetcode.com/problems/maximum-students-taking-exam/
// https://leetcode.cn/problems/maximum-students-taking-exam/
//
// Hard
//
// Given a m * n matrix seats  that represent seats distributions in a classroom. If a seat is broken,
// it is denoted by '#' character otherwise it is denoted by a '.' character.
//
// Students can see the answers of those sitting next to the left, right, upper left and upper right,
// but he cannot see the answers of the student sitting directly in front or behind him.
// Return the maximum number of students that can take the exam together without any cheating being possible..
//
// Students must be placed in seats in good condition.
//
// Example 1:
//
// Input: seats = [["#",".","#","#",".","#"],
//                 [".","#","#","#","#","."],
//                 ["#",".","#","#",".","#"]]
// Output: 4
// Explanation: Teacher can place 4 students in available seats so they don't cheat on the exam.
//
// Example 2:
//
// Input: seats = [[".","#"],
//                 ["#","#"],
//                 ["#","."],
//                 ["#","#"],
//                 [".","#"]]
// Output: 3
// Explanation: Place all students in available seats.
//
// Example 3:
//
// Input: seats = [["#",".",".",".","#"],
//                 [".","#",".","#","."],
//                 [".",".","#",".","."],
//                 [".","#",".","#","."],
//                 ["#",".",".",".","#"]]
// Output: 10
// Explanation: Place students in available seats in column 1, 3 and 5.
//
// Constraints:
//
// -    seats contains only characters '.' and'#'.
// -    m == seats.length
//     n == seats[i].length
// -    1 <= m <= 8
// -    1 <= n <= 8
//

struct Solution;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let m = seats.len();
        let n = seats[0].len();
        let mut validity = vec![0; m + 1];
        for i in 0..m {
            let mut rowvalid = 0;
            for j in 0..n {
                if seats[i][j] == '.' {
                    rowvalid += 1 << j;
                }
            }
            validity[i + 1] = rowvalid;
        }
        let mut dp = vec![vec![-1; (1 << n) + 1]; m + 1];
        dp[0][0] = 0;
        for i in 1..=m {
            let valid = validity[i];
            for j in 0..(1 << n) {
                if (j & valid) != j {
                    continue;
                }
                if (j & (j >> 1)) != 0 {
                    continue;
                }
                for k in 0..(1 << n) {
                    if dp[i - 1][k] == -1 {
                        continue;
                    }
                    if (j & (k >> 1)) != 0 || (k & (j >> 1)) != 0 {
                        continue;
                    }
                    dp[i][j] = dp[i][j].max(dp[i - 1][k] + j.count_ones() as i32);
                }
            }
        }
        *dp[m].iter().max().unwrap()
    }
}

#[test]
fn test() {
    let seats = vec![
        vec!['#', '.', '#', '#', '.', '#'],
        vec!['.', '#', '#', '#', '#', '.'],
        vec!['#', '.', '#', '#', '.', '#'],
    ];
    let res = 4;
    assert_eq!(Solution::max_students(seats), res);
    let seats = vec![
        vec!['.', '#'],
        vec!['#', '#'],
        vec!['#', '.'],
        vec!['#', '#'],
        vec!['.', '#'],
    ];
    let res = 3;
    assert_eq!(Solution::max_students(seats), res);
    let seats = vec![
        vec!['#', '.', '.', '.', '#'],
        vec!['.', '#', '.', '#', '.'],
        vec!['.', '.', '#', '.', '.'],
        vec!['.', '#', '.', '#', '.'],
        vec!['#', '.', '.', '.', '#'],
    ];
    let res = 10;
    assert_eq!(Solution::max_students(seats), res);
}
