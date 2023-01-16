#![allow(dead_code)]

// 1010. Pairs of Songs With Total Durations Divisible by 60
// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/
// https://leetcode.cn/problems/pairs-of-songs-with-total-durations-divisible-by-60/
//
// You are given a list of songs where the ith song has a duration of time[i] seconds.
//
// Return the number of pairs of songs for which their total duration in seconds is divisible by 60. Formally, we want the number of indices i, j such that i < j with (time[i] + time[j]) % 60 == 0.
//
// Example 1:
//
// Input: time = [30,20,150,100,40]
// Output: 3
// Explanation: Three pairs have a total duration divisible by 60:
// (time[0] = 30, time[2] = 150): total duration 180
// (time[1] = 20, time[3] = 100): total duration 120
// (time[1] = 20, time[4] = 40): total duration 60
//
// Example 2:
//
// Input: time = [60,60,60]
// Output: 3
// Explanation: All three pairs have a total duration of 120, which is divisible by 60.
//
// Constraints:
//
// - 1 <= time.length <= 6 * 10^4
// - 1 <= time[i] <= 500
//

struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut key: Vec<i32> = vec![0; 61];
        for i in time {
            let rem = (i % 60) as usize;
            if rem == 0 {
                count += key[0]
            }
            count += key[60 - rem];
            key[rem] += 1
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]), 3);
    assert_eq!(Solution::num_pairs_divisible_by60(vec![60, 60, 60]), 3);
}
