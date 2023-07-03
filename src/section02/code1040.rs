#![allow(dead_code)]

// 1040. Moving Stones Until Consecutive II
// https://leetcode.com/problems/moving-stones-until-consecutive-ii/
// https://leetcode.cn/problems/moving-stones-until-consecutive-ii/
//
// There are some stones in different positions on the X-axis. You are given an integer array stones, the positions of the stones.
//
// Call a stone an endpoint stone if it has the smallest or largest position. In one move, you pick up an endpoint
// stone and move it to an unoccupied position so that it is no longer an endpoint stone.
//
// In particular, if the stones are at say, stones = [1,2,5], you cannot move the endpoint stone at position 5,
// since moving it to any position (such as 0, or 3) will still keep that stone as an endpoint stone.
// The game ends when you cannot make any more moves (i.e., the stones are in three consecutive positions).
//
// Return an integer array answer of length 2 where:
//
// answer[0] is the minimum number of moves you can play, and
// answer[1] is the maximum number of moves you can play.
//
// Example 1:
//
// Input: stones = [7,4,9]
// Output: [1,2]
// Explanation: We can move 4 -> 8 for one move to finish the game.
// Or, we can move 9 -> 5, 4 -> 6 for two moves to finish the game.
//
// Example 2:
//
// Input: stones = [6,5,4,3,10]
// Output: [2,3]
// Explanation: We can move 3 -> 8 then 10 -> 7 to finish the game.
// Or, we can move 3 -> 7, 4 -> 8, 5 -> 9 to finish the game.
// Notice we cannot move 10 -> 2 to finish the game, because that would be an illegal move.
//
// Constraints:
//
// - 3 <= stones.length <= 10^4
// - 1 <= stones[i] <= 10^9
// - All the values of stones are unique.
//

struct Solution;

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let mut stones = stones;
        stones.sort();
        let n = stones.len();
        let mut i = 0;
        let mut low = n as i32;
        let high = std::cmp::max(stones[n - 1] - n as i32 + 2 - stones[1], stones[n - 2] - stones[0] - n as i32 + 2);
        for j in 0..n {
            while stones[j] - stones[i] >= n as i32 {
                i += 1;
            }
            if j - i + 1 == n - 1 && stones[j] - stones[i] == n as i32 - 2 {
                low = std::cmp::min(low, 2);
            } else {
                low = std::cmp::min(low, n as i32 - (j - i + 1) as i32);
            }
        }
        vec![low, high]
    }
}

#[test]
fn test() {
    let cases = vec![(vec![7, 4, 9], vec![1, 2]), (vec![6, 5, 4, 3, 10], vec![2, 3])];
    for (stones, expected) in cases {
        assert_eq!(Solution::num_moves_stones_ii(stones), expected);
    }
}
