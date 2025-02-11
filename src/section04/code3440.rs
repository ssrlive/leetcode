#![allow(dead_code)]

// 3440. Reschedule Meetings for Maximum Free Time II
// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-ii/
// https://leetcode.cn/problems/reschedule-meetings-for-maximum-free-time-ii/
//
// Medium
//
// You are given an integer eventTime denoting the duration of an event.
// You are also given two integer arrays startTime and endTime, each of length n.
//
// These represent the start and end times of n non-overlapping meetings that occur during the event between time t = 0
// and time t = eventTime, where the ith meeting occurs during the time [startTime[i], endTime[i]].
//
// You can reschedule at most one meeting by moving its start time while maintaining the same duration, such that the meetings
// remain non-overlapping, to maximize the longest continuous period of free time during the event.
//
// Return the maximum amount of free time possible after rearranging the meetings.
//
// Note that the meetings can not be rescheduled to a time outside the event and they should remain non-overlapping.
//
// Note: In this version, it is valid for the relative ordering of the meetings to change after rescheduling one meeting.
//
// Example 1:
//
// Input: eventTime = 5, startTime = [1,3], endTime = [2,5]
//
// Output: 2
//
// Explanation:
//
// Reschedule the meeting at [1, 2] to [2, 3], leaving no meetings during the time [0, 2].
//
// Example 2:
//
// Input: eventTime = 10, startTime = [0,7,9], endTime = [1,8,10]
//
// Output: 7
//
// Explanation:
//
// Reschedule the meeting at [0, 1] to [8, 9], leaving no meetings during the time [0, 7].
//
// Example 3:
//
// Input: eventTime = 10, startTime = [0,3,7,9], endTime = [1,4,8,10]
//
// Output: 6
//
// Explanation:
//
// Reschedule the meeting at [3, 4] to [8, 9], leaving no meetings during the time [1, 7].
//
// Example 4:
//
// Input: eventTime = 5, startTime = [0,1,2,3,4], endTime = [1,2,3,4,5]
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
//     0 <= startTime[i] < endTime[i] <= eventTime
//     endTime[i] <= startTime[i + 1] where i lies in the range [0, n - 2].
//

struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut gap = vec![start_time[0]];
        for i in 1..start_time.len() {
            gap.push(start_time[i] - end_time[i - 1]);
        }
        gap.push(event_time - end_time[end_time.len() - 1]);
        let mut largest_right = vec![0; gap.len()];
        for i in (0..gap.len() - 1).rev() {
            largest_right[i] = largest_right[i + 1].max(gap[i + 1]);
        }
        let mut ans = 0;
        let mut largest_left = 0;
        for i in 1..gap.len() {
            let cur_gap = end_time[i - 1] - start_time[i - 1];
            if cur_gap <= largest_left.max(largest_right[i]) {
                ans = ans.max(gap[i - 1] + gap[i] + cur_gap);
            }
            ans = ans.max(gap[i - 1] + gap[i]);
            largest_left = largest_left.max(gap[i - 1]);
        }
        ans
    }
}

#[test]
fn test() {
    let event_time = 5;
    let start_time = vec![1, 3];
    let end_time = vec![2, 5];
    let res = 2;
    assert_eq!(Solution::max_free_time(event_time, start_time, end_time), res);

    let event_time = 10;
    let start_time = vec![0, 7, 9];
    let end_time = vec![1, 8, 10];
    let res = 7;
    assert_eq!(Solution::max_free_time(event_time, start_time, end_time), res);

    let event_time = 10;
    let start_time = vec![0, 3, 7, 9];
    let end_time = vec![1, 4, 8, 10];
    let res = 6;
    assert_eq!(Solution::max_free_time(event_time, start_time, end_time), res);

    let event_time = 5;
    let start_time = vec![0, 1, 2, 3, 4];
    let end_time = vec![1, 2, 3, 4, 5];
    let res = 0;
    assert_eq!(Solution::max_free_time(event_time, start_time, end_time), res);
}
