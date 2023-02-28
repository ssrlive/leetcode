#![allow(dead_code)]

/*

// 1942. The Number of the Smallest Unoccupied Chair
// https://leetcode.com/problems/the-number-of-the-smallest-unoccupied-chair/
// https://leetcode.cn/problems/the-number-of-the-smallest-unoccupied-chair/
//
// Medium
//
// There is a party where n friends numbered from 0 to n - 1 are attending. There is an infinite number of chairs in this party that are numbered from 0 to infinity. When a friend arrives at the party, they sit on the unoccupied chair with the smallest number.

    For example, if chairs 0, 1, and 5 are occupied when a friend comes, they will sit on chair number 2.

When a friend leaves the party, their chair becomes unoccupied at the moment they leave. If another friend arrives at that same moment, they can sit in that chair.

You are given a 0-indexed 2D integer array times where times[i] = [arrivali, leavingi], indicating the arrival and leaving times of the ith friend respectively, and an integer targetFriend. All arrival times are distinct.

Return the chair number that the friend numbered targetFriend will sit on.

Example 1:

Input: times = [[1,4],[2,3],[4,6]], targetFriend = 1
Output: 1
Explanation:
- Friend 0 arrives at time 1 and sits on chair 0.
- Friend 1 arrives at time 2 and sits on chair 1.
- Friend 1 leaves at time 3 and chair 1 becomes empty.
- Friend 0 leaves at time 4 and chair 0 becomes empty.
- Friend 2 arrives at time 4 and sits on chair 0.
Since friend 1 sat on chair 1, we return 1.

Example 2:

Input: times = [[3,10],[1,5],[2,6]], targetFriend = 0
Output: 2
Explanation:
- Friend 1 arrives at time 1 and sits on chair 0.
- Friend 2 arrives at time 2 and sits on chair 1.
- Friend 0 arrives at time 3 and sits on chair 2.
- Friend 1 leaves at time 5 and chair 0 becomes empty.
- Friend 2 leaves at time 6 and chair 1 becomes empty.
- Friend 0 leaves at time 10 and chair 2 becomes empty.
Since friend 0 sat on chair 2, we return 2.

Constraints:

    n == times.length
    2 <= n <= 10^4
    times[i].length == 2
    1 <= arrivali < leavingi <= 10^5
    0 <= targetFriend <= n - 1
    Each arrivali time is distinct.
*/

struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Event {
    arrival: bool,
    index: usize,
    time: i32,
}

impl Event {
    fn new(arrival: bool, index: usize, time: i32) -> Event {
        Event { arrival, index, time }
    }
}

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        fn compare(a: &Event, b: &Event) -> Ordering {
            let res = a.time.cmp(&b.time);
            if res == Ordering::Equal {
                if !a.arrival && b.arrival {
                    return Ordering::Less;
                }
                if a.arrival && !b.arrival {
                    return Ordering::Greater;
                }
            }
            res
        }

        let mut pq = BinaryHeap::<i32>::new();
        let mut map = HashMap::<usize, i32>::new();
        let mut highest = 0;
        let mut chair: i32 = 0;

        let mut events: Vec<Event> = times
            .iter()
            .enumerate()
            .flat_map(|e| [Event::new(true, e.0, e.1[0]), Event::new(false, e.0, e.1[1])])
            .collect();

        events.sort_by(compare);

        for e in events {
            if e.arrival {
                if pq.is_empty() {
                    chair = highest;
                    highest += 1;
                } else {
                    chair = -pq.pop().unwrap();
                }
                if e.index == target_friend as usize {
                    return chair;
                }
                map.insert(e.index, chair);
            } else {
                pq.push(-map.get(&e.index).unwrap());
            }
        }
        chair
    }
}

#[test]
fn test() {
    let times = vec![vec![1, 4], vec![2, 3], vec![4, 6]];
    let target_friend = 1;
    let result = Solution::smallest_chair(times, target_friend);
    assert_eq!(result, 1);

    let times = vec![vec![3, 10], vec![1, 5], vec![2, 6]];
    let target_friend = 0;
    let result = Solution::smallest_chair(times, target_friend);
    assert_eq!(result, 2);
}
