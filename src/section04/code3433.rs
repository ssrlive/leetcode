#![allow(dead_code)]

// 3433. Count Mentions Per User
// https://leetcode.com/problems/count-mentions-per-user/
// https://leetcode.cn/problems/count-mentions-per-user/
//
// Medium
//
// You are given an integer numberOfUsers representing the total number of users and an array events of size n x 3.
//
// Each events[i] can be either of the following two types:
//
//     Message Event: ["MESSAGE", "timestampi", "mentions_stringi"]
//         This event indicates that a set of users was mentioned in a message at timestampi.
//         The mentions_stringi string can contain one of the following tokens:
//             id<number>: where <number> is an integer in range [0,numberOfUsers - 1]. There can be multiple ids separated by a single whitespace and may contain duplicates. This can mention even the offline users.
//             ALL: mentions all users.
//             HERE: mentions all online users.
//     Offline Event: ["OFFLINE", "timestampi", "idi"]
//         This event indicates that the user idi had become offline at timestampi for 60 time units. The user will automatically be online again at time timestampi + 60.
//
// Return an array mentions where mentions[i] represents the number of mentions the user with id i has across all MESSAGE events.
//
// All users are initially online, and if a user goes offline or comes back online, their status change is processed before handling any message event that occurs at the same timestamp.
//
// Note that a user can be mentioned multiple times in a single message event, and each mention should be counted separately.
//
// Example 1:
//
// Input: numberOfUsers = 2, events = [["MESSAGE","10","id1 id0"],["OFFLINE","11","0"],["MESSAGE","71","HERE"]]
//
// Output: [2,2]
//
// Explanation:
//
// Initially, all users are online.
//
// At timestamp 10, id1 and id0 are mentioned. mentions = [1,1]
//
// At timestamp 11, id0 goes offline.
//
// At timestamp 71, id0 comes back online and "HERE" is mentioned. mentions = [2,2]
//
// Example 2:
//
// Input: numberOfUsers = 2, events = [["MESSAGE","10","id1 id0"],["OFFLINE","11","0"],["MESSAGE","12","ALL"]]
//
// Output: [2,2]
//
// Explanation:
//
// Initially, all users are online.
//
// At timestamp 10, id1 and id0 are mentioned. mentions = [1,1]
//
// At timestamp 11, id0 goes offline.
//
// At timestamp 12, "ALL" is mentioned. This includes offline users, so both id0 and id1 are mentioned. mentions = [2,2]
//
// Example 3:
//
// Input: numberOfUsers = 2, events = [["OFFLINE","10","0"],["MESSAGE","12","HERE"]]
//
// Output: [0,1]
//
// Explanation:
//
// Initially, all users are online.
//
// At timestamp 10, id0 goes offline.
//
// At timestamp 12, "HERE" is mentioned. Because id0 is still offline, they will not be mentioned. mentions = [0,1]
//
// Constraints:
//
//     1 <= numberOfUsers <= 100
//     1 <= events.length <= 100
//     events[i].length == 3
//     events[i][0] will be one of MESSAGE or OFFLINE.
//     1 <= int(events[i][1]) <= 105
//     The number of id<number> mentions in any "MESSAGE" event is between 1 and 100.
//     0 <= <number> <= numberOfUsers - 1
//     It is guaranteed that the user id referenced in the OFFLINE event is online at the time the event occurs.
//

struct Solution;

impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        use std::cmp::Reverse;

        fn extract_integers(s: &str) -> Vec<i32> {
            s.split_whitespace().map(|id| id[2..].parse().unwrap()).collect()
        }

        let mut ans = vec![0; number_of_users as usize];
        let mut user_status = vec![1; number_of_users as usize];
        let mut pq: std::collections::BinaryHeap<Reverse<(i32, i32)>> = std::collections::BinaryHeap::new();
        let mut all = 0;

        let mut events = events;
        events.sort_by(|a, b| {
            let a1 = a[1].parse::<i32>().unwrap();
            let b1 = b[1].parse::<i32>().unwrap();
            match a1.cmp(&b1) {
                std::cmp::Ordering::Equal => b[0].cmp(&a[0]),
                other => other,
            }
        });

        for events_i in &events {
            let event_i_i = events_i[1].parse::<i32>().unwrap();
            if events_i[0] == "MESSAGE" {
                while !pq.is_empty() && pq.peek().unwrap().0.0 <= event_i_i {
                    user_status[pq.pop().unwrap().0.1 as usize] += 1;
                }
                if events_i[2] == "ALL" {
                    all += 1;
                } else if events_i[2] == "HERE" {
                    for i in 0..user_status.len() {
                        ans[i] += if user_status[i] == 1 { 1 } else { 0 };
                    }
                } else {
                    let t = extract_integers(&events_i[2]);
                    for i in t {
                        ans[i as usize] += 1;
                    }
                }
            } else {
                let event_i_2 = events_i[2].parse::<i32>().unwrap();
                pq.push(Reverse((event_i_i + 60, event_i_2)));
                user_status[event_i_2 as usize] -= 1;
            }
        }

        for ans_i in ans.iter_mut().take(user_status.len()) {
            *ans_i += all;
        }

        ans
    }
}

#[test]
fn test() {
    let number_of_users = 2;
    let events = vec![
        vec!["MESSAGE".to_string(), "10".to_string(), "id1 id0".to_string()],
        vec!["OFFLINE".to_string(), "11".to_string(), "0".to_string()],
        vec!["MESSAGE".to_string(), "71".to_string(), "HERE".to_string()],
    ];
    let expected = vec![2, 2];
    assert_eq!(Solution::count_mentions(number_of_users, events), expected);

    let number_of_users = 2;
    let events = vec![
        vec!["MESSAGE".to_string(), "10".to_string(), "id1 id0".to_string()],
        vec!["OFFLINE".to_string(), "11".to_string(), "0".to_string()],
        vec!["MESSAGE".to_string(), "12".to_string(), "ALL".to_string()],
    ];
    let expected = vec![2, 2];
    assert_eq!(Solution::count_mentions(number_of_users, events), expected);

    let number_of_users = 2;
    let events = vec![
        vec!["OFFLINE".to_string(), "10".to_string(), "0".to_string()],
        vec!["MESSAGE".to_string(), "12".to_string(), "HERE".to_string()],
    ];
    let expected = vec![0, 1];
    assert_eq!(Solution::count_mentions(number_of_users, events), expected);
}
