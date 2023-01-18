#![allow(dead_code)]

// 1248. Count Number of Nice Subarrays
// https://leetcode.com/problems/count-number-of-nice-subarrays/
// https://leetcode.cn/problems/count-number-of-nice-subarrays/
//
// Medium
// Given an array of integers nums and an integer k. A continuous subarray is called nice if there are k odd numbers on it.
//
// Return the number of nice sub-arrays.
//
// Example 1:
//
// Input: nums = [1,1,2,1,1], k = 3
// Output: 2
// Explanation: The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].
//
// Example 2:
//
// Input: nums = [2,4,6], k = 1
// Output: 0
// Explanation: There is no odd numbers in the array.
//
// Example 3:
//
// Input: nums = [2,2,2,1,2,2,1,2,2,2], k = 2
// Output: 16
//
// Constraints:
//
// -    1 <= nums.length <= 50000
// -    1 <= nums[i] <= 10^5
// -    1 <= k <= nums.length
//

struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut res: i32 = 0;
        let mut count: i32 = 0;
        map.insert(0, 1);
        for i in nums.iter() {
            if i % 2 != 0 {
                count += 1;
            }
            if map.contains_key(&(count - k)) {
                res += map.get(&(count - k)).unwrap();
            }
            match map.get_mut(&count) {
                Some(v) => *v += 1,
                None => {
                    map.insert(count, 1);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 1, 2, 1, 1], 3, 2),
        (vec![2, 4, 6], 1, 0),
        (vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2, 16),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::number_of_subarrays(nums, k), expected);
    }
}
