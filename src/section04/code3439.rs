#![allow(dead_code)]

// 3439. Reschedule Meetings for Maximum Free Time I
// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-i/
// https://leetcode.cn/problems/reschedule-meetings-for-maximum-free-time-i/
//
// Medium
//
// You are given an integer eventTime denoting the duration of an event, where the event occurs from time t = 0 to time t = eventTime.
//
// You are also given two integer arrays startTime and endTime, each of length n. These represent the start and end
// time of n non-overlapping meetings, where the ith meeting occurs during the time [startTime[i], endTime[i]].
//
// You can reschedule at most k meetings by moving their start time while maintaining the same duration,
// to maximize the longest continuous period of free time during the event.
//
// The relative order of all the meetings should stay the same and they should remain non-overlapping.
//
// Return the maximum amount of free time possible after rearranging the meetings.
//
// Note that the meetings can not be rescheduled to a time outside the event.
//
// Example 1:
//
// Input: eventTime = 5, k = 1, startTime = [1,3], endTime = [2,5]
//
// Output: 2
//
// Explanation:
//
// Reschedule the meeting at [1, 2] to [2, 3], leaving no meetings during the time [0, 2].
//
// Example 2:
//
// Input: eventTime = 10, k = 1, startTime = [0,2,9], endTime = [1,4,10]
//
// Output: 6
//
// Explanation:
//
// Reschedule the meeting at [2, 4] to [1, 3], leaving no meetings during the time [3, 9].
//
// Example 3:
//
// Input: eventTime = 5, k = 2, startTime = [0,1,2,3,4], endTime = [1,2,3,4,5]
//
// Output: 0
//
// Explanation:
//
// There is no time during the event not occupied by meetings.
//
// Constraints:
//
//     1 <= eventTime <= 10^9
//     n == startTime.length == endTime.length
//     2 <= n <= 10^5
//     1 <= k <= n
//     0 <= startTime[i] < endTime[i] <= eventTime
//     endTime[i] <= startTime[i + 1] where i lies in the range [0, n - 2].
//

struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut gaps = vec![0; n + 1];

        // Calculate the first and last gap separately
        gaps[0] = start_time[0]; // Free time before the first meeting
        gaps[n] = event_time - end_time[n - 1]; // Free time after the last meeting

        // Compute gaps between meetings
        for i in 1..n {
            gaps[i] = start_time[i] - end_time[i - 1];
        }

        // Compute prefix sum for efficient range sum calculation
        let mut pref = vec![0; n + 2];
        for i in 1..=n + 1 {
            pref[i] = pref[i - 1] + gaps[i - 1];
        }

        let mut ans = 0;
        for i in (k as usize + 1)..=n + 1 {
            ans = ans.max(pref[i] - pref[i - (k + 1) as usize]);
        }

        ans
    }
}

#[test]
fn test() {
    let event_time = 5;
    let k = 1;
    let start_time = vec![1, 3];
    let end_time = vec![2, 5];
    let res = 2;
    assert_eq!(Solution::max_free_time(event_time, k, start_time, end_time), res);

    let event_time = 10;
    let k = 1;
    let start_time = vec![0, 2, 9];
    let end_time = vec![1, 4, 10];
    let res = 6;
    assert_eq!(Solution::max_free_time(event_time, k, start_time, end_time), res);

    let event_time = 5;
    let k = 2;
    let start_time = vec![0, 1, 2, 3, 4];
    let end_time = vec![1, 2, 3, 4, 5];
    let res = 0;
    assert_eq!(Solution::max_free_time(event_time, k, start_time, end_time), res);
}
