#![allow(dead_code)]

/*
// 2615. Sum of Distances
// https://leetcode.com/problems/sum-of-distances/
//
// Medium
//
// You are given a 0-indexed integer array nums. There exists an array arr of length nums.length,
// where arr[i] is the sum of |i - j| over all j such that nums[j] == nums[i] and j != i. If there is no such j, set arr[i] to be 0.

Return the array arr.

Example 1:

Input: nums = [1,3,1,1,2]
Output: [5,0,3,4,0]
Explanation:
When i = 0, nums[0] == nums[2] and nums[0] == nums[3]. Therefore, arr[0] = |0 - 2| + |0 - 3| = 5.
When i = 1, arr[1] = 0 because there is no other index with value 3.
When i = 2, nums[2] == nums[0] and nums[2] == nums[3]. Therefore, arr[2] = |2 - 0| + |2 - 3| = 3.
When i = 3, nums[3] == nums[0] and nums[3] == nums[2]. Therefore, arr[3] = |3 - 0| + |3 - 2| = 4.
When i = 4, arr[4] = 0 because there is no other index with value 2.

Example 2:

Input: nums = [0,5,3]
Output: [0,0,0]
Explanation: Since each element in nums is distinct, arr[i] = 0 for all i.

Constraints:

1 <= nums.length <= 10^5
0 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut res = vec![0; nums.len()];
        let mut sum_l = std::collections::HashMap::new();
        let mut sum_r = std::collections::HashMap::new();
        let mut cnt_l = std::collections::HashMap::new();
        let mut cnt_r = std::collections::HashMap::new();
        for i in 0..nums.len() {
            res[i] = cnt_l.get(&nums[i]).unwrap_or(&0) * i as i64 - sum_l.get(&nums[i]).unwrap_or(&0);
            *sum_l.entry(nums[i]).or_insert(0) += i as i64;
            *cnt_l.entry(nums[i]).or_insert(0) += 1;
        }
        for i in (0..nums.len()).rev() {
            res[i] += sum_r.get(&nums[i]).unwrap_or(&0) - cnt_r.get(&nums[i]).unwrap_or(&0) * i as i64;
            *sum_r.entry(nums[i]).or_insert(0) += i as i64;
            *cnt_r.entry(nums[i]).or_insert(0) += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::distance(vec![1, 3, 1, 1, 2]), vec![5, 0, 3, 4, 0]);
    assert_eq!(Solution::distance(vec![0, 5, 3]), vec![0, 0, 0]);
}
