#![allow(dead_code)]

// 2517. Maximum Tastiness of Candy Basket
// https://leetcode.com/problems/maximum-tastiness-of-candy-basket/
// https://leetcode.cn/problems/maximum-tastiness-of-candy-basket/
//
// You are given an array of positive integers price where price[i] denotes the price of the ith candy and a positive integer k.
//
// The store sells baskets of k distinct candies. The tastiness of a candy basket is the smallest absolute difference of the prices of any two candies in the basket.
//
// Return the maximum tastiness of a candy basket.
//
// Example 1:
//
// Input: price = [13,5,1,8,21,2], k = 3
// Output: 8
// Explanation: Choose the candies with the prices [13,5,21].
// The tastiness of the candy basket is: min(|13 - 5|, |13 - 21|, |5 - 21|) = min(8, 8, 16) = 8.
// It can be proven that 8 is the maximum tastiness that can be achieved.
//
// Example 2:
//
// Input: price = [1,3,1], k = 2
// Output: 2
// Explanation: Choose the candies with the prices [1,3].
// The tastiness of the candy basket is: min(|1 - 3|) = min(2) = 2.
// It can be proven that 2 is the maximum tastiness that can be achieved.
//
// Example 3:
//
// Input: price = [7,7,7,7], k = 2
// Output: 0
// Explanation: Choosing any two distinct candies from the candies we have will result in a tastiness of 0.
//
// Constraints:
//
// - 1 <= price.length <= 10^5
// - 1 <= price[i] <= 10^9
// - 2 <= k <= price.length
//

struct Solution;

impl Solution {
    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        fn check(data: &Vec<i32>, k: i32, mid: i32) -> bool {
            let n = data.len();
            let mut last = data[0];
            let mut k = k - 1;

            for &item in data.iter().take(n) {
                if item < last + mid {
                    continue;
                }

                last = item;
                k -= 1;
                if k == 0 {
                    return true;
                }
            }

            false
        }

        use std::collections::HashSet;
        let mut s = HashSet::new();
        for p in price {
            s.insert(p);
        }

        let mut data = vec![];
        for a in s.into_iter() {
            data.push(a);
        }
        data.sort();

        if data.len() < k as usize {
            return 0;
        }
        let (mut left, mut right) = (1, data[data.len() - 1] - data[0]);

        while left < right {
            let mid = right - (right - left) / 2;
            if check(&data, k, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_tastiness(vec![13, 5, 1, 8, 21, 2], 3), 8);
    assert_eq!(Solution::maximum_tastiness(vec![1, 3, 1], 2), 2);
    assert_eq!(Solution::maximum_tastiness(vec![7, 7, 7, 7], 2), 0);
}
