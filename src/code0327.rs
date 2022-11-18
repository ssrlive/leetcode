#![allow(dead_code)]

// 327. Count of Range Sum
// https://leetcode.com/problems/count-of-range-sum/
//
// Given an integer array nums, return the number of range sums that lie in
// [lower, upper] inclusive.
// Range sum S(i, j) is defined as the sum of the elements in nums between
// indices i and j (i ≤ j), inclusive.
//
// Note:
// A naive algorithm of O(n2) is trivial. You MUST do better than that.
//
// Example:
//
// Input: nums = [-2,5,-1], lower = -2, upper = 2,
// Output: 3
// Explanation: The three ranges are : [0,0], [2,2], [0,2] and their respective sums are: -2, -1, 2.
//
// Constraints:
//
// 0 <= nums.length <= 10^4
//

struct Solution;

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        let mut sum = vec![0; n + 1];
        for i in 1..=n {
            sum[i] = sum[i - 1] + nums[i - 1] as i64;
        }
        Self::merge(&mut sum, 0, n, lower as i64, upper as i64) as i32
    }

    fn merge(sum: &mut [i64], l: usize, r: usize, lower: i64, upper: i64) -> usize {
        if l >= r {
            return 0;
        }
        let mid = (l + r) / 2;
        let mut count = 0;
        count += Self::merge(sum, l, mid, lower, upper);
        count += Self::merge(sum, mid + 1, r, lower, upper);
        let mut m = mid + 1;
        let mut n = mid + 1;
        for i in l..=mid {
            while m <= r && sum[m] - sum[i] < lower {
                m += 1;
            }
            while n <= r && sum[n] - sum[i] <= upper {
                n += 1;
            }
            count += n - m;
        }
        let left = sum[l..=mid].to_vec();
        let right = sum[(mid + 1)..=r].to_vec();
        let mut i = l;
        let mut ll = 0;
        let mut rr = 0;
        while ll < left.len() && rr < right.len() {
            if left[ll] < right[rr] {
                sum[i] = left[ll];
                ll += 1;
            } else {
                sum[i] = right[rr];
                rr += 1;
            }
            i += 1;
        }
        while rr < right.len() {
            sum[i] = right[rr];
            rr += 1;
            i += 1;
        }
        while ll < left.len() {
            sum[i] = left[ll];
            ll += 1;
            i += 1;
        }
        count
    }

    // // slow solution
    // pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
    //     let mut sum = 0;
    //     let mut sums = vec![0];
    //     for num in nums.iter() {
    //         sum += *num as i64;
    //         sums.push(sum);
    //     }
    //     Self::count_while_merge_sort(&mut sums, lower as i64, upper as i64) as i32
    // }
    // fn count_while_merge_sort(sums: &mut [i64], lower: i64, upper: i64) -> usize {
    //     let len = sums.len();
    //     if len <= 1 {
    //         return 0;
    //     }
    //     let mid = len / 2;
    //     let mut count = Self::count_while_merge_sort(&mut sums[..mid], lower, upper)
    //         + Self::count_while_merge_sort(&mut sums[mid..], lower, upper);
    //     let mut j = mid;
    //     let mut k = mid;
    //     let mut t = mid;
    //     let mut cache = vec![];
    //     for i in 0..mid {
    //         let left = sums[i];
    //         while j < len && sums[j] - left < lower {
    //             j += 1;
    //         }
    //         while k < len && sums[k] - left <= upper {
    //             k += 1;
    //         }
    //         while t < len && sums[t] < left {
    //             cache.push(sums[t]);
    //             t += 1;
    //         }
    //         cache.push(left);
    //         count += k - j;
    //     }
    //     loop {
    //         if cache.is_empty() {
    //             break;
    //         }
    //         sums[t - (cache.len() + 1)] = cache.remove(0);
    //     }
    //     count
    // }
}

#[test]
fn test_count_range_sum() {
    assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
    assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 3), 4);
    assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);
}
