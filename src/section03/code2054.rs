#![allow(dead_code)]

/*

// 2054. Two Best Non-Overlapping Events
// https://leetcode.com/problems/two-best-non-overlapping-events/
// https://leetcode.cn/problems/two-best-non-overlapping-events/
//
// Medium
//
// You are given a 0-indexed 2D integer array of events where events[i] = [startTimei, endTimei, valuei]. The ith event starts at startTimei and ends at endTimei, and if you attend this event, you will receive a value of valuei. You can choose at most two non-overlapping events to attend such that the sum of their values is maximized.

Return this maximum sum.

Note that the start time and end time is inclusive: that is, you cannot attend two events where one of them starts and the other ends at the same time. More specifically, if you attend an event with end time t, the next event must start at or after t + 1.

Example 1:

Input: events = [[1,3,2],[4,5,2],[2,4,3]]
Output: 4
Explanation: Choose the green events, 0 and 1 for a sum of 2 + 2 = 4.

Example 2:
Example 1 Diagram

Input: events = [[1,3,2],[4,5,2],[1,5,5]]
Output: 5
Explanation: Choose event 2 for a sum of 5.

Example 3:

Input: events = [[1,5,3],[1,5,1],[6,6,5]]
Output: 8
Explanation: Choose events 0 and 2 for a sum of 3 + 5 = 8.

Constraints:

    2 <= events.length <= 10^5
    events[i].length == 3
    1 <= startTimei <= endTimei <= 10^9
    1 <= valuei <= 10^6
*/

struct Solution;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        let l = events.len();
        for i in 0..l {
            events[i].push(0);

            let mut e = events[i].clone();
            e[3] = 1;
            events.push(e);
        }
        events.sort_by_key(|e| e[e[3] as usize]);
        let mut max_until = 0;
        let mut max_ret = 0;
        for e in events {
            if e[3] == 1 {
                max_until = max_until.max(e[2]);
            } else {
                max_ret = max_ret.max(max_until + e[2]);
            }
        }
        max_ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]], 4),
        (vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]], 5),
        (vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]], 8),
    ];
    for (events, expected) in cases {
        assert_eq!(Solution::max_two_events(events), expected);
    }
}
