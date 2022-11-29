#![allow(dead_code)]

// 475. Heaters
// https://leetcode.com/problems/heaters/
//
// Winter is coming! During the contest, your first job is to design a standard heater with a fixed warm radius to warm all the houses.
//
// Every house can be warmed, as long as the house is within the heater's warm radius range.
//
// Given the positions of houses and heaters on a horizontal line, return the minimum radius standard of heaters so that those heaters could cover all houses.
//
// Notice that all the heaters follow your radius standard, and the warm radius will the same.
//
// Example 1:
//
// Input: houses = [1,2,3], heaters = [2]
// Output: 1
// Explanation: The only heater was placed in the position 2, and if we use the radius 1 standard, then all the houses can be warmed.
//
// Example 2:
//
// Input: houses = [1,2,3,4], heaters = [1,4]
// Output: 1
// Explanation: The two heater was placed in the position 1 and 4. We need to use radius 1 standard, then all the houses can be warmed.
//
// Example 3:
//
// Input: houses = [1,5], heaters = [2]
// Output: 3
//
// Constraints:
//
// - 1 <= houses.length, heaters.length <= 3 * 104
// - 1 <= houses[i], heaters[i] <= 109
//

struct Solution;

impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        let mut heaters = heaters;
        houses.sort();
        heaters.sort();
        let mut i = 0;
        let mut ans = 0;
        for house in houses {
            while i < heaters.len() - 1 && heaters[i] + heaters[i + 1] <= house * 2 {
                i += 1;
            }
            ans = ans.max((house - heaters[i]).abs());
        }
        ans
    }
}

#[test]
fn test_find_radius() {
    let houses = vec![1, 2, 3];
    let heaters = vec![2];
    assert_eq!(Solution::find_radius(houses, heaters), 1);

    let houses = vec![1, 2, 3, 4];
    let heaters = vec![1, 4];
    assert_eq!(Solution::find_radius(houses, heaters), 1);

    let houses = vec![1, 5];
    let heaters = vec![2];
    assert_eq!(Solution::find_radius(houses, heaters), 3);
}
