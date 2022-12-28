#![allow(dead_code)]

// 2449. Minimum Number of Operations to Make Arrays Similar
// https://leetcode.com/problems/minimum-number-of-operations-to-make-arrays-similar/
// https://leetcode.cn/problems/minimum-number-of-operations-to-make-arrays-similar/
//
// You are given two positive integer arrays nums and target, of the same length.
//
// In one operation, you can choose any two distinct indices i and j where 0 <= i, j < nums.length and:
//
// set nums[i] = nums[i] + 2 and
// set nums[j] = nums[j] - 2.
// Two arrays are considered to be similar if the frequency of each element is the same.
//
// Return the minimum number of operations required to make nums similar to target. The test cases are generated such that nums can always be similar to target.
//
// Example 1:
//
// Input: nums = [8,12,6], target = [2,14,10]
// Output: 2
// Explanation: It is possible to make nums similar to target in two operations:
// - Choose i = 0 and j = 2, nums = [10,12,4].
// - Choose i = 1 and j = 2, nums = [10,14,2].
// It can be shown that 2 is the minimum number of operations needed.
//
// Example 2:
//
// Input: nums = [1,2,5], target = [4,1,3]
// Output: 1
// Explanation: We can make nums similar to target in one operation:
// - Choose i = 1 and j = 2, nums = [1,4,3].
//
// Example 3:
//
// Input: nums = [1,1,1,1,1], target = [1,1,1,1,1]
// Output: 0
// Explanation: The array nums is already similiar to target.
//
// Constraints:
//
// - n == nums.length == target.length
// - 1 <= n <= 10^5
// - 1 <= nums[i], target[i] <= 10^6
// - It is possible to make nums similar to target.
//

struct Solution;

impl Solution {
    pub fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let (mut v1_even, mut v1_odd) = (vec![], vec![]);
        let (mut v2_even, mut v2_odd) = (vec![], vec![]);

        for a in nums {
            if a % 2 == 0 {
                v1_even.push(a);
            } else {
                v1_odd.push(a);
            }
        }
        for a in target {
            if a % 2 == 0 {
                v2_even.push(a);
            } else {
                v2_odd.push(a);
            }
        }

        v1_even.sort();
        v1_odd.sort();
        v2_even.sort();
        v2_odd.sort();

        let mut ret = 0;

        while !v1_even.is_empty() {
            let b = v1_even[v1_even.len() - 1] - v2_even[v2_even.len() - 1];
            if b > 0 {
                ret += b as i64 / 2;
            }
            v1_even.pop();
            v2_even.pop();
        }
        while !v1_odd.is_empty() {
            let b = v1_odd[v1_odd.len() - 1] - v2_odd[v2_odd.len() - 1];
            if b > 0 {
                ret += b as i64 / 2;
            }
            v1_odd.pop();
            v2_odd.pop();
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::make_similar(vec![8, 12, 6], vec![2, 14, 10]), 2);
    assert_eq!(Solution::make_similar(vec![1, 2, 5], vec![4, 1, 3]), 1);
    assert_eq!(Solution::make_similar(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]), 0);
}
