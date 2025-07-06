#![allow(dead_code)]

/*

// 1900. The Earliest and Latest Rounds Where Players Compete
// https://leetcode.com/problems/the-earliest-and-latest-rounds-where-players-compete/
// https://leetcode.cn/problems/the-earliest-and-latest-rounds-where-players-compete/
//
// Hard
//
// There is a tournament where n players are participating. The players are standing in a single row and are numbered from 1 to n based on their initial standing position (player 1 is the first player in the row, player 2 is the second player in the row, etc.).

The tournament consists of multiple rounds (starting from round number 1). In each round, the ith player from the front of the row competes against the ith player from the end of the row, and the winner advances to the next round. When the number of players is odd for the current round, the player in the middle automatically advances to the next round.

    For example, if the row consists of players 1, 2, 4, 6, 7
        Player 1 competes against player 7.
        Player 2 competes against player 6.
        Player 4 automatically advances to the next round.

After each round is over, the winners are lined back up in the row based on the original ordering assigned to them initially (ascending order).

The players numbered firstPlayer and secondPlayer are the best in the tournament. They can win against any other player before they compete against each other. If any two other players compete against each other, either of them might win, and thus you may choose the outcome of this round.

Given the integers n, firstPlayer, and secondPlayer, return an integer array containing two values, the earliest possible round number and the latest possible round number in which these two players will compete against each other, respectively.

Example 1:

Input: n = 11, firstPlayer = 2, secondPlayer = 4
Output: [3,4]
Explanation:
One possible scenario which leads to the earliest round number:
First round: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
Second round: 2, 3, 4, 5, 6, 11
Third round: 2, 3, 4
One possible scenario which leads to the latest round number:
First round: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
Second round: 1, 2, 3, 4, 5, 6
Third round: 1, 2, 4
Fourth round: 2, 4

Example 2:

Input: n = 5, firstPlayer = 1, secondPlayer = 5
Output: [1,1]
Explanation: The players numbered 1 and 5 compete in the first round.
There is no way to make them compete in any other round.

Constraints:

    2 <= n <= 28
    1 <= firstPlayer < secondPlayer <= n
*/

struct Solution;

impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        let mut mn = 10000;
        let mut mx = 0;

        let mut arr = vec![0; n as usize];
        for i in 1..=n {
            arr[i as usize - 1] = i;
        }
        Solution::dfs(&arr, 1, &mut mn, &mut mx, first_player, second_player);
        vec![mn, mx]
    }

    fn dfs(arr: &Vec<i32>, round: i32, mn: &mut i32, mx: &mut i32, first: i32, second: i32) {
        let size = arr.len() / 2;
        if arr.len() == 1 {
            return;
        }
        for i in 0..size {
            if arr[i] == first && arr[arr.len() - i - 1] == second {
                *mn = (*mn).min(round);
                *mx = (*mx).max(round);
                return;
            }
        }
        let mut f1 = false;
        let mut f2 = false;
        for n in arr {
            f1 |= *n == first;
            f2 |= *n == second;
        }
        if !f1 || !f2 {
            return;
        }
        let mut nextarr = vec![0; size + (arr.len() % 2)];
        let m = (1 << size) - 1;
        for i in 0..=m {
            let mut left = 0_i32;
            let mut right = nextarr.len() as i32 - 1;
            for j in 0..size {
                if (1 << j) & i != 0 {
                    nextarr[left as usize] = arr[j];
                    left += 1;
                } else {
                    nextarr[right as usize] = arr[arr.len() - j - 1];
                    right -= 1;
                }
            }
            if !arr.len().is_multiple_of(2) {
                nextarr[left as usize] = arr[arr.len() / 2];
            }
            Solution::dfs(&nextarr, round + 1, mn, mx, first, second);
        }
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (11, 2, 4, vec![3, 4]),
        (5, 1, 5, vec![1, 1]),
        (6, 1, 6, vec![1, 1]),
        (6, 1, 5, vec![2, 2]),
    ];
    for (n, first_player, second_player, expected) in cases {
        assert_eq!(Solution::earliest_and_latest(n, first_player, second_player), expected);
    }
}
