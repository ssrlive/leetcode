#![allow(dead_code)]

// 495. Teemo Attacking
// https://leetcode.com/problems/teemo-attacking/
// https://leetcode.cn/problems/teemo-attacking/
//
// Our hero Teemo is attacking an enemy Ashe with poison attacks! When Teemo attacks Ashe, Ashe gets poisoned for a exactly duration seconds. More formally, an attack at second t will mean Ashe is poisoned during the inclusive time interval [t, t + duration - 1]. If Teemo attacks again before the poison effect ends, the timer for it is reset, and the poison effect will end duration seconds after the new attack.
//
// You are given a non-decreasing integer array timeSeries, where timeSeries[i] denotes that Teemo attacks Ashe at second timeSeries[i], and an integer duration.
//
// Return the total number of seconds that Ashe is poisoned.
//
// Example 1:
//
// Input: timeSeries = [1,4], duration = 2
// Output: 4
// Explanation: Teemo's attacks on Ashe go as follows:
// - At second 1, Teemo attacks, and Ashe is poisoned for seconds 1 and 2.
// - At second 4, Teemo attacks, and Ashe is poisoned for seconds 4 and 5.
// Ashe is poisoned for seconds 1, 2, 4, and 5, which is 4 seconds in total.
//
// Example 2:
//
// Input: timeSeries = [1,2], duration = 2
// Output: 3
// Explanation: Teemo's attacks on Ashe go as follows:
// - At second 1, Teemo attacks, and Ashe is poisoned for seconds 1 and 2.
// - At second 2 however, Teemo attacks again and resets the poison timer. Ashe is poisoned for seconds 2 and 3.
// Ashe is poisoned for seconds 1, 2, and 3, which is 3 seconds in total.
//
// Constraints:
//
// - 1 <= timeSeries.length <= 10^4
// - 0 <= timeSeries[i], duration <= 10^7
// - timeSeries is sorted in non-decreasing order.
//

struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut poisoned = 0;
        let mut last_attack = 0;
        for &attack in time_series.iter() {
            if attack > last_attack {
                poisoned += duration;
            } else {
                poisoned += attack + duration - last_attack;
            }
            last_attack = attack + duration;
        }
        poisoned
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
    assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
}
