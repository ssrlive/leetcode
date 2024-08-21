#![allow(dead_code)]

// 3259. Maximum Energy Boost From Two Drinks
// https://leetcode.com/problems/maximum-energy-boost-from-two-drinks/
// https://leetcode.cn/problems/maximum-energy-boost-from-two-drinks/
//
// Medium
//
// You are given two integer arrays energyDrinkA and energyDrinkB of the same length n
// by a futuristic sports scientist. These arrays represent the energy boosts per
// hour provided by two different energy drinks, A and B, respectively.
//
// You want to maximize your total energy boost by drinking one energy drink per hour.
// However, if you want to switch from consuming one energy drink to the other,
// you need to wait for one hour to cleanse your system (meaning you won't get any energy boost in that hour).
//
// Return the maximum total energy boost you can gain in the next n hours.
//
// Note that you can start consuming either of the two energy drinks.
//
// Example 1:
//
// Input: energyDrinkA = [1,3,1], energyDrinkB = [3,1,1]
//
// Output: 5
//
// Explanation:
//
// To gain an energy boost of 5, drink only the energy drink A (or only B).
//
// Example 2:
//
// Input: energyDrinkA = [4,1,1], energyDrinkB = [1,1,3]
//
// Output: 7
//
// Explanation:
//
// To gain an energy boost of 7:
//
//     Drink the energy drink A for the first hour.
//     Switch to the energy drink B and we lose the energy boost of the second hour.
//     Gain the energy boost of the drink B in the third hour.
//
// Constraints:
//
//     n == energyDrinkA.length == energyDrinkB.length
//     3 <= n <= 10^5
//     1 <= energyDrinkA[i], energyDrinkB[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn max_energy_boost(arr1: Vec<i32>, arr2: Vec<i32>) -> i64 {
        let mut res1 = vec![0; arr1.len() + 1];
        let mut res2 = vec![0; res1.len()];

        res1[1] = arr1[0] as i64;
        res2[1] = arr2[0] as i64;

        for i in 1..arr1.len() {
            res1[i + 1] = res1[i].max(res2[i - 1]) + arr1[i] as i64;
            res2[i + 1] = res2[i].max(res1[i - 1]) + arr2[i] as i64;
        }

        res1[res1.len() - 1].max(res2[res1.len() - 1])
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_energy_boost(vec![1, 3, 1], vec![3, 1, 1]), 5);
    assert_eq!(Solution::max_energy_boost(vec![4, 1, 1], vec![1, 1, 3]), 7);
}
