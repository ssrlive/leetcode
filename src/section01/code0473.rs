#![allow(dead_code)]

// 473. Matchsticks to Square
// https://leetcode.com/problems/matchsticks-to-square/
// https://leetcode.cn/problems/matchsticks-to-square/
//
// You are given an integer array matchsticks where matchsticks[i] is the length of the ith matchstick.
// You want to use all the matchsticks to make one square. You should not break any stick,
// but you can link them up, and each matchstick must be used exactly one time.
//
// Return true if you can make this square and false otherwise.
//
// Example 1:
//
// Input: matchsticks = [1,1,2,2,2]
// Output: true
// Explanation: You can form a square with length 2, one side of the square came two sticks with length 1.
//
// Example 2:
//
// Input: matchsticks = [3,3,3,3,4]
// Output: false
// Explanation: You cannot find a way to form a square with all the matchsticks.
//
// Constraints:
//
// - 1 <= matchsticks.length <= 15
// - 1 <= matchsticks[i] <= 10^8
//

struct Solution;

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let mut matchsticks = matchsticks;
        let perim: i32 = matchsticks.iter().sum();
        perim % 4 == 0 && Self::solve(&mut matchsticks[..], 0, 0, 1, perim / 4)
    }

    fn solve(sticks: &mut [i32], l: usize, len: i32, side: u8, max: i32) -> bool {
        side == 4
            || len == max && Self::solve(sticks, 0, 0, side + 1, max)
            || (l..sticks.len()).any(|i| {
                let stick = std::mem::take(&mut sticks[i]);
                let res = stick != 0 && Self::solve(sticks, i + 1, len + stick, side, max);
                sticks[i] = stick;
                res
            })
    }
}

#[test]
fn test_makesquare() {
    let matchsticks = vec![1, 1, 2, 2, 2];
    assert!(Solution::makesquare(matchsticks));

    let matchsticks = vec![3, 3, 3, 3, 4];
    assert!(!Solution::makesquare(matchsticks));

    let matchsticks = vec![8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 60];
    assert!(!Solution::makesquare(matchsticks));

    let matchsticks = vec![13, 11, 1, 8, 6, 7, 8, 8, 6, 7, 8, 9, 8];
    assert!(Solution::makesquare(matchsticks));
}
