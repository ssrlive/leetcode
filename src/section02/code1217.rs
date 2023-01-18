#![allow(dead_code)]

// 1217. Minimum Cost to Move Chips to The Same Position
// https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/
// https://leetcode.cn/problems/minimum-cost-to-move-chips-to-the-same-position/
//
// Easy
//
// We have n chips, where the position of the ith chip is position[i].
//
// We need to move all the chips to the same position. In one step, we can change the position of the ith chip from position[i] to:
//
//     position[i] + 2 or position[i] - 2 with cost = 0.
//     position[i] + 1 or position[i] - 1 with cost = 1.
//
// Return the minimum cost needed to move all the chips to the same position.
//
// Example 1:
//
// Input: position = [1,2,3]
// Output: 1
// Explanation: First step: Move the chip at position 3 to position 1 with cost = 0.
// Second step: Move the chip at position 2 to position 1 with cost = 1.
// Total cost is 1.
//
// Example 2:
//
// Input: position = [2,2,2,3,3]
// Output: 2
// Explanation: We can move the two chips at position  3 to position 2. Each move has cost = 1. The total cost = 2.
//
// Example 3:
//
// Input: position = [1,1000000000]
// Output: 1
//
// Constraints:
//
// -    1 <= position.length <= 100
// -    1 <= position[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut odd = 0;
        let mut even = 0;
        for i in position {
            if i % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        if odd > even {
            even
        } else {
            odd
        }
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 3], 1), (vec![2, 2, 2, 3, 3], 2), (vec![1, 1000000000], 1)];
    for (position, expected) in cases {
        assert_eq!(Solution::min_cost_to_move_chips(position), expected);
    }
}
