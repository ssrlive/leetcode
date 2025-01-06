#![allow(dead_code)]

// 3410. Maximize Subarray Sum After Removing All Occurrences of One Element
// https://leetcode.com/problems/maximize-subarray-sum-after-removing-all-occurrences-of-one-element/
// https://leetcode.cn/problems/maximize-subarray-sum-after-removing-all-occurrences-of-one-element/
//
// Hard
//
// You are given an integer array nums.
//
// You can do the following operation on the array at most once:
//
// - Choose any integer x such that nums remains non-empty on removing all occurrences of x.
// - Remove all occurrences of x from the array.
//
// Return the maximum subarray sum across all possible resulting arrays.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [-3,2,-2,-1,3,-2,3]
//
// Output: 7
//
// Explanation:
//
// We can have the following arrays after at most one operation:
//
//     The original array is nums = [-3, 2, -2, -1, 3, -2, 3]. The maximum subarray sum is 3 + (-2) + 3 = 4.
//     Deleting all occurences of x = -3 results in nums = [2, -2, -1, 3, -2, 3]. The maximum subarray sum is 3 + (-2) + 3 = 4.
//     Deleting all occurences of x = -2 results in nums = [-3, 2, -1, 3, 3]. The maximum subarray sum is 2 + (-1) + 3 + 3 = 7.
//     Deleting all occurences of x = -1 results in nums = [-3, 2, -2, 3, -2, 3]. The maximum subarray sum is 3 + (-2) + 3 = 4.
//     Deleting all occurences of x = 3 results in nums = [-3, 2, -2, -1, -2]. The maximum subarray sum is 2.
//
// The output is max(4, 4, 7, 4, 2) = 7.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
//
// Output: 10
//
// Explanation:
//
// It is optimal to not perform any operations.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     -10^6 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>) -> i64 {
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let mut pre = std::collections::HashMap::new();
        let mut res = nums[0];
        let mut prefix = 0;
        let mut low = 0;
        pre.insert(0, 0);
        for &n in nums.iter() {
            prefix += n;
            res = res.max(prefix - low);
            if n < 0 {
                if pre.contains_key(&n) {
                    pre.insert(n, pre[&n].min(pre[&0]) + n);
                } else {
                    pre.insert(n, pre[&0] + n);
                }
                low = low.min(pre[&n]);
            }
            pre.insert(0, pre[&0].min(prefix));
            low = low.min(pre[&0]);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![-3, 2, -2, -1, 3, -2, 3];
    let res = 7;
    assert_eq!(Solution::max_subarray_sum(nums), res);

    let nums = vec![1, 2, 3, 4];
    let res = 10;
    assert_eq!(Solution::max_subarray_sum(nums), res);
}
