#![allow(dead_code)]

// 3347. Maximum Frequency of an Element After Performing Operations II
// https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-ii/
// https://leetcode.cn/problems/maximum-frequency-of-an-element-after-performing-operations-ii/
//
// Hard
//
// You are given an integer array nums and two integers k and numOperations.
//
// You must perform an operation numOperations times on nums, where in each operation you:
//
//     Select an index i that was not selected in any previous operations.
//     Add an integer in the range [-k, k] to nums[i].
//
// Return the maximum possible frequency of any element in nums after performing the operations.
//
// Example 1:
//
// Input: nums = [1,4,5], k = 1, numOperations = 2
//
// Output: 2
//
// Explanation:
//
// We can achieve a maximum frequency of two by:
//
//     Adding 0 to nums[1], after which nums becomes [1, 4, 5].
//     Adding -1 to nums[2], after which nums becomes [1, 4, 4].
//
// Example 2:
//
// Input: nums = [5,11,20,20], k = 5, numOperations = 1
//
// Output: 2
//
// Explanation:
//
// We can achieve a maximum frequency of two by:
//
//     Adding 0 to nums[1].
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^9
//     0 <= k <= 10^9
//     0 <= numOperations <= nums.length
//

struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        use std::collections::{BTreeMap, BTreeSet, HashMap};
        let mut mp = BTreeMap::new();
        let mut hash = HashMap::new();
        let mut points = BTreeSet::new();
        for &it in nums.iter() {
            *hash.entry(it).or_insert(0) += 1;
            *mp.entry(it - k).or_insert(0) += 1;
            *mp.entry(it + k + 1).or_insert(0) -= 1;
            points.insert(it);
            points.insert(it - k);
            points.insert(it + k + 1);
        }
        let mut ans = 1;
        let mut sum = 0;
        for &it in points.iter() {
            sum += mp.get(&it).unwrap_or(&0);
            let hash_val = hash.get(&it).unwrap_or(&0);
            ans = ans.max(hash_val + (sum - hash_val).min(num_operations));
        }
        ans
    }
}

#[test]
fn test() {
    let nums = vec![1, 4, 5];
    let k = 1;
    let num_operations = 2;
    let output = 2;
    assert_eq!(Solution::max_frequency(nums, k, num_operations), output);

    let nums = vec![5, 11, 20, 20];
    let k = 5;
    let num_operations = 1;
    let output = 2;
    assert_eq!(Solution::max_frequency(nums, k, num_operations), output);
}
