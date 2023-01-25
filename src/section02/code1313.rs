#![allow(dead_code)]

// 1313. Decompress Run-Length Encoded List
// https://leetcode.com/problems/decompress-run-length-encoded-list/
// https://leetcode.cn/problems/decompress-run-length-encoded-list/
//
// Easy
//
// We are given a list nums of integers representing a list compressed with run-length encoding.
//
// Consider each adjacent pair of elements [freq, val] = [nums[2*i], nums[2*i+1]] (with i >= 0).
//   For each such pair, there are freq elements with value val concatenated in a sublist.
//   Concatenate all the sublists from left to right to generate the decompressed list.
//
// Return the decompressed list.
//
// Example 1:
//
// Input: nums = [1,2,3,4]
// Output: [2,4,4,4]
// Explanation: The first pair [1,2] means we have freq = 1 and val = 2 so we generate the array [2].
// The second pair [3,4] means we have freq = 3 and val = 4 so we generate [4,4,4].
// At the end the concatenation [2] + [4,4,4] is [2,4,4,4].
//
// Example 2:
//
// Input: nums = [1,1,2,3]
// Output: [1,3,3]
//
// Constraints:
//
// -    2 <= nums.length <= 100
// -    nums.length % 2 == 0
// -    1 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(nums.len());
        for values in nums.windows(2).step_by(2) {
            res.extend(vec![values[1]; values[0] as usize]);
        }
        res
    }

    pub fn decompress_rl_elist_2(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            let freq = nums[i];
            let val = nums[i + 1];
            for _ in 0..freq {
                result.push(val);
            }
            i += 2;
        }
        result
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let result = vec![2, 4, 4, 4];
    assert_eq!(Solution::decompress_rl_elist(nums), result);

    let nums = vec![1, 1, 2, 3];
    let result = vec![1, 3, 3];
    assert_eq!(Solution::decompress_rl_elist(nums), result);
}
