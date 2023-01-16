#![allow(dead_code)]

// 697. Degree of an Array
// https://leetcode.com/problems/degree-of-an-array/
// https://leetcode.cn/problems/degree-of-an-array/
//
// Given a non-empty array of non-negative integers nums, the degree of this array is defined as the maximum frequency of any one of its elements.
//
// Your task is to find the smallest possible length of a (contiguous) subarray of nums, that has the same degree as nums.
//
// Example 1:
//
// Input: nums = [1,2,2,3,1]
// Output: 2
// Explanation:
// The input array has a degree of 2 because both elements 1 and 2 appear twice.
// Of the subarrays that have the same degree:
// [1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
// The shortest length is 2. So return 2.
//
// Example 2:
//
// Input: nums = [1,2,2,3,1,4,2]
// Output: 6
// Explanation:
// The degree is 3 because the element 2 is repeated 3 times.
// So [2,2,3,1,4,2] is the shortest subarray, therefore returning 6.
//
// Constraints:
// - nums.length will be between 1 and 50,000.
// - nums[i] will be an integer between 0 and 49,999.
//

struct Solution;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            map.entry(n).or_insert((1, i, i)).0 += 1;
            map.entry(n).or_insert((1, i, i)).2 = i;
        }
        let mut max = 0;
        let mut min = 0;
        for (_, (count, start, end)) in map.iter() {
            match max.cmp(count) {
                std::cmp::Ordering::Less => {
                    max = *count;
                    min = end - start + 1;
                }
                std::cmp::Ordering::Equal => {
                    min = min.min(end - start + 1);
                }
                _ => {}
            }
        }
        min as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]), 6);
}
