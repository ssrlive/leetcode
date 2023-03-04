#![allow(dead_code)]

/*

// 2134. Minimum Swaps to Group All 1's Together II
// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/
// https://leetcode.cn/problems/minimum-swaps-to-group-all-1s-together-ii/
//
// Medium
//
// A swap is defined as taking two distinct positions in an array and swapping the values in them.

A circular array is defined as an array where we consider the first element and the last element to be adjacent.

Given a binary circular array nums, return the minimum number of swaps required to group all 1's present in the array together at any location.

Example 1:

Input: nums = [0,1,0,1,1,0,0]
Output: 1
Explanation: Here are a few of the ways to group all the 1's together:
[0,0,1,1,1,0,0] using 1 swap.
[0,1,1,1,0,0,0] using 1 swap.
[1,1,0,0,0,0,1] using 2 swaps (using the circular property of the array).
There is no way to group all 1's together with 0 swaps.
Thus, the minimum number of swaps required is 1.

Example 2:

Input: nums = [0,1,1,1,0,0,1,1,0]
Output: 2
Explanation: Here are a few of the ways to group all the 1's together:
[1,1,1,0,0,0,0,1,1] using 2 swaps (using the circular property of the array).
[1,1,1,1,1,0,0,0,0] using 2 swaps.
There is no way to group all 1's together with 0 or 1 swaps.
Thus, the minimum number of swaps required is 2.

Example 3:

Input: nums = [1,1,0,0,1]
Output: 0
Explanation: All the 1's are already grouped together due to the circular property of the array.
Thus, the minimum number of swaps required is 0.

Constraints:

    1 <= nums.length <= 10^5
    nums[i] is either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let mut count = 0usize;
        for &num in nums.iter().take(n) {
            count += num as usize;
        }
        for i in 0..n {
            nums.push(nums[i]);
        }
        let mut min = n as i32;
        let mut zero = 0;
        for &num in nums.iter().take(count) {
            if num == 0 {
                zero += 1;
            }
        }
        min = min.min(zero);
        for i in count..nums.len() {
            if nums[i - count] == 0 {
                zero -= 1;
            }
            if nums[i] == 0 {
                zero += 1;
            }
            min = min.min(zero);
        }
        min
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
    assert_eq!(Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
    assert_eq!(Solution::min_swaps(vec![1, 1, 0, 0, 1]), 0);
}
