#![allow(dead_code)]

// 605. Can Place Flowers
// https://leetcode.com/problems/can-place-flowers/
//
// You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
//
// Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n,
// return if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule.
//
// Example 1:
//
// Input: flowerbed = [1,0,0,0,1], n = 1
// Output: true
//
// Example 2:
//
// Input: flowerbed = [1,0,0,0,1], n = 2
// Output: false
//
// Constraints:
//
// - 1 <= flowerbed.length <= 2 * 10^4
// - flowerbed[i] is 0 or 1.
// - There are no two adjacent flowers in flowerbed.
// - 0 <= n <= flowerbed.length
//

struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed;
        let mut n = n;
        let mut i = 0;
        while i < flowerbed.len() {
            if flowerbed[i] == 0 {
                let mut can_plant = true;
                if i > 0 && flowerbed[i - 1] == 1 {
                    can_plant = false;
                }
                if i < flowerbed.len() - 1 && flowerbed[i + 1] == 1 {
                    can_plant = false;
                }
                if can_plant {
                    flowerbed[i] = 1;
                    n -= 1;
                }
            }
            if n <= 0 {
                return true;
            }
            i += 1;
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
}
