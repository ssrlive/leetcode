#![allow(dead_code)]

/*

// 2012. Sum of Beauty in the Array
// https://leetcode.com/problems/sum-of-beauty-in-the-array/
// https://leetcode.cn/problems/sum-of-beauty-in-the-array/
//
// Medium
//
// You are given a 0-indexed integer array nums. For each index i (1 <= i <= nums.length - 2) the beauty of nums[i] equals:

    2, if nums[j] < nums[i] < nums[k], for all 0 <= j < i and for all i < k <= nums.length - 1.
    1, if nums[i - 1] < nums[i] < nums[i + 1], and the previous condition is not satisfied.
    0, if none of the previous conditions holds.

Return the sum of beauty of all nums[i] where 1 <= i <= nums.length - 2.

Example 1:

Input: nums = [1,2,3]
Output: 2
Explanation: For each index i in the range 1 <= i <= 1:
- The beauty of nums[1] equals 2.

Example 2:

Input: nums = [2,4,6,4]
Output: 1
Explanation: For each index i in the range 1 <= i <= 2:
- The beauty of nums[1] equals 1.
- The beauty of nums[2] equals 0.

Example 3:

Input: nums = [3,2,1]
Output: 0
Explanation: For each index i in the range 1 <= i <= 1:
- The beauty of nums[1] equals 0.

Constraints:

    3 <= nums.length <= 10^5
    1 <= nums[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        use std::collections::*;
        let n = nums.len();
        let mut count = 0;
        let mut btreemap = BTreeMap::new();
        for &num in nums.iter().take(n).skip(1) {
            *btreemap.entry(num).or_insert(0) += 1;
        }

        let mut max = nums[0];
        for i in 1..n - 1 {
            let v = nums[i];

            let entry = btreemap.entry(v).or_insert(0);
            if *entry == 1 {
                btreemap.remove(&v);
            } else {
                *entry -= 1;
            }
            let min = btreemap.iter().next().unwrap();

            if max < v && v < *min.0 {
                count += 2;
            } else if nums[i - 1] < v && v < nums[i + 1] {
                count += 1;
            }
            max = max.max(v);
        }
        count
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3], 2),
        (vec![2, 4, 6, 4], 1),
        (vec![3, 2, 1], 0),
        (vec![1, 2, 3, 4, 5], 6),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::sum_of_beauties(nums), expected);
    }
}
