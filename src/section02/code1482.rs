#![allow(dead_code)]

/*

// 1482. Minimum Number of Days to Make m Bouquets
// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/
// https://leetcode.cn/problems/minimum-number-of-days-to-make-m-bouquets/
//
// Medium
//
// You are given an integer array bloomDay, an integer m and an integer k.

You want to make m bouquets. To make a bouquet, you need to use k adjacent flowers from the garden.

The garden consists of n flowers, the ith flower will bloom in the bloomDay[i] and then can be used in exactly one bouquet.

Return the minimum number of days you need to wait to be able to make m bouquets from the garden. If it is impossible to make m bouquets return -1.

Example 1:

Input: bloomDay = [1,10,3,10,2], m = 3, k = 1
Output: 3
Explanation: Let us see what happened in the first three days. x means flower bloomed and _ means flower did not bloom in the garden.
We need 3 bouquets each should contain 1 flower.
After day 1: [x, _, _, _, _]   // we can only make one bouquet.
After day 2: [x, _, _, _, x]   // we can only make two bouquets.
After day 3: [x, _, x, _, x]   // we can make 3 bouquets. The answer is 3.

Example 2:

Input: bloomDay = [1,10,3,10,2], m = 3, k = 2
Output: -1
Explanation: We need 3 bouquets each has 2 flowers, that means we need 6 flowers. We only have 5 flowers so it is impossible to get the needed bouquets and we return -1.

Example 3:

Input: bloomDay = [7,7,7,7,12,7,7], m = 2, k = 3
Output: 12
Explanation: We need 2 bouquets each should have 3 flowers.
Here is the garden after the 7 and 12 days:
After day 7: [x, x, x, x, _, x, x]
We can make one bouquet of the first three flowers that bloomed. We cannot make another bouquet from the last three flowers that bloomed because they are not adjacent.
After day 12: [x, x, x, x, x, x, x]
It is obvious that we can make two bouquets in different ways.

Constraints:

    bloomDay.length == n
    1 <= n <= 10^5
    1 <= bloomDay[i] <= 10^9
    1 <= m <= 10^6
    1 <= k <= n
*/

struct Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let bloom_day = bloom_day.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let (m, k) = (m as i64, k as i64);
        let n = bloom_day.len();
        if m * k > n as i64 {
            return -1;
        }
        let mut left = 1;
        let mut right = 1_000_000_000;
        while left < right {
            let mid = (left + right) / 2;
            let mut bouquets = 0;
            let mut flowers = 0;
            for &item in bloom_day.iter().take(n) {
                if item <= mid {
                    flowers += 1;
                    if flowers == k {
                        bouquets += 1;
                        flowers = 0;
                    }
                } else {
                    flowers = 0;
                }
            }
            if bouquets >= m {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 10, 3, 10, 2], 3, 1, 3),
        (vec![1, 10, 3, 10, 2], 3, 2, -1),
        (vec![7, 7, 7, 7, 12, 7, 7], 2, 3, 12),
    ];
    for (bloom_day, m, k, expected) in cases {
        assert_eq!(Solution::min_days(bloom_day, m, k), expected);
    }
}
