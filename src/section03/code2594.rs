#![allow(dead_code)]

/*

// 2594. Minimum Time to Repair Cars
// https://leetcode.com/problems/minimum-time-to-repair-cars/
// https://leetcode.cn/problems/minimum-time-to-repair-cars/
//
// Medium
//
// You are given an integer array ranks representing the ranks of some mechanics.
// ranksi is the rank of the ith mechanic. A mechanic with a rank r can repair n cars in r * n2 minutes.

You are also given an integer cars representing the total number of cars waiting in the garage to be repaired.

Return the minimum time taken to repair all the cars.

Note: All the mechanics can repair the cars simultaneously.

Example 1:

Input: ranks = [4,2,3,1], cars = 10
Output: 16
Explanation:
- The first mechanic will repair two cars. The time required is 4 * 2 * 2 = 16 minutes.
- The second mechanic will repair two cars. The time required is 2 * 2 * 2 = 8 minutes.
- The third mechanic will repair two cars. The time required is 3 * 2 * 2 = 12 minutes.
- The fourth mechanic will repair four cars. The time required is 1 * 4 * 4 = 16 minutes.
It can be proved that the cars cannot be repaired in less than 16 minutes.​​​​​

Example 2:

Input: ranks = [5,1,8], cars = 6
Output: 16
Explanation:
- The first mechanic will repair one car. The time required is 5 * 1 * 1 = 5 minutes.
- The second mechanic will repair four cars. The time required is 1 * 4 * 4 = 16 minutes.
- The third mechanic will repair one car. The time required is 8 * 1 * 1 = 8 minutes.
It can be proved that the cars cannot be repaired in less than 16 minutes.​​​​​

Constraints:

    1 <= ranks.length <= 10^5
    1 <= ranks[i] <= 100
    1 <= cars <= 10^6
*/

struct Solution;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        fn check(ranks: &Vec<i32>, cars: i32, mid: i64) -> bool {
            let mut cars = cars as i64;
            for r in ranks {
                let r = *r as i64;
                let mut d = (mid as f64 / r as f64).sqrt() as i64 + 1;
                if d * d * r > mid {
                    d -= 1;
                }
                cars -= d;
                if cars <= 0 {
                    return true;
                }
            }
            false
        }

        let (mut left, mut right) = (1, i64::MAX);
        while left < right {
            let mid = left + (right - left) / 2;
            if check(&ranks, cars, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[test]
fn test() {
    assert_eq!(Solution::repair_cars(vec![4, 2, 3, 1], 10), 16);
    assert_eq!(Solution::repair_cars(vec![5, 1, 8], 6), 16);
}
