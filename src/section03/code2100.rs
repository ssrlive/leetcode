#![allow(dead_code)]

/*

// 2100. Find Good Days to Rob the Bank
// https://leetcode.com/problems/find-good-days-to-rob-the-bank/
// https://leetcode.cn/problems/find-good-days-to-rob-the-bank/
//
// Medium
//
// You and a gang of thieves are planning on robbing a bank. You are given a 0-indexed integer array security,
// where security[i] is the number of guards on duty on the ith day. The days are numbered starting from 0.
// You are also given an integer time.

The ith day is a good day to rob the bank if:

    There are at least time days before and after the ith day,
    The number of guards at the bank for the time days before i are non-increasing, and
    The number of guards at the bank for the time days after i are non-decreasing.

More formally, this means day i is a good day to rob the bank if and only if security[i - time] >= security[i - time + 1] >= ... >= security[i] <= ... <= security[i + time - 1] <= security[i + time].

Return a list of all days (0-indexed) that are good days to rob the bank. The order that the days are returned in does not matter.

Example 1:

Input: security = [5,3,3,3,5,6,2], time = 2
Output: [2,3]
Explanation:
On day 2, we have security[0] >= security[1] >= security[2] <= security[3] <= security[4].
On day 3, we have security[1] >= security[2] >= security[3] <= security[4] <= security[5].
No other days satisfy this condition, so days 2 and 3 are the only good days to rob the bank.

Example 2:

Input: security = [1,1,1,1,1], time = 0
Output: [0,1,2,3,4]
Explanation:
Since time equals 0, every day is a good day to rob the bank, so return every day.

Example 3:

Input: security = [1,2,3,4,5,6], time = 2
Output: []
Explanation:
No day has 2 days before it that have a non-increasing number of guards.
Thus, no day is a good day to rob the bank, so return an empty list.

Constraints:

    1 <= security.length <= 10^5
    0 <= security[i], time <= 10^5
*/

struct Solution;

impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        use std::collections::HashSet;
        // use std::iter::FromIterator;
        let mut pl = i32::MIN;
        let mut pr = i32::MIN;

        let mut cl = 0;
        let mut cr = 0;

        let mut a: HashSet<i32> = HashSet::new();
        let mut b: HashSet<i32> = HashSet::new();
        for i in 0..security.len() {
            if security[i] <= pl {
                cl += 1;
            } else {
                cl = 0;
            }
            if security[security.len() - 1 - i] <= pr {
                cr += 1;
            } else {
                cr = 0;
            }

            if cl >= time {
                a.insert(i as i32);
            }
            if cr >= time {
                b.insert((security.len() - 1 - i) as i32);
            }
            pl = security[i];
            pr = security[security.len() - 1 - i];
        }
        let x = Vec::from_iter((&a & &b).iter().cloned());
        x
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![5, 3, 3, 3, 5, 6, 2], 2, vec![2, 3]),
        (vec![1, 1, 1, 1, 1], 0, vec![0, 1, 2, 3, 4]),
        (vec![1, 2, 3, 4, 5, 6], 2, vec![]),
    ];
    for (security, time, expected) in cases {
        let mut v = Solution::good_days_to_rob_bank(security, time);
        v.sort();
        assert_eq!(v, expected);
    }
}
