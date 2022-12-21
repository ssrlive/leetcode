#![allow(dead_code)]

// 837. New 21 Game
// https://leetcode.com/problems/new-21-game/
// https://leetcode.cn/problems/new-21-game/
//
// Alice plays the following game, loosely based on the card game "21".
//
// Alice starts with 0 points and draws numbers while she has less than k points.
// During each draw, she gains an integer number of points randomly from the range [1, maxPts],
// where maxPts is an integer. Each draw is independent and the outcomes have equal probabilities.
//
// Alice stops drawing numbers when she gets k or more points.
//
// Return the probability that Alice has n or fewer points.
//
// Answers within 10-5 of the actual answer are considered accepted.
//
// Example 1:
//
// Input: n = 10, k = 1, maxPts = 10
// Output: 1.00000
// Explanation: Alice gets a single card, then stops.
//
// Example 2:
//
// Input: n = 6, k = 1, maxPts = 10
// Output: 0.60000
// Explanation: Alice gets a single card, then stops.
// In 6 out of 10 possibilities, she is at or below 6 points.
//
// Example 3:
//
// Input: n = 21, k = 17, maxPts = 10
// Output: 0.73278
//
// Constraints:
//
// - 0 <= k <= n <= 10^4
// - 1 <= maxPts <= 10^4
//

struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;
        if k == 0 || n >= k + max_pts {
            return 1.0;
        }
        let mut prob = vec![0.0; k + max_pts];
        for item in prob.iter_mut().take(n + 1).skip(k) {
            *item = 1.0;
        }
        let mut cur_sum = prob[k..].iter().sum::<f64>();
        for pts in (0..k).rev() {
            prob[pts] = cur_sum / max_pts as f64;
            cur_sum += prob[pts] - prob[pts + max_pts];
        }
        prob[0]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::new21_game(1, 0, 1), 1.0);
    assert_eq!(Solution::new21_game(10, 1, 10), 1.0);
    assert_eq!(Solution::new21_game(6, 1, 10), 0.6);
    assert_eq!(Solution::new21_game(21, 17, 10), 0.7327777870686075);
}
