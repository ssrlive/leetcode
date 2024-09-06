#![allow(dead_code)]

// 3273. Minimum Amount of Damage Dealt to Bob
// https://leetcode.com/problems/minimum-amount-of-damage-dealt-to-bob/
// https://leetcode.cn/problems/minimum-amount-of-damage-dealt-to-bob/
//
// Hard
//
// You are given an integer power and two integer arrays damage and health, both having length n.
//
// Bob has n enemies, where enemy i will deal Bob damage[i] points of damage per second while they are alive (i.e. health[i] > 0).
//
// Every second, after the enemies deal damage to Bob, he chooses one of the enemies that is still alive and deals power points of damage to them.
//
// Determine the minimum total amount of damage points that will be dealt to Bob before all n enemies are dead.
//
// Example 1:
//
// Input: power = 4, damage = [1,2,3,4], health = [4,5,6,8]
//
// Output: 39
//
// Explanation:
//
//     Attack enemy 3 in the first two seconds, after which enemy 3 will go down, the number of damage points dealt to Bob is 10 + 10 = 20 points.
//     Attack enemy 2 in the next two seconds, after which enemy 2 will go down, the number of damage points dealt to Bob is 6 + 6 = 12 points.
//     Attack enemy 0 in the next second, after which enemy 0 will go down, the number of damage points dealt to Bob is 3 points.
//     Attack enemy 1 in the next two seconds, after which enemy 1 will go down, the number of damage points dealt to Bob is 2 + 2 = 4 points.
//
// Example 2:
//
// Input: power = 1, damage = [1,1,1,1], health = [1,2,3,4]
//
// Output: 20
//
// Explanation:
//
//     Attack enemy 0 in the first second, after which enemy 0 will go down, the number of damage points dealt to Bob is 4 points.
//     Attack enemy 1 in the next two seconds, after which enemy 1 will go down, the number of damage points dealt to Bob is 3 + 3 = 6 points.
//     Attack enemy 2 in the next three seconds, after which enemy 2 will go down, the number of damage points dealt to Bob is 2 + 2 + 2 = 6 points.
//     Attack enemy 3 in the next four seconds, after which enemy 3 will go down, the number of damage points dealt to Bob is 1 + 1 + 1 + 1 = 4 points.
//
// Example 3:
//
// Input: power = 8, damage = [40], health = [59]
//
// Output: 320
//
// Constraints:
//
//     1 <= power <= 10^4
//     1 <= n == damage.length == health.length <= 10^5
//     1 <= damage[i], health[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
        let n = damage.len();
        let time: Vec<_> = health.iter().map(|&h| (h - 1) / power + 1).collect();
        let mut order: Vec<_> = (0..n).collect();
        order.sort_unstable_by(|&a, &b| (time[a] * damage[b]).cmp(&(time[b] * damage[a])));
        let mut res = 0;
        let mut t = 0;
        for i in order {
            t += time[i] as i64;
            res += t * damage[i] as i64;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_damage(4, vec![1, 2, 3, 4], vec![4, 5, 6, 8]), 39);
    assert_eq!(Solution::min_damage(1, vec![1, 1, 1, 1], vec![1, 2, 3, 4]), 20);
    assert_eq!(Solution::min_damage(8, vec![40], vec![59]), 320);
}
