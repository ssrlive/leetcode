#![allow(dead_code)]

/*

// 2170. Minimum Operations to Make the Array Alternating
// https://leetcode.com/problems/minimum-operations-to-make-the-array-alternating/
// https://leetcode.cn/problems/minimum-operations-to-make-the-array-alternating/
//
// Medium
//
// You are given a 0-indexed array nums consisting of n positive integers.

The array nums is called alternating if:

    nums[i - 2] == nums[i], where 2 <= i <= n - 1.
    nums[i - 1] != nums[i], where 1 <= i <= n - 1.

In one operation, you can choose an index i and change nums[i] into any positive integer.

Return the minimum number of operations required to make the array alternating.

Example 1:

Input: nums = [3,1,3,2,4,3]
Output: 3
Explanation:
One way to make the array alternating is by converting it to [3,1,3,1,3,1].
The number of operations required in this case is 3.
It can be proven that it is not possible to make the array alternating in less than 3 operations.

Example 2:

Input: nums = [1,2,2,2,2]
Output: 2
Explanation:
One way to make the array alternating is by converting it to [1,2,1,2,1].
The number of operations required in this case is 2.
Note that the array cannot be converted to [2,2,2,2,2] because in this case nums[0] == nums[1] which violates the conditions of an alternating array.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        use std::collections::*;
        let n = nums.len();
        let mut odds = HashMap::new();
        let mut odd_num = 0;
        let mut evens = HashMap::new();
        let mut even_num = 0;
        for (i, &num) in nums.iter().enumerate() {
            let v = num;
            if i.is_multiple_of(2) {
                odd_num += 1;
                *odds.entry(v).or_insert(0) += 1;
            } else {
                even_num += 1;
                *evens.entry(v).or_insert(0) += 1;
            }
        }
        odds.insert(10i32.pow(6), 0);
        evens.insert(10i32.pow(7), 0);
        evens.insert(10i32.pow(7) + 1, 0);

        let mut a = evens.into_iter().map(|v| (v.1, v.0)).collect::<Vec<(i32, i32)>>();
        a.sort();
        let top = a.pop().unwrap();
        let second = a.pop().unwrap();

        let mut result = n as i32;
        for (v1, num1) in odds {
            let mut temp = odd_num - num1;
            if top.1 != v1 {
                temp += even_num - top.0;
            } else {
                temp += even_num - second.0;
            }
            result = result.min(temp);
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_operations(vec![3, 1, 3, 2, 4, 3]), 3);
    assert_eq!(Solution::minimum_operations(vec![1, 2, 2, 2, 2]), 2);
    assert_eq!(Solution::minimum_operations(vec![1, 1, 1, 1, 1]), 2);
    assert_eq!(Solution::minimum_operations(vec![1, 1, 1, 1, 2]), 2);
}
