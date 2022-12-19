#![allow(dead_code)]

// 154. Find Minimum in Rotated Sorted Array II
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/
// https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array-ii/
//
// Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,4,4,5,6,7] might become:
// - [4,5,6,7,0,1,4] if it was rotated 4 times.
// - [0,1,4,4,5,6,7] if it was rotated 7 times.
//
// Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
//
// Given the sorted rotated array nums that may contain duplicates, return the minimum element of this array.
//
// You must decrease the overall operation steps as much as possible.
//

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right): (usize, usize) = (0, nums.len() - 1);

        if nums[left] < nums[right] {
            return nums[left];
        }

        while left < right {
            let mid = left + (right - left) / 2;

            match nums[mid].cmp(&nums[right]) {
                std::cmp::Ordering::Less => {
                    right = mid;
                }
                std::cmp::Ordering::Equal => {
                    right -= 1;
                }
                std::cmp::Ordering::Greater => {
                    left = mid + 1;
                }
            }
        }

        nums[right]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    assert_eq!(Solution::find_min(vec![3, 1, 3]), 1);
    assert_eq!(Solution::find_min(vec![10, 1, 10, 10, 10]), 1);
}
