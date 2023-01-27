#![allow(dead_code)]

// 1353. Maximum Number of Events That Can Be Attended
// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/
// https://leetcode.cn/problems/maximum-number-of-events-that-can-be-attended/
//
// Medium
//
// You are given an array of events where events[i] = [startDayi, endDayi]. Every event i starts at startDayi and ends at endDayi.
//
// You can attend an event i at any day d where startTimei <= d <= endTimei. You can only attend one event at any time d.
//
// Return the maximum number of events you can attend.
//
// Example 1:
//
// Input: events = [[1,2],[2,3],[3,4]]
// Output: 3
// Explanation: You can attend all the three events.
// One way to attend them all is as shown.
// Attend the first event on day 1.
// Attend the second event on day 2.
// Attend the third event on day 3.
//
// Example 2:
//
// Input: events= [[1,2],[2,3],[3,4],[1,2]]
// Output: 4
//
// Constraints:
//
// -    1 <= events.length <= 10^5
// -    events[i].length == 2
// -    1 <= startDayi <= endDayi <= 10^5
//

struct Solution;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        fn find(x: i32, f: &mut Vec<i32>) -> i32 {
            if f[x as usize] == x {
                x
            } else {
                f[x as usize] = find(f[x as usize], f);
                f[x as usize]
            }
        }

        let mut events = events;
        events.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut f = vec![0; events[events.len() - 1][1] as usize + 2];
        for (i, item) in f.iter_mut().enumerate() {
            *item = i as i32;
        }
        let mut ans = 0;
        for it in events.iter() {
            let x = find(it[0], &mut f);
            if x <= it[1] {
                ans += 1;
                f[x as usize] = find(x + 1, &mut f);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let events = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    assert_eq!(Solution::max_events(events), 3);

    let events = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]];
    assert_eq!(Solution::max_events(events), 4);
}
