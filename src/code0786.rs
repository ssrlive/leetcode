#![allow(dead_code)]

// 786. K-th Smallest Prime Fraction
// https://leetcode.com/problems/k-th-smallest-prime-fraction/
// https://leetcode.cn/problems/k-th-smallest-prime-fraction/
//
// You are given a sorted integer array arr containing 1 and prime numbers,
// where all the integers of arr are unique. You are also given an integer k.
//
// For every i and j where 0 <= i < j < arr.length, we consider the fraction arr[i] / arr[j].
//
// Return the kth smallest fraction considered. Return your answer as an array of integers of size 2,
// where answer[0] == arr[i] and answer[1] == arr[j].
//
// Example 1:
//
// Input: arr = [1,2,3,5], k = 3
// Output: [2,5]
// Explanation: The fractions to be considered in sorted order are:
// 1/5, 1/3, 2/5, 1/2, 3/5, and 2/3.
// The third fraction is 2/5.
//
// Example 2:
//
// Input: arr = [1,7], k = 1
// Output: [1,7]
//
// Constraints:
//
// - 2 <= arr.length <= 1000
// - 1 <= arr[i] <= 3 * 104
// - arr[0] == 1
// - arr[i] is a prime number for i > 0.
// - All the numbers of arr are unique and sorted in strictly increasing order.
// - 1 <= k <= arr.length * (arr.length - 1) / 2
//
// Follow up: Can you solve the problem with better than O(n2) complexity?
//

struct Solution;

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut lo = 0.0;
        let mut hi = 1.0;
        let k = k as usize;
        let n = arr.len();
        while lo < hi {
            let mid = (lo + hi) / 2.0;
            let mut count = 0;
            let mut j = 1;
            let mut best = vec![0, 1];
            for i in 0..n - 1 {
                while j < n && arr[i] as f64 > mid * arr[j] as f64 {
                    j += 1;
                }
                count += n - j;
                if j < n && best[0] * arr[j] < best[1] * arr[i] {
                    best = vec![arr[i], arr[j]];
                }
            }
            match count.cmp(&k) {
                std::cmp::Ordering::Equal => return best,
                std::cmp::Ordering::Less => lo = mid,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        vec![0, 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3), vec![2, 5]);
    assert_eq!(Solution::kth_smallest_prime_fraction(vec![1, 7], 1), vec![1, 7]);
}
