#![allow(dead_code)]

// 3207. Maximum Points After Enemy Battles
// https://leetcode.com/problems/maximum-points-after-enemy-battles/
// https://leetcode.cn/problems/maximum-points-after-enemy-battles/
//
// Medium
//
// You are given an integer array enemyEnergies denoting the energy values of various enemies.
//
// You are also given an integer currentEnergy denoting the amount of energy you have initially.
//
// You start with 0 points, and all the enemies are unmarked initially.
//
// You can perform either of the following operations zero or multiple times to gain points:
//
//     Choose an unmarked enemy, i, such that currentEnergy >= enemyEnergies[i]. By choosing this option:
//         You gain 1 point.
//         Your energy is reduced by the enemy's energy, i.e. currentEnergy = currentEnergy - enemyEnergies[i].
//     If you have at least 1 point, you can choose an unmarked enemy, i. By choosing this option:
//         Your energy increases by the enemy's energy, i.e. currentEnergy = currentEnergy + enemyEnergies[i].
//         The enemy i is marked.
//
// Return an integer denoting the maximum points you can get in the end by optimally performing operations.
//
// Example 1:
//
// Input: enemyEnergies = [3,2,2], currentEnergy = 2
//
// Output: 3
//
// Explanation:
//
// The following operations can be performed to get 3 points, which is the maximum:
//
//     First operation on enemy 1: points increases by 1, and currentEnergy decreases by 2. So, points = 1, and currentEnergy = 0.
//     Second operation on enemy 0: currentEnergy increases by 3, and enemy 0 is marked. So, points = 1, currentEnergy = 3, and marked enemies = [0].
//     First operation on enemy 2: points increases by 1, and currentEnergy decreases by 2. So, points = 2, currentEnergy = 1, and marked enemies = [0].
//     Second operation on enemy 2: currentEnergy increases by 2, and enemy 2 is marked. So, points = 2, currentEnergy = 3, and marked enemies = [0, 2].
//     First operation on enemy 1: points increases by 1, and currentEnergy decreases by 2. So, points = 3, currentEnergy = 1, and marked enemies = [0, 2].
//
// Example 2:
//
// Input: enemyEnergies = [2], currentEnergy = 10
//
// Output: 5
//
// Explanation:
//
// Performing the first operation 5 times on enemy 0 results in the maximum number of points.
//
// Constraints:
//
//     1 <= enemyEnergies.length <= 10^5
//     1 <= enemyEnergies[i] <= 10^9
//     0 <= currentEnergy <= 10^9
//

struct Solution;

impl Solution {
    pub fn maximum_points(enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
        let min = *enemy_energies.iter().min().unwrap() as i64;
        if (current_energy as i64) < min {
            0
        } else {
            (current_energy as i64 - min + enemy_energies.into_iter().map(|x| x as i64).sum::<i64>()) / min
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_points(vec![3, 2, 2], 2), 3);
    assert_eq!(Solution::maximum_points(vec![2], 10), 5);
}
