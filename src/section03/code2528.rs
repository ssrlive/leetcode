#![allow(dead_code)]

// 2528. Maximize the Minimum Powered City
// https://leetcode.com/problems/maximize-the-minimum-powered-city/
// https://leetcode.cn/problems/maximize-the-minimum-powered-city/
//
// You are given a 0-indexed integer array stations of length n, where stations[i] represents the number of power stations in the ith city.
//
// Each power station can provide power to every city in a fixed range. In other words, if the range is denoted by r,
// then a power station at city i can provide power to all cities j such that |i - j| <= r and 0 <= i, j <= n - 1.
//
// Note that |x| denotes absolute value. For example, |7 - 5| = 2 and |3 - 10| = 7.
// The power of a city is the total number of power stations it is being provided power from.
//
// The government has sanctioned building k more power stations, each of which can be built in any city, and have the same range as the pre-existing ones.
//
// Given the two integers r and k, return the maximum possible minimum power of a city, if the additional power stations are built optimally.
//
// Note that you can build the k power stations in multiple cities.
//
// Example 1:
//
// Input: stations = [1,2,4,5,0], r = 1, k = 2
// Output: 5
// Explanation:
// One of the optimal ways is to install both the power stations at city 1.
// So stations will become [1,4,4,5,0].
// - City 0 is provided by 1 + 4 = 5 power stations.
// - City 1 is provided by 1 + 4 + 4 = 9 power stations.
// - City 2 is provided by 4 + 4 + 5 = 13 power stations.
// - City 3 is provided by 5 + 4 = 9 power stations.
// - City 4 is provided by 5 + 0 = 5 power stations.
// So the minimum power of a city is 5.
// Since it is not possible to obtain a larger power, we return 5.
//
// Example 2:
//
// Input: stations = [4,4,4,4], r = 0, k = 3
// Output: 4
// Explanation:
// It can be proved that we cannot make the minimum power of a city greater than 4.
//
// Constraints:
//
// - n == stations.length
// - 1 <= n <= 10^5
// - 0 <= stations[i] <= 10^5
// - 0 <= r <= n - 1
// - 0 <= k <= 10^9
//

struct Solution;

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        fn check(stations: &[i32], r: i32, k: i64, amt: i64) -> bool {
            let mut stations = stations.to_vec();
            let (n, mut sum, mut k) = (stations.len(), 0, k);
            let (mut left, mut right) = (0, r as usize);

            for &station in stations.iter().skip(left).take(right + 1) {
                sum += station as i64;
            }
            for i in 0..n {
                if amt > sum {
                    if amt > sum + k {
                        return false;
                    }
                    k -= amt - sum;
                    stations[right] += (amt - sum) as i32;
                    sum = amt;
                }
                if left as i32 == i as i32 - r {
                    sum -= stations[left] as i64;
                    left += 1;
                }

                if right + 1 < n {
                    right += 1;
                    sum += stations[right] as i64;
                }
            }

            true
        }

        let (mut left, mut right) = (0, i64::MAX);

        while left < right {
            let mid = right - (right - left) / 2;
            if check(&stations, r, k as i64, mid) {
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
    let cases = vec![
        (vec![1, 2, 4, 5, 0], 1, 2, 5),
        (vec![4, 4, 4, 4], 0, 3, 4),
        (vec![0, 0, 0, 0], 2, 3, 3),
        (vec![0, 0, 0, 0], 2, 4, 4),
    ];
    for (stations, r, k, expected) in cases {
        assert_eq!(Solution::max_power(stations, r, k), expected);
    }
}
