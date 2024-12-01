#![allow(dead_code)]

// 3371. Identify the Largest Outlier in an Array
// https://leetcode.com/problems/identify-the-largest-outlier-in-an-array/
// https://leetcode.cn/problems/identify-the-largest-outlier-in-an-array/
//
// Medium
//
// You are given an integer array nums. This array contains n elements, where exactly n - 2 elements are special numbers.
// One of the remaining two elements is the sum of these special numbers, and the other is an outlier.
//
// An outlier is defined as a number that is neither one of the original special numbers nor the element representing the sum of those numbers.
//
// Note that special numbers, the sum element, and the outlier must have distinct indices, but may share the same value.
//
// Return the largest potential outlier in nums.
//
// Example 1:
//
// Input: nums = [2,3,5,10]
//
// Output: 10
//
// Explanation:
//
// The special numbers could be 2 and 3, thus making their sum 5 and the outlier 10.
//
// Example 2:
//
// Input: nums = [-2,-1,-3,-6,4]
//
// Output: 4
//
// Explanation:
//
// The special numbers could be -2, -1, and -3, thus making their sum -6 and the outlier 4.
//
// Example 3:
//
// Input: nums = [1,1,1,1,1,5,5]
//
// Output: 5
//
// Explanation:
//
// The special numbers could be 1, 1, 1, 1, and 1, thus making their sum 5 and the other 5 as the outlier.
//
// Constraints:
//
// 3 <= nums.length <= 10^5
// -1000 <= nums[i] <= 1000
// The input is generated such that at least one potential outlier exists in nums.
//

struct Solution;

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut sum = 0;
        let mut freq = std::collections::HashMap::new();
        for n in nums.iter() {
            sum += n;
            *freq.entry(n * 2).or_insert(0) += 1;
        }
        for n in nums.iter() {
            let t = sum - n;
            let freq_t = *freq.get(&t).unwrap_or(&0);
            if freq_t >= 2 || (freq_t == 1) && (t != n * 2) {
                ans = ans.max(*n);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 5, 10];
    let res = 10;
    assert_eq!(Solution::get_largest_outlier(nums), res);

    let nums = vec![-2, -1, -3, -6, 4];
    let res = 4;
    assert_eq!(Solution::get_largest_outlier(nums), res);

    let nums = vec![1, 1, 1, 1, 1, 5, 5];
    let res = 5;
    assert_eq!(Solution::get_largest_outlier(nums), res);
}
