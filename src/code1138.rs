#![allow(dead_code)]

// 1138. Alphabet Board Path
// https://leetcode.com/problems/alphabet-board-path/
// https://leetcode.cn/problems/alphabet-board-path/
//
// On an alphabet board, we start at position (0, 0), corresponding to character board[0][0].
//
// Here, board = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"], as shown in the diagram below.
//
// We may make the following moves:
//
// 'U' moves our position up one row, if the position exists on the board;
// 'D' moves our position down one row, if the position exists on the board;
// 'L' moves our position left one column, if the position exists on the board;
// 'R' moves our position right one column, if the position exists on the board;
// '!' adds the character board[r][c] at our current position (r, c) to the answer.
// (Here, the only positions that exist on the board are positions with letters on them.)
//
// Return a sequence of moves that makes our answer equal to target in the minimum number of moves.  You may return any path that does so.
//
// Example 1:
//
// Input: target = "leet"
// Output: "DDR!UURRR!!DDD!"
//
// Example 2:
//
// Input: target = "code"
// Output: "RR!DDRR!UUL!R!"
//
// Constraints:
//
// - 1 <= target.length <= 100
// - target consists only of English lowercase letters.
//

struct Solution;

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut start: u8 = 97;
        let mut result: String = String::from("");

        for c in target.chars() {
            let dest: u8 = c as u8;

            while start != dest {
                let current_row: u8 = (start - 97) / 5;
                let target_row: u8 = (dest - 97) / 5;
                match current_row.cmp(&target_row) {
                    std::cmp::Ordering::Greater => {
                        result.push('U');
                        start -= 5;
                    }
                    std::cmp::Ordering::Less => {
                        if start + 5 > 122 {
                            result.push('L');
                            start -= 1;
                        } else {
                            result.push('D');
                            start += 5;
                        }
                    }
                    std::cmp::Ordering::Equal => {
                        if start > dest {
                            result.push('L');
                            start -= 1;
                        } else {
                            result.push('R');
                            start += 1;
                        }
                    }
                }
            }

            result.push('!');
        }

        result
    }
}

#[test]
fn test() {
    let cases = vec![
        ("leet".to_string(), "DDR!UURRR!!DDD!".to_string()),
        ("code".to_string(), "RR!DDRR!UUL!R!".to_string()),
    ];
    for (target, expected) in cases {
        assert_eq!(Solution::alphabet_board_path(target), expected);
    }
}
