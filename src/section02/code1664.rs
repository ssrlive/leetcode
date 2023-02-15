#![allow(dead_code)]

/*

// 1664. Ways to Make a Fair Array
// https://leetcode.com/problems/ways-to-make-a-fair-array/
// https://leetcode.cn/problems/ways-to-make-a-fair-array/
//
// Medium
//
// You are given an integer array nums. You can choose exactly one index (0-indexed) and remove the element. Notice that the index of the elements may change after the removal.

For example, if nums = [6,1,7,4,1]:

    Choosing to remove index 1 results in nums = [6,7,4,1].
    Choosing to remove index 2 results in nums = [6,1,4,1].
    Choosing to remove index 4 results in nums = [6,1,7,4].

An array is fair if the sum of the odd-indexed values equals the sum of the even-indexed values.

Return the number of indices that you could choose such that after the removal, nums is fair.

Example 1:

Input: nums = [2,1,6,4]
Output: 1
Explanation:
Remove index 0: [1,6,4] -> Even sum: 1 + 4 = 5. Odd sum: 6. Not fair.
Remove index 1: [2,6,4] -> Even sum: 2 + 4 = 6. Odd sum: 6. Fair.
Remove index 2: [2,1,4] -> Even sum: 2 + 4 = 6. Odd sum: 1. Not fair.
Remove index 3: [2,1,6] -> Even sum: 2 + 6 = 8. Odd sum: 1. Not fair.
There is 1 index that you can remove to make nums fair.

Example 2:

Input: nums = [1,1,1]
Output: 3
Explanation: You can remove any index and the remaining array is fair.

Example 3:

Input: nums = [1,2,3]
Output: 0
Explanation: You cannot make a fair array after removing any index.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^4
*/

struct Solution;

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut odds = vec![0; n + 1];
        let mut evens = vec![0; n + 1];

        for i in 0..n {
            if i % 2 == 0 {
                odds[i + 1] = odds[i];
                evens[i + 1] = nums[i] + evens[i];
            } else {
                odds[i + 1] = nums[i] + odds[i];
                evens[i + 1] = evens[i];
            }
        }

        let mut result = 0;
        for i in 0..n {
            let ov = odds[i] - odds[0] + evens[n] - evens[i];
            let ev = evens[i] - evens[0] + odds[n] - odds[i];
            if i % 2 == 0 {
                if ov - nums[i] == ev {
                    result += 1;
                }
            } else if ov == ev - nums[i] {
                result += 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 1, 6, 4], 1),
        (vec![1, 1, 1], 3),
        (vec![1, 2, 3], 0),
        (vec![1, 1, 1, 1, 1], 5),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::ways_to_make_fair(nums), expected);
    }
}
