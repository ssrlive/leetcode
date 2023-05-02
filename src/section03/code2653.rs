#![allow(dead_code)]

/*

// 2653. Sliding Subarray Beauty
// https://leetcode.com/problems/sliding-subarray-beauty/
// https://leetcode.cn/problems/sliding-subarray-beauty/
//
// Medium
//
// Given an integer array nums containing n integers, find the beauty of each subarray of size k.

The beauty of a subarray is the xth smallest integer in the subarray if it is negative, or 0 if there are fewer than x negative integers.

Return an integer array containing n - k + 1 integers, which denote the beauty of the subarrays in order from the first index in the array.

    A subarray is a contiguous non-empty sequence of elements within an array.

Example 1:

Input: nums = [1,-1,-3,-2,3], k = 3, x = 2
Output: [-1,-2,-2]
Explanation: There are 3 subarrays with size k = 3.
The first subarray is [1, -1, -3] and the 2nd smallest negative integer is -1.
The second subarray is [-1, -3, -2] and the 2nd smallest negative integer is -2.
The third subarray is [-3, -2, 3] and the 2nd smallest negative integer is -2.

Example 2:

Input: nums = [-1,-2,-3,-4,-5], k = 2, x = 2
Output: [-1,-2,-3,-4]
Explanation: There are 4 subarrays with size k = 2.
For [-1, -2], the 2nd smallest negative integer is -1.
For [-2, -3], the 2nd smallest negative integer is -2.
For [-3, -4], the 2nd smallest negative integer is -3.
For [-4, -5], the 2nd smallest negative integer is -4.

Example 3:

Input: nums = [-3,1,2,-3,0,-3], k = 2, x = 1
Output: [-3,0,-3,-3,-3]
Explanation: There are 5 subarrays with size k = 2.
For [-3, 1], the 1st smallest negative integer is -3.
For [1, 2], there is no negative integer so the beauty is 0.
For [2, -3], the 1st smallest negative integer is -3.
For [-3, 0], the 1st smallest negative integer is -3.
For [0, -3], the 1st smallest negative integer is -3.

Constraints:

    n == nums.length
    1 <= n <= 10^5
    1 <= k <= n
    1 <= x <= k
    -50 <= nums[i] <= 50
*/

struct Solution;

impl Solution {
    pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut mp = std::collections::BTreeMap::new();
        for i in 0..nums.len() {
            *mp.entry(nums[i]).or_insert(0) += 1;
            if i >= k as usize - 1 {
                if i >= k as usize {
                    *mp.get_mut(&nums[i - k as usize]).unwrap() -= 1;
                }
                let mut sum = 0;
                let mut t = 0;
                for (key, value) in mp.iter() {
                    sum += value;
                    if sum >= x {
                        t = if *key < 0 { *key } else { 0 };
                        break;
                    }
                }
                ans.push(t);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, -1, -3, -2, 3], 3, 2, vec![-1, -2, -2]),
        (vec![-1, -2, -3, -4, -5], 2, 2, vec![-1, -2, -3, -4]),
        (vec![-3, 1, 2, -3, 0, -3], 2, 1, vec![-3, 0, -3, -3, -3]),
    ];
    for (nums, k, x, expect) in cases {
        assert_eq!(Solution::get_subarray_beauty(nums, k, x), expect);
    }
}
