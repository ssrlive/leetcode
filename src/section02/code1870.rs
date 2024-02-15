#![allow(dead_code)]

/*

// 1870. Minimum Speed to Arrive on Time
// https://leetcode.com/problems/minimum-speed-to-arrive-on-time/
// https://leetcode.cn/problems/minimum-speed-to-arrive-on-time/
//
// Medium
//
// You are given a floating-point number hour, representing the amount of time you have to reach the office. To commute to the office, you must take n trains in sequential order. You are also given an integer array dist of length n, where dist[i] describes the distance (in kilometers) of the ith train ride.

Each train can only depart at an integer hour, so you may need to wait in between each train ride.

    For example, if the 1st train ride takes 1.5 hours, you must wait for an additional 0.5 hours before you can depart on the 2nd train ride at the 2 hour mark.

Return the minimum positive integer speed (in kilometers per hour) that all the trains must travel at for you to reach the office on time, or -1 if it is impossible to be on time.

Tests are generated such that the answer will not exceed 107 and hour will have at most two digits after the decimal point.

Example 1:

Input: dist = [1,3,2], hour = 6
Output: 1
Explanation: At speed 1:
- The first train ride takes 1/1 = 1 hour.
- Since we are already at an integer hour, we depart immediately at the 1 hour mark. The second train takes 3/1 = 3 hours.
- Since we are already at an integer hour, we depart immediately at the 4 hour mark. The third train takes 2/1 = 2 hours.
- You will arrive at exactly the 6 hour mark.

Example 2:

Input: dist = [1,3,2], hour = 2.7
Output: 3
Explanation: At speed 3:
- The first train ride takes 1/3 = 0.33333 hours.
- Since we are not at an integer hour, we wait until the 1 hour mark to depart. The second train ride takes 3/3 = 1 hour.
- Since we are already at an integer hour, we depart immediately at the 2 hour mark. The third train takes 2/3 = 0.66667 hours.
- You will arrive at the 2.66667 hour mark.

Example 3:

Input: dist = [1,3,2], hour = 1.9
Output: -1
Explanation: It is impossible because the earliest the third train can depart is at the 2 hour mark.

Constraints:

    n == dist.length
    1 <= n <= 10^5
    1 <= dist[i] <= 10^5
    1 <= hour <= 10^9
    There will be at most two digits after the decimal point in hour.
*/

struct Solution;

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        fn good(dist: &[i64], hour: f64, h: i64) -> bool {
            let mut cnt = 0.0;
            for i in 0..dist.len() {
                if i == dist.len() - 1 {
                    cnt += dist[i] as f64 / h as f64;
                } else {
                    cnt += ((dist[i] + h - 1) / h) as f64;
                }
            }
            cnt <= hour
        }

        let dist: Vec<i64> = dist.iter().map(|x| *x as i64).collect();
        let n = dist.len();
        if hour <= (n - 1) as f64 {
            return -1;
        }
        let mut lo = 1;
        let mut hi = 1e9 as i64;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if good(&dist, hour, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 3, 2], 6.0, 1),
        (vec![1, 3, 2], 2.7, 3),
        (vec![1, 3, 2], 1.9, -1),
        (vec![1, 1, 100000], 2.01, 10000000),
    ];
    for (dist, hour, expect) in cases {
        assert_eq!(Solution::min_speed_on_time(dist, hour), expect);
    }
}
