#![allow(dead_code)]

/*

// 1675. Minimize Deviation in Array
// https://leetcode.com/problems/minimize-deviation-in-array/
// https://leetcode.cn/problems/minimize-deviation-in-array/
//
// Hard
//
// You are given an array nums of n positive integers.

You can perform two types of operations on any element of the array any number of times:

    If the element is even, divide it by 2.
        For example, if the array is [1,2,3,4], then you can do this operation on the last element, and the array will be [1,2,3,2].
    If the element is odd, multiply it by 2.
        For example, if the array is [1,2,3,4], then you can do this operation on the first element, and the array will be [2,2,3,4].

The deviation of the array is the maximum difference between any two elements in the array.

Return the minimum deviation the array can have after performing some number of operations.

Example 1:

Input: nums = [1,2,3,4]
Output: 1
Explanation: You can transform the array to [1,2,3,2], then to [2,2,3,2], then the deviation will be 3 - 2 = 1.

Example 2:

Input: nums = [4,1,5,20,3]
Output: 3
Explanation: You can transform the array after two operations to [4,2,5,5,3], then the deviation will be 5 - 2 = 3.

Example 3:

Input: nums = [2,10,8]
Output: 3

Constraints:

    n == nums.length
    2 <= n <= 5 * 10^4
    1 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut min = i32::MAX;
        for num in nums.iter_mut() {
            if *num % 2 == 1 {
                *num *= 2;
            }
            min = min.min(*num);
        }
        let mut ans = i32::MAX;
        let mut heap = std::collections::BinaryHeap::new();
        for num in nums {
            heap.push(num);
        }
        while let Some(num) = heap.pop() {
            ans = ans.min(num - min);
            if num % 2 == 1 {
                break;
            }
            min = min.min(num / 2);
            heap.push(num / 2);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 3, 4], 1), (vec![4, 1, 5, 20, 3], 3), (vec![2, 10, 8], 3)];
    for (nums, expect) in cases {
        assert_eq!(Solution::minimum_deviation(nums), expect);
    }
}
