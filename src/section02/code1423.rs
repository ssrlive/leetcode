#![allow(dead_code)]

// 1423. Maximum Points You Can Obtain from Cards
// https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/
// https://leetcode.cn/problems/maximum-points-you-can-obtain-from-cards/
//
// Medium
//
// There are several cards arranged in a row, and each card has an associated number of points. The points are given in the integer array cardPoints.
//
// In one step, you can take one card from the beginning or from the end of the row. You have to take exactly k cards.
//
// Your score is the sum of the points of the cards you have taken.
//
// Given the integer array cardPoints and the integer k, return the maximum score you can obtain.
//
// Example 1:
//
// Input: cardPoints = [1,2,3,4,5,6,1], k = 3
// Output: 12
// Explanation: After the first step, your score will always be 1. However, choosing the rightmost card first will maximize your total score.
// The optimal strategy is to take the three cards on the right, giving a final score of 1 + 6 + 5 = 12.
//
// Example 2:
//
// Input: cardPoints = [2,2,2], k = 2
// Output: 4
// Explanation: Regardless of which two cards you take, your score will always be 4.
//
// Example 3:
//
// Input: cardPoints = [9,7,7,9,7,7,9], k = 7
// Output: 55
// Explanation: You have to take all the cards. Your score is the sum of points of all cards.
//
// Constraints:
//
// -    1 <= cardPoints.length <= 10^5
// -    1 <= cardPoints[i] <= 10^4
// -    1 <= k <= cardPoints.length
//

struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let f_init = |x: i32| (x, x);
        let init = f_init(card_points.iter().take(k as usize).sum::<i32>());
        card_points
            .iter()
            .take(k as usize)
            .rev()
            .zip(card_points.iter().rev())
            .fold(init, |(max, sum), (&p1, &p2)| {
                let f = |x: i32| (max.max(x), x);
                f(sum - p1 + p2)
            })
            .0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
    assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
}
