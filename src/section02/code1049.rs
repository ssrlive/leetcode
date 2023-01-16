#![allow(dead_code)]

// 1049. Last Stone Weight II
// https://leetcode.com/problems/last-stone-weight-ii/
// https://leetcode.cn/problems/last-stone-weight-ii/
//
// You are given an array of integers stones where stones[i] is the weight of the ith stone.
//
// We are playing a game with the stones. On each turn, we choose any two stones and smash them together.
// Suppose the stones have weights x and y with x <= y. The result of this smash is:
//
// If x == y, both stones are destroyed, and
// If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
// At the end of the game, there is at most one stone left.
//
// Return the smallest possible weight of the left stone. If there are no stones left, return 0.
//
// Example 1:
//
// Input: stones = [2,7,4,1,8,1]
// Output: 1
// Explanation:
// We can combine 2 and 4 to get 2, so the array converts to [2,7,1,8,1] then,
// we can combine 7 and 8 to get 1, so the array converts to [2,1,1,1] then,
// we can combine 2 and 1 to get 1, so the array converts to [1,1,1] then,
// we can combine 1 and 1 to get 0, so the array converts to [1], then that's the optimal value.
//
// Example 2:
//
// Input: stones = [31,26,33,21,40]
// Output: 5
//
// Constraints:
//
// - 1 <= stones.length <= 30
// - 1 <= stones[i] <= 100
//

struct Solution;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum::<i32>();
        let target = sum / 2;
        let mut dp = vec![false; (target + 1) as usize];
        dp[0] = true;
        for &stone in stones.iter() {
            for i in (stone..=target).rev() {
                dp[i as usize] = dp[i as usize] || dp[(i - stone) as usize];
            }
        }
        for i in (0..=target).rev() {
            if dp[i as usize] {
                return sum - 2 * i;
            }
        }
        unreachable!()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
}
