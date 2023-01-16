#![allow(dead_code)]

// 875. Koko Eating Bananas
// https://leetcode.com/problems/koko-eating-bananas/
// https://leetcode.cn/problems/koko-eating-bananas/
//
// Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.
//
// Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile.
// If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.
//
// Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
//
// Return the minimum integer k such that she can eat all the bananas within h hours.
//
// Example 1:
//
// Input: piles = [3,6,7,11], h = 8
// Output: 4
//
// Example 2:
//
// Input: piles = [30,11,23,4,20], h = 5
// Output: 30
//
// Example 3:
//
// Input: piles = [30,11,23,4,20], h = 6
// Output: 23
//
// Constraints:
//
// - 1 <= piles.length <= 10^4
// - piles.length <= h <= 10^9
// - 1 <= piles[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = 1_000_000_000;

        while left < right {
            let mid: i32 = left + (right - left) / 2;
            let mut cnt: i32 = 0;

            for pile in piles.iter() {
                cnt += (*pile as f64 / mid as f64).ceil() as i32;
                if cnt > h {
                    break;
                }
            }
            if cnt > h {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        right
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
}
