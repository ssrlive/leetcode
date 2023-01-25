#![allow(dead_code)]

// 1326. Minimum Number of Taps to Open to Water a Garden
// https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/
// https://leetcode.cn/problems/minimum-number-of-taps-to-open-to-water-a-garden/
//
// Hard
//
// There is a one-dimensional garden on the x-axis. The garden starts at the point 0 and ends at the point n. (i.e The length of the garden is n).
//
// There are n + 1 taps located at points [0, 1, ..., n] in the garden.
//
// Given an integer n and an integer array ranges of length n + 1 where ranges[i] (0-indexed) means the i-th tap can water the area [i - ranges[i], i + ranges[i]] if it was open.
//
// Return the minimum number of taps that should be open to water the whole garden, If the garden cannot be watered return -1.
//
// Example 1:
//
// Input: n = 5, ranges = [3,4,1,1,0,0]
// Output: 1
// Explanation: The tap at point 0 can cover the interval [-3,3]
// The tap at point 1 can cover the interval [-3,5]
// The tap at point 2 can cover the interval [1,3]
// The tap at point 3 can cover the interval [2,4]
// The tap at point 4 can cover the interval [4,4]
// The tap at point 5 can cover the interval [5,5]
// Opening Only the second tap will water the whole garden [0,5]
//
// Example 2:
//
// Input: n = 3, ranges = [0,0,0,0]
// Output: -1
// Explanation: Even if you activate all the four taps you cannot water the whole garden.
//
// Constraints:
//
// -    1 <= n <= 10^4
// -    ranges.length == n + 1
// -    0 <= ranges[i] <= 100
//

struct Solution;

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let (mut taps, mut maxi, mut mini, mut last_tap) = (0, 0, 0, 0);
        while maxi < n {
            let tmp = last_tap;
            for i in tmp..=n {
                if i - ranges[i as usize] <= mini && i + ranges[i as usize] > maxi {
                    last_tap = i;
                    maxi = i + ranges[i as usize];
                }
            }
            if maxi == mini {
                return -1;
            }
            taps += 1;
            mini = maxi;
        }
        taps
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
    assert_eq!(Solution::min_taps(3, vec![0, 0, 0, 0]), -1);
}
