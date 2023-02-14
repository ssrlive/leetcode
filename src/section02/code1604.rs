#![allow(dead_code)]

/*

// 1604. Alert Using Same Key-Card Three or More Times in a One Hour Period
// https://leetcode.com/problems/alert-using-same-key-card-three-or-more-times-in-a-one-hour-period/
// https://leetcode.cn/problems/alert-using-same-key-card-three-or-more-times-in-a-one-hour-period/
//
// Medium
//
// LeetCode company workers use key-cards to unlock office doors. Each time a worker uses their key-card, the security system saves the worker's name and the time when it was used. The system emits an alert if any worker uses the key-card three or more times in a one-hour period.

You are given a list of strings keyName and keyTime where [keyName[i], keyTime[i]] corresponds to a person's name and the time when their key-card was used in a single day.

Access times are given in the 24-hour time format "HH:MM", such as "23:51" and "09:49".

Return a list of unique worker names who received an alert for frequent keycard use. Sort the names in ascending order alphabetically.

Notice that "10:00" - "11:00" is considered to be within a one-hour period, while "22:51" - "23:52" is not considered to be within a one-hour period.



Example 1:

Input: keyName = ["daniel","daniel","daniel","luis","luis","luis","luis"], keyTime = ["10:00","10:40","11:00","09:00","11:00","13:00","15:00"]
Output: ["daniel"]
Explanation: "daniel" used the keycard 3 times in a one-hour period ("10:00","10:40", "11:00").

Example 2:

Input: keyName = ["alice","alice","alice","bob","bob","bob","bob"], keyTime = ["12:01","12:00","18:00","21:00","21:20","21:30","23:00"]
Output: ["bob"]
Explanation: "bob" used the keycard 3 times in a one-hour period ("21:00","21:20", "21:30").

Constraints:

    1 <= keyName.length, keyTime.length <= 105
    keyName.length == keyTime.length
    keyTime[i] is in the format "HH:MM".
    [keyName[i], keyTime[i]] is unique.
    1 <= keyName[i].length <= 10
    keyName[i] contains only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        for (name, time) in key_name.iter().zip(key_time.iter()) {
            let time = time.split(':').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
            let time = time[0] * 60 + time[1];
            map.entry(name).or_insert(vec![]).push(time);
        }
        let mut res = vec![];
        for (name, times) in map {
            let mut times = times;
            times.sort();
            for (i, j) in (2..times.len()).enumerate() {
                if times[j] - times[i] <= 60 {
                    res.push(name.to_string());
                    break;
                }
            }
        }
        res.sort();
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"],
            vec!["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"],
            vec!["daniel"],
        ),
        (
            vec!["alice", "alice", "alice", "bob", "bob", "bob", "bob"],
            vec!["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"],
            vec!["bob"],
        ),
    ];
    for (key_name, key_time, expected) in cases {
        let key_name = key_name.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let key_time = key_time.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(Solution::alert_names(key_name, key_time), expected);
    }
}
