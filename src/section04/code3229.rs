#![allow(dead_code)]

// 3229. Minimum Operations to Make Array Equal to Target
// https://leetcode.com/problems/minimum-operations-to-make-array-equal-to-target/
// https://leetcode.cn/problems/minimum-operations-to-make-array-equal-to-target/
//
// Hard
//
// You are given two positive integer arrays nums and target, of the same length.
//
// In a single operation, you can select any subarray of nums and increment or decrement each element within that subarray by 1.
//
// Return the minimum number of operations required to make nums equal to the array target.
//
// Example 1:
//
// Input: nums = [3,5,1,2], target = [4,6,2,4]
//
// Output: 2
//
// Explanation:
//
// We will perform the following operations to make nums equal to target:
// - Increment nums[0..3] by 1, nums = [4,6,2,3].
// - Increment nums[3..3] by 1, nums = [4,6,2,4].
//
// Example 2:
//
// Input: nums = [1,3,2], target = [2,1,4]
//
// Output: 5
//
// Explanation:
//
// We will perform the following operations to make nums equal to target:
// - Increment nums[0..0] by 1, nums = [2,3,2].
// - Decrement nums[1..1] by 1, nums = [2,2,2].
// - Decrement nums[1..1] by 1, nums = [2,1,2].
// - Increment nums[2..2] by 1, nums = [2,1,3].
// - Increment nums[2..2] by 1, nums = [2,1,4].
//
// Constraints:
//
//     1 <= nums.length == target.length <= 10^5
//     1 <= nums[i], target[i] <= 10^8
//

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<i64>>();
        let target = target.iter().map(|&x| x as i64).collect::<Vec<i64>>();
        let n = nums.len();
        let mut incr = 0;
        let mut decr = 0;
        let mut ops = 0_i64;

        for i in 0..n {
            let diff = target[i] - nums[i];
            match diff.cmp(&0) {
                std::cmp::Ordering::Greater => {
                    if incr < diff {
                        ops += diff - incr;
                    }
                    incr = diff;
                    decr = 0;
                }
                std::cmp::Ordering::Less => {
                    if diff < decr {
                        ops += decr - diff;
                    }
                    decr = diff;
                    incr = 0;
                }
                std::cmp::Ordering::Equal => {
                    incr = 0;
                    decr = 0;
                }
            }
        }

        ops
    }
}

#[test]
fn test() {
    let nums = vec![3, 5, 1, 2];
    let target = vec![4, 6, 2, 4];
    let output = 2;
    assert_eq!(Solution::minimum_operations(nums, target), output);

    let nums = vec![1, 3, 2];
    let target = vec![2, 1, 4];
    let output = 5;
    assert_eq!(Solution::minimum_operations(nums, target), output);
}
