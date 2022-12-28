#![allow(dead_code)]

// 2446. Determine if Two Events Have Conflict
// https://leetcode.com/problems/determine-if-two-events-have-conflict/
// https://leetcode.cn/problems/determine-if-two-events-have-conflict/
//
// You are given two arrays of strings that represent two inclusive events that happened on the same day, event1 and event2, where:
//
// event1 = [startTime1, endTime1] and
// event2 = [startTime2, endTime2].
// Event times are valid 24 hours format in the form of HH:MM.
//
// A conflict happens when two events have some non-empty intersection (i.e., some moment is common to both events).
//
// Return true if there is a conflict between two events. Otherwise, return false.
//
// Example 1:
//
// Input: event1 = ["01:15","02:00"], event2 = ["02:00","03:00"]
// Output: true
// Explanation: The two events intersect at time 2:00.
//
// Example 2:
//
// Input: event1 = ["01:00","02:00"], event2 = ["01:20","03:00"]
// Output: true
// Explanation: The two events intersect starting from 01:20 to 02:00.
//
// Example 3:
//
// Input: event1 = ["10:00","11:00"], event2 = ["14:00","15:00"]
// Output: false
// Explanation: The two events do not intersect.
//
// Constraints:
//
// - evnet1.length == event2.length == 2.
// - event1[i].length == event2[i].length == 5
// - startTime1 <= endTime1
// - startTime2 <= endTime2
// - All the event times follow the HH:MM format.
//

struct Solution;

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        event1[0] <= event2[1] && event1[1] >= event2[0]
    }
}

#[test]
fn test() {
    let event1 = vec!["01:15".to_string(), "02:00".to_string()];
    let event2 = vec!["02:00".to_string(), "03:00".to_string()];
    assert_eq!(Solution::have_conflict(event1, event2), true);

    let event1 = vec!["01:00".to_string(), "02:00".to_string()];
    let event2 = vec!["01:20".to_string(), "03:00".to_string()];
    assert_eq!(Solution::have_conflict(event1, event2), true);

    let event1 = vec!["10:00".to_string(), "11:00".to_string()];
    let event2 = vec!["14:00".to_string(), "15:00".to_string()];
    assert_eq!(Solution::have_conflict(event1, event2), false);
}
