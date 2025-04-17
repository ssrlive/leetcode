#![allow(dead_code)]

// 3134. Find the Median of the Uniqueness Array
// https://leetcode.com/problems/find-the-median-of-the-uniqueness-array/
// https://leetcode.cn/problems/find-the-median-of-the-uniqueness-array/
//
// Hard
//
// You are given an integer array nums. The uniqueness array of nums is the sorted array that contains the number of distinct elements of all the
// subarrays of nums. In other words, it is a sorted array consisting of distinct(nums[i..j]), for all 0 <= i <= j < nums.length.
//
// Here, distinct(nums[i..j]) denotes the number of distinct elements in the subarray that starts at index i and ends at index j.
//
// Return the median of the uniqueness array of nums.
//
// Note that the median of an array is defined as the middle element of the array when it is sorted in non-decreasing order.
// If there are two choices for a median, the smaller of the two values is taken.
//
// Example 1:
//
// Input: nums = [1,2,3]
//
// Output: 1
//
// Explanation:
//
// The uniqueness array of nums is [distinct(nums[0..0]), distinct(nums[1..1]), distinct(nums[2..2]), distinct(nums[0..1]), distinct(nums[1..2]),
// distinct(nums[0..2])] which is equal to [1, 1, 1, 2, 2, 3]. The uniqueness array has a median of 1. Therefore, the answer is 1.
//
// Example 2:
//
// Input: nums = [3,4,3,4,5]
//
// Output: 2
//
// Explanation:
//
// The uniqueness array of nums is [1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3]. The uniqueness array has a median of 2. Therefore, the answer is 2.
//
// Example 3:
//
// Input: nums = [4,3,5,4]
//
// Output: 2
//
// Explanation:
//
// The uniqueness array of nums is [1, 1, 1, 1, 2, 2, 2, 3, 3, 3]. The uniqueness array has a median of 2. Therefore, the answer is 2.
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let sz = n * (n + 1) / 2;
        let mut l = 1;
        let mut r = n as i32;
        let median = sz.div_ceil(2);

        while l < r {
            let m = (l + r) / 2;
            if Self::subarrays_with_distinct_up_to_k(&nums, m as usize) < median as i64 {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l
    }

    pub fn subarrays_with_distinct_up_to_k(nums: &[i32], k: usize) -> i64 {
        let mut cnt = vec![0; 100001];
        let mut res = 0;
        let mut j = 0;
        let sz = nums.len();
        let mut distinct_count = 0;

        for i in 0..sz {
            let num = nums[i] as usize;
            cnt[num] += 1;
            if cnt[num] == 1 {
                distinct_count += 1;
            }
            while j < sz && distinct_count > k {
                let j_num = nums[j] as usize;
                cnt[j_num] -= 1;
                if cnt[j_num] == 0 {
                    distinct_count -= 1;
                }
                j += 1;
            }
            res += (i - j + 1) as i64;
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::median_of_uniqueness_array(vec![1, 2, 3]), 1);
    assert_eq!(Solution::median_of_uniqueness_array(vec![3, 4, 3, 4, 5]), 2);
    assert_eq!(Solution::median_of_uniqueness_array(vec![4, 3, 5, 4]), 2);
}
