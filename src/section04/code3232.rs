#![allow(dead_code)]

// 3232. Find if Digit Game Can Be Won
// https://leetcode.com/problems/find-if-digit-game-can-be-won/
// https://leetcode.cn/problems/find-if-digit-game-can-be-won/
//
// Easy
//
// You are given an array of positive integers nums.
//
// Alice and Bob are playing a game. In the game, Alice can choose either all single-digit numbers or all double-digit
// numbers from nums, and the rest of the numbers are given to Bob.
// Alice wins if the sum of her numbers is strictly greater than the sum of Bob's numbers.
//
// Return true if Alice can win this game, otherwise, return false.
//
// Example 1:
//
// Input: nums = [1,2,3,4,10]
//
// Output: false
//
// Explanation:
//
// Alice cannot win by choosing either single-digit or double-digit numbers.
//
// Example 2:
//
// Input: nums = [1,2,3,4,5,14]
//
// Output: true
//
// Explanation:
//
// Alice can win by choosing single-digit numbers which have a sum equal to 15.
//
// Example 3:
//
// Input: nums = [5,5,5,25]
//
// Output: true
//
// Explanation:
//
// Alice can win by choosing double-digit numbers which have a sum equal to 25.
//
// Constraints:
//
//     1 <= nums.length <= 100
//     1 <= nums[i] <= 99
//

struct Solution;

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let (mut a, mut b) = (0, 0);
        for n in nums {
            if n < 10 {
                a += n;
            } else {
                b += n;
            }
        }
        a != b
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 10];
    let expected = false;
    assert_eq!(Solution::can_alice_win(nums), expected);

    let nums = vec![1, 2, 3, 4, 5, 14];
    let expected = true;
    assert_eq!(Solution::can_alice_win(nums), expected);

    let nums = vec![5, 5, 5, 25];
    let expected = true;
    assert_eq!(Solution::can_alice_win(nums), expected);
}
