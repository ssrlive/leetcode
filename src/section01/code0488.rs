#![allow(dead_code)]

// 488. Zuma Game
// https://leetcode.com/problems/zuma-game/
// https://leetcode.cn/problems/zuma-game/
//
// You are playing a variation of the game Zuma.
//
// In this variation of Zuma, there is a single row of colored balls on a board, where each ball can be colored red 'R',
// yellow 'Y', blue 'B', green 'G', or white 'W'. You also have several colored balls in your hand.
//
// Your goal is to clear all of the balls from the board. On each turn:
//
// Pick any ball from your hand and insert it in between two balls in the row or on either end of the row.
// If there is a group of three or more consecutive balls of the same color, remove the group of balls from the board.
// If this removal causes more groups of three or more of the same color to form, then continue removing each group until there are none left.
// If there are no more balls on the board, then you win the game.
// Repeat this process until you either win or do not have any more balls in your hand.
// Given a string board, representing the row of balls on the board, and a string hand, representing the balls in your hand,
// return the minimum number of balls you have to insert to clear all the balls from the board.
// If you cannot clear all the balls from the board using the balls in your hand, return -1.
//
// Example 1:
//
// Input: board = "WRRBBW", hand = "RB"
// Output: -1
// Explanation: It is impossible to clear all the balls. The best you can do is:
// - Insert 'R' so the board becomes WRRRBBW. WRRRBBW -> WBBW.
// - Insert 'B' so the board becomes WBBBW. WBBBW -> WW.
// There are still balls remaining on the board, and you are out of balls to insert.
//
// Example 2:
//
// Input: board = "WWRRBBWW", hand = "WRBRW"
// Output: 2
// Explanation: To make the board empty:
// - Insert 'R' so the board becomes WWRRRBBWW. WWRRRBBWW -> WWBBWW.
// - Insert 'B' so the board becomes WWBBBWW. WWBBBWW -> WWWW -> empty.
// 2 balls from your hand were needed to clear the board.
//
// Example 3:
//
// Input: board = "G", hand = "GGGGG"
// Output: 2
// Explanation: To make the board empty:
// - Insert 'G' so the board becomes GG.
// - Insert 'G' so the board becomes GGG. GGG -> empty.
// 2 balls from your hand were needed to clear the board.
//
// Constraints:
//
// - 1 <= board.length <= 16
// - 1 <= hand.length <= 5
// - board and hand consist of the characters 'R', 'Y', 'B', 'G', and 'W'.
// - The initial row of balls on the board will not have any groups of three or more consecutive balls of the same color.
//

struct Solution;

impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        Self::_find_min_step(board, hand).unwrap_or(-1)
    }

    pub fn _find_min_step(board: String, hand: String) -> Option<i32> {
        let mut hand = hand.chars().collect::<Vec<char>>();
        hand.sort();
        let mut q = std::collections::VecDeque::new();
        q.push_back((board.clone(), hand.clone(), 0));
        let mut visited = std::collections::HashSet::new();
        visited.insert((board, hand));
        while let Some((curr_board, curr_hand, step)) = q.pop_front() {
            for i in 0..=curr_board.len() {
                for j in 0..curr_hand.len() {
                    if j > 0 && curr_hand[j] == curr_hand[j - 1] {
                        continue;
                    }
                    if i > 0 && curr_board.chars().nth(i - 1)? == curr_hand[j] {
                        continue;
                    }
                    let mut pick = false;
                    if i < curr_board.len() && curr_board.chars().nth(i)? == curr_hand[j] {
                        pick = true;
                    }
                    if i > 0
                        && i < curr_board.len()
                        && curr_board.chars().nth(i - 1)? == curr_board.chars().nth(i)?
                        && curr_board.chars().nth(i)? != curr_hand[j]
                    {
                        pick = true;
                    }
                    if pick {
                        let mut new_board = curr_board.clone();
                        new_board.insert(i, curr_hand[j]);
                        new_board = Self::remove_same(&new_board, i)?;
                        let mut new_hand = curr_hand.clone();
                        new_hand.remove(j);
                        if new_board.is_empty() {
                            return Some(step + 1);
                        }
                        if !visited.contains(&(new_board.clone(), new_hand.clone())) {
                            q.push_back((new_board.clone(), new_hand.clone(), step + 1));
                            visited.insert((new_board, new_hand));
                        }
                    }
                }
            }
        }
        Some(-1)
    }

    fn remove_same(s: &str, i: usize) -> Option<String> {
        if i >= s.len() {
            return Some(s.to_string());
        }
        let mut left = i;
        let mut right = i;
        while left > 0 && s.chars().nth(left - 1)? == s.chars().nth(i)? {
            left -= 1;
        }
        while right + 1 < s.len() && s.chars().nth(right + 1)? == s.chars().nth(i)? {
            right += 1;
        }
        let length = right - left + 1;
        if length >= 3 {
            let mut new_s = s.to_string();
            new_s.replace_range(left..=right, "");
            Self::remove_same(&new_s, left)
        } else {
            Some(s.to_string())
        }
    }
}

#[test]
fn test_find_min_step() {
    assert_eq!(Solution::find_min_step("YYRGWRBYGGBGBGWY".to_string(), "BWGRY".to_string()), -1);
    assert_eq!(Solution::find_min_step("RRWWRRBBRR".to_string(), "WB".to_string()), 2);
    assert_eq!(Solution::find_min_step("WWBBWBBWW".to_string(), "BB".to_string()), -1);
    assert_eq!(Solution::find_min_step("RRWWRRW".to_string(), "RR".to_string()), 2);
    assert_eq!(Solution::find_min_step("RRWWRRW".to_string(), "W".to_string()), -1);
    assert_eq!(Solution::find_min_step("WRRBBW".to_string(), "RB".to_string()), -1);
    assert_eq!(Solution::find_min_step("WWRRBBWW".to_string(), "WRBRW".to_string()), 2);
    assert_eq!(Solution::find_min_step("G".to_string(), "GGGGG".to_string()), 2);
}
