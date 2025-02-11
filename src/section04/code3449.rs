#![allow(dead_code)]

// 3449. Maximize the Minimum Game Score
// https://leetcode.com/problems/maximize-the-minimum-game-score/
// https://leetcode.cn/problems/maximize-the-minimum-game-score/
//
// Hard
//
// You are given an array points of size n and an integer m. There is another array gameScore of size n,
// where gameScore[i] represents the score achieved at the ith game. Initially, gameScore[i] == 0 for all i.
//
// You start at index -1, which is outside the array (before the first position at index 0).
// You can make at most m moves. In each move, you can either:
//
//     Increase the index by 1 and add points[i] to gameScore[i].
//     Decrease the index by 1 and add points[i] to gameScore[i].
//
// Note that the index must always remain within the bounds of the array after the first move.
//
// Return the maximum possible minimum value in gameScore after at most m moves.
//
// Example 1:
//
// Input: points = [2,4], m = 3
//
// Output: 4
//
// Explanation:
//
// Initially, index i = -1 and gameScore = [0, 0].
// Move	Index	gameScore
// Increase i	0	[2, 0]
// Increase i	1	[2, 4]
// Decrease i	0	[4, 4]
//
// The minimum value in gameScore is 4, and this is the maximum possible minimum among all configurations. Hence, 4 is the output.
//
// Example 2:
//
// Input: points = [1,2,3], m = 5
//
// Output: 2
//
// Explanation:
//
// Initially, index i = -1 and gameScore = [0, 0, 0].
// Move	Index	gameScore
// Increase i	0	[1, 0, 0]
// Increase i	1	[1, 2, 0]
// Decrease i	0	[2, 2, 0]
// Increase i	1	[2, 4, 0]
// Increase i	2	[2, 4, 3]
//
// The minimum value in gameScore is 2, and this is the maximum possible minimum among all configurations. Hence, 2 is the output.
//
// Constraints:
//
//     2 <= n == points.length <= 5 * 10^4
//     1 <= points[i] <= 10^6
//     1 <= m <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_score(points: Vec<i32>, m: i32) -> i64 {
        let n = points.len();
        if m < n as i32 {
            return 0;
        }

        let mut left = 1;
        let mut right = 1e18 as i64;
        let mut ans = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::can_achieve(&points, n, m, mid) {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans
    }

    fn can_achieve(points: &[i32], n: usize, m: i32, val: i64) -> bool {
        let mut total = 0;
        let mut transfer = 0;
        let mut skip_add = 0;

        for &points_i in points.iter().take(n) {
            if total > m as i64 {
                break;
            }

            let point = points_i as i64;
            let necessary = (val + point - 1) / point;

            if transfer >= necessary {
                transfer = 0;
                skip_add += 1;
            } else {
                let p = transfer * point;
                let ops = ((val - p) + point - 1) / point;
                total += 2 * ops - 1;
                total += skip_add;

                transfer = std::cmp::max(ops - 1, 0);
                skip_add = 0;
            }
        }

        total <= m as i64
    }
}

#[test]
fn test() {
    let points = vec![2, 4];
    let m = 3;
    let res = 4;
    assert_eq!(Solution::max_score(points, m), res);

    let points = vec![1, 2, 3];
    let m = 5;
    let res = 2;
    assert_eq!(Solution::max_score(points, m), res);
}
