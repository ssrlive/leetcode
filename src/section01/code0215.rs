#![allow(dead_code)]

// 215. Kth Largest Element in an Array
// https://leetcode.com/problems/kth-largest-element-in-an-array/
// https://leetcode.cn/problems/kth-largest-element-in-an-array/
//
// Medium
//
// Given an integer array nums and an integer k, return the kth largest element in the array.
//
// Note that it is the kth largest element in the sorted order, not the kth distinct element.
//
// You must solve it in O(n) time complexity.
//
// Example 1:
//
// Input: nums = [3,2,1,5,6,4], k = 2
// Output: 5
//
// Example 2:
//
// Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
// Output: 4
//
// Constraints:
//
// -    1 <= k <= nums.length <= 10^5
// -    -10^4 <= nums[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        *nums.select_nth_unstable_by((k - 1) as usize, |a, b| b.cmp(a)).1
    }

    pub fn find_kth_largest_3(nums: Vec<i32>, k: i32) -> i32 {
        fn _find_kth_largest(nums: Vec<i32>, k: i32) -> Option<i32> {
            let mut nums = nums;
            nums.sort();
            nums.get(nums.len() - k as usize).copied()
        }

        _find_kth_largest(nums, k).unwrap_or_default()
    }

    pub fn find_kth_largest_2(nums: Vec<i32>, k: i32) -> i32 {
        fn _find_kth_largest(nums: Vec<i32>, k: i32) -> Option<i32> {
            let mut heap = std::collections::BinaryHeap::new();
            for n in nums {
                heap.push(n);
            }
            for _ in 0..k - 1 {
                heap.pop();
            }
            heap.pop()
        }

        _find_kth_largest(nums, k).unwrap_or_default()
    }
}

#[test]
fn test_find_kth_largest() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
}
