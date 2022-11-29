#![allow(dead_code)]

// 464. Can I Win
// https://leetcode.com/problems/can-i-win/
//
// In the "100 game" two players take turns adding, to a running total, any integer from 1 to 10.
// The player who first causes the running total to reach or exceed 100 wins.
//
// What if we change the game so that players cannot re-use integers?
//
// For example, two players might take turns drawing from a common pool of numbers from 1 to 15
// without replacement until they reach a total >= 100.
//
// Given two integers maxChoosableInteger and desiredTotal, return true if the first player
// to move can force a win, otherwise, return false. Assume both players play optimally.
//
// Example 1:
//
// Input: maxChoosableInteger = 10, desiredTotal = 11
// Output: false
// Explanation:
// No matter which integer the first player choose, the first player will lose.
// The first player can choose an integer from 1 up to 10.
// If the first player choose 1, the second player can only choose integers from 2 up to 10.
// The second player will win by choosing 10 and get a total = 11, which is >= desiredTotal.
// Same with other integers chosen by the first player, the second player will always win.
//
// Example 2:
//
// Input: maxChoosableInteger = 10, desiredTotal = 0
// Output: true
//
// Example 3:
//
// Input: maxChoosableInteger = 10, desiredTotal = 1
// Output: true
//
// Constraints:
//
// - 1 <= maxChoosableInteger <= 20
// - 0 <= desiredTotal <= 300
//

struct Solution;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if desired_total <= 0 {
            return true;
        }

        if (1 + max_choosable_integer) * max_choosable_integer / 2 < desired_total {
            return false;
        }

        let mut memo = vec![vec![None; 1 << max_choosable_integer as usize]; desired_total as usize + 1];

        Solution::can_i_win_helper(max_choosable_integer, desired_total, 0, &mut memo)
    }

    fn can_i_win_helper(
        max_choosable_integer: i32,
        desired_total: i32,
        used: i32,
        memo: &mut Vec<Vec<Option<bool>>>,
    ) -> bool {
        if desired_total <= 0 {
            return false;
        }

        if let Some(result) = memo[desired_total as usize][used as usize] {
            return result;
        }

        for i in 0..max_choosable_integer {
            let mask = 1 << i;
            if used & mask == 0
                && !Solution::can_i_win_helper(max_choosable_integer, desired_total - i - 1, used | mask, memo)
            {
                memo[desired_total as usize][used as usize] = Some(true);
                return true;
            }
        }

        memo[desired_total as usize][used as usize] = Some(false);
        false
    }
}

#[test]
fn test_can_i_win() {
    assert_eq!(Solution::can_i_win(10, 11), false);
    assert_eq!(Solution::can_i_win(10, 0), true);
    assert_eq!(Solution::can_i_win(10, 1), true);
}
