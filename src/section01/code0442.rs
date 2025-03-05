#![allow(dead_code)]

// 442. Find All Duplicates in an Array
// https://leetcode.com/problems/find-all-duplicates-in-an-array/
// https://leetcode.cn/problems/find-all-duplicates-in-an-array/
//
// Given an integer array nums of length n where all the integers of nums are in the range [1, n] and
// each integer appears once or twice, return an array of all the integers that appears twice.
//
// You must write an algorithm that runs in O(n) time and uses only constant extra space.
//
// Example 1:
//
// Input: nums = [4,3,2,7,8,2,3,1]
// Output: [2,3]
//
// Example 2:
//
// Input: nums = [1,1,2]
// Output: [1]
//
// Example 3:
//
// Input: nums = [1]
// Output: []
//
// Constraints:
//
// - n == nums.length
// - 1 <= n <= 10^5
// - 1 <= nums[i] <= n
// - Each element in nums appears once or twice.
//

struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut set = std::collections::HashSet::new();
        let mut result = Vec::new();
        for num in nums {
            if set.contains(&num) {
                result.push(num);
            } else {
                set.insert(num);
            }
        }
        result
    }

    pub fn find_duplicates_2(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for &num in nums.iter() {
            *map.entry(num).or_insert(0) += 1;
        }
        map.iter().filter(|&(_, &v)| v > 1).map(|(&k, _)| k).collect()
    }
}

#[test]
fn test() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    let mut result = Solution::find_duplicates(nums);
    result.sort();
    assert_eq!(result, vec![2, 3]);

    assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
    assert_eq!(Solution::find_duplicates(vec![1]), vec![]);

    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    let mut result = Solution::find_duplicates_2(nums);
    result.sort();
    assert_eq!(result, vec![2, 3]);

    assert_eq!(Solution::find_duplicates_2(vec![1, 1, 2]), vec![1]);
    assert_eq!(Solution::find_duplicates_2(vec![1]), vec![]);
}
