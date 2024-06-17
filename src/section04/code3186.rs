#![allow(dead_code)]

// 3186. Maximum Total Damage With Spell Casting
// https://leetcode.com/problems/maximum-total-damage-with-spell-casting/
// https://leetcode.cn/problems/maximum-total-damage-with-spell-casting/
//
// Medium
//
// A magician has various spells.
//
// You are given an array power, where each element represents the damage of a spell. Multiple spells can have the same damage value.
//
// It is a known fact that if a magician decides to cast a spell with a damage of power[i],
// they cannot cast any spell with a damage of power[i] - 2, power[i] - 1, power[i] + 1, or power[i] + 2.
//
// Each spell can be cast only once.
//
// Return the maximum possible total damage that a magician can cast.
//
// Example 1:
//
// Input: power = [1,1,3,4]
//
// Output: 6
//
// Explanation:
//
// The maximum possible damage of 6 is produced by casting spells 0, 1, 3 with damage 1, 1, 4.
//
// Example 2:
//
// Input: power = [7,1,6,6]
//
// Output: 13
//
// Explanation:
//
// The maximum possible damage of 13 is produced by casting spells 1, 2, 3 with damage 1, 6, 6.
//
// Constraints:
//
//     1 <= power.length <= 10^5
//     1 <= power[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut power = power;
        power.sort_unstable();
        let (vals, count) = {
            let (mut vals, mut count) = (Vec::new(), Vec::new());
            for p in power {
                if vals.is_empty() || vals[vals.len() - 1] != p as i64 {
                    vals.push(p as i64);
                    count.push(1);
                } else {
                    *count.last_mut().unwrap() += 1;
                }
            }
            (vals, count)
        };

        let mut dp = vec![0; vals.len()];

        for i in 0..dp.len() {
            let res = vals[i] * count[i];
            let mut max_add = 0;

            for j in 1..=5 {
                if i >= j && vals[i] - vals[i - j] > 2 {
                    max_add = i64::max(max_add, dp[i - j]);
                }
            }

            dp[i] = res + max_add;
        }

        *dp.iter().max().unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_total_damage(vec![1, 1, 3, 4]), 6);
    assert_eq!(Solution::maximum_total_damage(vec![7, 1, 6, 6]), 13);
}
