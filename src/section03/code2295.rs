#![allow(dead_code)]

/*

// 2295. Replace Elements in an Array
// https://leetcode.com/problems/replace-elements-in-an-array/
// https://leetcode.cn/problems/replace-elements-in-an-array/
//
// Medium
//
// You are given a 0-indexed array nums that consists of n distinct positive integers.
// Apply m operations to this array, where in the ith operation you
// replace the number operations[i][0] with operations[i][1].

It is guaranteed that in the ith operation:

    operations[i][0] exists in nums.
    operations[i][1] does not exist in nums.

Return the array obtained after applying all the operations.

Example 1:

Input: nums = [1,2,4,6], operations = [[1,3],[4,7],[6,1]]
Output: [3,2,7,1]
Explanation: We perform the following operations on nums:
- Replace the number 1 with 3. nums becomes [3,2,4,6].
- Replace the number 4 with 7. nums becomes [3,2,7,6].
- Replace the number 6 with 1. nums becomes [3,2,7,1].
We return the final array [3,2,7,1].

Example 2:

Input: nums = [1,2], operations = [[1,3],[2,1],[3,2]]
Output: [2,1]
Explanation: We perform the following operations to nums:
- Replace the number 1 with 3. nums becomes [3,2].
- Replace the number 2 with 1. nums becomes [3,1].
- Replace the number 3 with 2. nums becomes [2,1].
We return the array [2,1].

Constraints:

    n == nums.length
    m == operations.length
    1 <= n, m <= 10^5
    All the values of nums are distinct.
    operations[i].length == 2
    1 <= nums[i], operations[i][0], operations[i][1] <= 10^6
    operations[i][0] will exist in nums when applying the ith operation.
    operations[i][1] will not exist in nums when applying the ith operation.
*/

struct Solution;

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::*;
        let mut nums = nums;
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            map.entry(num).or_insert(vec![]).push(i);
        }
        for arr in operations {
            let v1 = arr[0];
            let v2 = arr[1];
            let arr = map.remove(&v1);
            if arr.is_none() {
                continue;
            }
            let arr = arr.unwrap();
            if let Some(base) = map.get_mut(&v2) {
                for v in arr {
                    base.push(v);
                }
            } else {
                map.insert(v2, arr);
            }
        }
        for (val, arr) in map {
            for i in arr {
                nums[i] = val;
            }
        }
        nums
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![1, 2, 4, 6],
            vec![vec![1, 3], vec![4, 7], vec![6, 1]],
            vec![3, 2, 7, 1],
        ),
        (vec![1, 2], vec![vec![1, 3], vec![2, 1], vec![3, 2]], vec![2, 1]),
    ];
    for (nums, operations, expect) in cases {
        assert_eq!(Solution::array_change(nums, operations), expect);
    }
}
