#![allow(dead_code)]

/*

// 2188. Minimum Time to Finish the Race
// https://leetcode.com/problems/minimum-time-to-finish-the-race/
// https://leetcode.cn/problems/minimum-time-to-finish-the-race/
//
// Hard
//
// You are given a 0-indexed 2D integer array tires where tires[i] = [fi, ri] indicates that the ith tire can finish its xth successive lap in fi * ri(x-1) seconds.

    For example, if fi = 3 and ri = 2, then the tire would finish its 1st lap in 3 seconds, its 2nd lap in 3 * 2 = 6 seconds, its 3rd lap in 3 * 22 = 12 seconds, etc.

You are also given an integer changeTime and an integer numLaps.

The race consists of numLaps laps and you may start the race with any tire. You have an unlimited supply of each tire and after every lap, you may change to any given tire (including the current tire type) if you wait changeTime seconds.

Return the minimum time to finish the race.

Example 1:

Input: tires = [[2,3],[3,4]], changeTime = 5, numLaps = 4
Output: 21
Explanation:
Lap 1: Start with tire 0 and finish the lap in 2 seconds.
Lap 2: Continue with tire 0 and finish the lap in 2 * 3 = 6 seconds.
Lap 3: Change tires to a new tire 0 for 5 seconds and then finish the lap in another 2 seconds.
Lap 4: Continue with tire 0 and finish the lap in 2 * 3 = 6 seconds.
Total time = 2 + 6 + 5 + 2 + 6 = 21 seconds.
The minimum time to complete the race is 21 seconds.

Example 2:

Input: tires = [[1,10],[2,2],[3,4]], changeTime = 6, numLaps = 5
Output: 25
Explanation:
Lap 1: Start with tire 1 and finish the lap in 2 seconds.
Lap 2: Continue with tire 1 and finish the lap in 2 * 2 = 4 seconds.
Lap 3: Change tires to a new tire 1 for 6 seconds and then finish the lap in another 2 seconds.
Lap 4: Continue with tire 1 and finish the lap in 2 * 2 = 4 seconds.
Lap 5: Change tires to tire 0 for 6 seconds then finish the lap in another 1 second.
Total time = 2 + 4 + 6 + 2 + 4 + 6 + 1 = 25 seconds.
The minimum time to complete the race is 25 seconds.

Constraints:

    1 <= tires.length <= 10^5
    tires[i].length == 2
    1 <= fi, changeTime <= 10^5
    2 <= ri <= 10^5
    1 <= numLaps <= 1000
*/

struct Solution;

impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        use std::collections::*;
        let mut a: BTreeMap<usize, i32> = BTreeMap::new();
        for t in &tires {
            let (mut f, r, mut c, mut i) = (t[0], t[1], t[0], 0_usize);
            while c < 1000000 {
                if !a.contains_key(&i) || a[&i] > c {
                    a.insert(i, c);
                }
                f *= r;
                c += f;
                i += 1;
            }
        }
        let a: Vec<_> = a.values().collect();
        let mut dp = vec![i32::MAX; num_laps as usize + 1];
        dp[0] = 0;
        for i in 0..a.len() {
            for j in (i + 1)..dp.len() {
                dp[j] = dp[j].min(dp[j - i - 1] + a[i] + change_time);
            }
        }
        dp[num_laps as usize] - change_time
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![2, 3], vec![3, 4]], 5, 4, 21),
        (vec![vec![1, 10], vec![2, 2], vec![3, 4]], 6, 5, 25),
    ];
    for (tires, change_time, num_laps, expected) in cases {
        assert_eq!(Solution::minimum_finish_time(tires, change_time, num_laps), expected);
    }
}
