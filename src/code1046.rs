#![allow(dead_code)]

// 1046. Last Stone Weight
// https://leetcode.com/problems/last-stone-weight/
// https://leetcode.cn/problems/last-stone-weight/
//
// You are given an array of integers stones where stones[i] is the weight of the ith stone.
//
// We are playing a game with the stones. On each turn, we choose the heaviest two stones and smash them together.
// Suppose the heaviest two stones have weights x and y with x <= y. The result of this smash is:
//
// If x == y, both stones are destroyed, and
// If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
// At the end of the game, there is at most one stone left.
//
// Return the weight of the last remaining stone. If there are no stones left, return 0.
//
// Example 1:
//
// Input: stones = [2,7,4,1,8,1]
// Output: 1
// Explanation:
// We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
// we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
// we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
// we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of the last stone.
//
// Example 2:
//
// Input: stones = [1]
// Output: 1
//
// Constraints:
//
// - 1 <= stones.length <= 30
// - 1 <= stones[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        stones.sort_unstable();
        while stones.len() > 1 {
            let x = stones.pop().unwrap();
            let y = stones.pop().unwrap();
            if x != y {
                let z = x - y;
                let mut i = stones.len();
                while i > 0 && stones[i - 1] > z {
                    i -= 1;
                }
                stones.insert(i, z);
            }
        }
        stones.pop().unwrap_or(0)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    assert_eq!(Solution::last_stone_weight(vec![1, 1]), 0);
    assert_eq!(Solution::last_stone_weight(vec![1, 1, 1]), 1);
}
