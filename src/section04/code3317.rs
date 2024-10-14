#![allow(dead_code)]

// 3317. Find the Number of Possible Ways for an Event
// https://leetcode.com/problems/find-the-number-of-possible-ways-for-an-event/
// https://leetcode.cn/problems/find-the-number-of-possible-ways-for-an-event/
//
// Hard
//
// You are given three integers n, x, and y.
//
// An event is being held for n performers. When a performer arrives, they are assigned to one of the x stages.
// All performers assigned to the same stage will perform together as a band, though some stages might remain empty.
//
// After all performances are completed, the jury will award each band a score in the range [1, y].
//
// Return the total number of possible ways the event can take place.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Note that two events are considered to have been held differently if either of the following conditions is satisfied:
//
//     Any performer is assigned a different stage.
//     Any band is awarded a different score.
//
// Example 1:
//
// Input: n = 1, x = 2, y = 3
//
// Output: 6
//
// Explanation:
//
//     There are 2 ways to assign a stage to the performer.
//     The jury can award a score of either 1, 2, or 3 to the only band.
//
// Example 2:
//
// Input: n = 5, x = 2, y = 1
//
// Output: 32
//
// Explanation:
//
//     Each performer will be assigned either stage 1 or stage 2.
//     All bands will be awarded a score of 1.
//
// Example 3:
//
// Input: n = 3, x = 3, y = 4
//
// Output: 684
//
// Constraints:
//
//     1 <= n, x, y <= 1000
//

struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
        let mut s: Vec<Vec<i64>> = vec![vec![0; 1001]; 1001];
        let mod_num = 1e9 as i64 + 7;
        s[0][0] = 1;
        for i in 1..=1000 {
            for j in 1..=i {
                s[i][j] = (s[i - 1][j - 1] + s[i - 1][j] * j as i64) % mod_num;
            }
        }

        let mut res = 0;
        let mut perm = 1;
        let mut pow = 1;
        for i in 1..=std::cmp::min(n as i64, x as i64) {
            perm = perm * (x as i64 - i + 1) % mod_num;
            pow = pow * y as i64 % mod_num;
            res = (res + perm * s[n as usize][i as usize] % mod_num * pow) % mod_num;
        }
        res as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_ways(1, 2, 3), 6);
    assert_eq!(Solution::number_of_ways(5, 2, 1), 32);
    assert_eq!(Solution::number_of_ways(3, 3, 4), 684);
    assert_eq!(Solution::number_of_ways(76, 31, 194), 544840908);
}
