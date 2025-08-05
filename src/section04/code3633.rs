#![allow(dead_code)]

// 3633. Earliest Finish Time for Land and Water Rides I
// https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-i/
// https://leetcode.cn/problems/earliest-finish-time-for-land-and-water-rides-i/
//
// Easy
//
// You are given two categories of theme park attractions: land rides and water rides.
//
//     Land rides
//         landStartTime[i] – the earliest time the ith land ride can be boarded.
//         landDuration[i] – how long the ith land ride lasts.
//     Water rides
//         waterStartTime[j] – the earliest time the jth water ride can be boarded.
//         waterDuration[j] – how long the jth water ride lasts.
//
// A tourist must experience exactly one ride from each category, in either order.
//
//     A ride may be started at its opening time or any later moment.
//     If a ride is started at time t, it finishes at time t + duration.
//     Immediately after finishing one ride the tourist may board the other (if it is already open) or wait until it opens.
//
// Return the earliest possible time at which the tourist can finish both rides.
//
// Example 1:
//
// Input: landStartTime = [2,8], landDuration = [4,1], waterStartTime = [6], waterDuration = [3]
//
// Output: 9
//
// Explanation:​​​​​​​
//
//     Plan A (land ride 0 → water ride 0):
//         Start land ride 0 at time landStartTime[0] = 2. Finish at 2 + landDuration[0] = 6.
//         Water ride 0 opens at time waterStartTime[0] = 6. Start immediately at 6, finish at 6 + waterDuration[0] = 9.
//     Plan B (water ride 0 → land ride 1):
//         Start water ride 0 at time waterStartTime[0] = 6. Finish at 6 + waterDuration[0] = 9.
//         Land ride 1 opens at landStartTime[1] = 8. Start at time 9, finish at 9 + landDuration[1] = 10.
//     Plan C (land ride 1 → water ride 0):
//         Start land ride 1 at time landStartTime[1] = 8. Finish at 8 + landDuration[1] = 9.
//         Water ride 0 opened at waterStartTime[0] = 6. Start at time 9, finish at 9 + waterDuration[0] = 12.
//     Plan D (water ride 0 → land ride 0):
//         Start water ride 0 at time waterStartTime[0] = 6. Finish at 6 + waterDuration[0] = 9.
//         Land ride 0 opened at landStartTime[0] = 2. Start at time 9, finish at 9 + landDuration[0] = 13.
//
// Plan A gives the earliest finish time of 9.
//
// Example 2:
//
// Input: landStartTime = [5], landDuration = [3], waterStartTime = [1], waterDuration = [10]
//
// Output: 14
//
// Explanation:​​​​​​​
//
//     Plan A (water ride 0 → land ride 0):
//         Start water ride 0 at time waterStartTime[0] = 1. Finish at 1 + waterDuration[0] = 11.
//         Land ride 0 opened at landStartTime[0] = 5. Start immediately at 11 and finish at 11 + landDuration[0] = 14.
//     Plan B (land ride 0 → water ride 0):
//         Start land ride 0 at time landStartTime[0] = 5. Finish at 5 + landDuration[0] = 8.
//         Water ride 0 opened at waterStartTime[0] = 1. Start immediately at 8 and finish at 8 + waterDuration[0] = 18.
//
// Plan A provides the earliest finish time of 14.​​​​​​​
//
// Constraints:
//
//     1 <= n, m <= 100
//     landStartTime.length == landDuration.length == n
//     waterStartTime.length == waterDuration.length == m
//     1 <= landStartTime[i], landDuration[i], waterStartTime[j], waterDuration[j] <= 1000
//

struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let len_l = land_duration.len();
        let len_w = water_duration.len();

        let mut mn = i32::MAX;

        // start with land ride
        for l in 0..len_l {
            let ls = land_start_time[l];
            let ld = land_duration[l];
            for w in 0..len_w {
                let ws = water_start_time[w];
                let wd = water_duration[w];

                if ws < (ls + ld) {
                    mn = std::cmp::min(mn, ls + ld + wd);
                } else {
                    mn = std::cmp::min(mn, ws + wd)
                }
            }
        }

        // start with water ride
        for w in 0..len_w {
            let ws = water_start_time[w];
            let wd = water_duration[w];

            for l in 0..len_l {
                let ls = land_start_time[l];
                let ld = land_duration[l];

                if ls < (ws + wd) {
                    mn = std::cmp::min(mn, ws + wd + ld);
                } else {
                    mn = std::cmp::min(mn, ls + ld);
                }
            }
        }

        mn
    }
}

#[test]
fn test() {
    assert_eq!(Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]), 9);
    assert_eq!(Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10]), 14);
}
