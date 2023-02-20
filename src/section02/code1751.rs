#![allow(dead_code)]

/*

// 1751. Maximum Number of Events That Can Be Attended II
// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/
// https://leetcode.cN/problems/maximum-number-of-events-that-can-be-attended-ii/
//
// Hard
//
// You are given an array of events where events[i] = [startDayi, endDayi, valuei]. The ith event starts at startDayi and ends at endDayi, and if you attend this event, you will receive a value of valuei. You are also given an integer k which represents the maximum number of events you can attend.

You can only attend one event at a time. If you choose to attend an event, you must attend the entire event. Note that the end day is inclusive: that is, you cannot attend two events where one of them starts and the other ends on the same day.

Return the maximum sum of values that you can receive by attending events.

Example 1:

Input: events = [[1,2,4],[3,4,3],[2,3,1]], k = 2
Output: 7
Explanation: Choose the green events, 0 and 1 (0-indexed) for a total value of 4 + 3 = 7.

Example 2:

Input: events = [[1,2,4],[3,4,3],[2,3,10]], k = 2
Output: 10
Explanation: Choose event 2 for a total value of 10.
Notice that you cannot attend any other event as they overlap, and that you do not have to attend k events.

Example 3:

Input: events = [[1,1,1],[2,2,2],[3,3,3],[4,4,4]], k = 3
Output: 9
Explanation: Although the events do not overlap, you can only attend 3 events. Pick the highest valued three.

Constraints:

    1 <= k <= events.length
    1 <= k * events.length <= 10^6
    1 <= startDayi <= endDayi <= 10^9
    1 <= valuei <= 10^6
*/

struct Solution;

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let (n, k) = (events.len(), k as usize);
        let mut dp = vec![vec![-1; k]; n];
        let mut events = events;
        events.sort_by(|a, b| a[1].cmp(&b[1]));
        Self::eval(&mut dp, &events, n - 1, k - 1)
    }

    fn eval(dp: &mut Vec<Vec<i32>>, events: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if dp[i][j] != -1 {
            return dp[i][j];
        }
        dp[i][j] = events[i][2];
        if i == 0 {
            return dp[i][j];
        }
        dp[i][j] = dp[i][j].max(Self::eval(dp, events, i - 1, j));
        if j == 0 {
            return dp[i][j];
        }
        let it = Self::binary_search(events, i);
        if it == -1 {
            return dp[i][j];
        }
        dp[i][j] = dp[i][j].max(events[i][2] + Self::eval(dp, events, it as usize, j - 1));
        dp[i][j]
    }

    fn binary_search(events: &[Vec<i32>], i: usize) -> i32 {
        if events[0][1] >= events[i][0] {
            return -1;
        }
        let (mut left, mut right) = (0, i - 1);
        while left < right {
            let mid = right - (right - left) / 2;
            if events[mid][1] < events[i][0] {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2, 7),
        (vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]], 2, 10),
        (vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]], 3, 9),
    ];
    for (events, k, expected) in cases {
        assert_eq!(Solution::max_value(events, k), expected);
    }
}
