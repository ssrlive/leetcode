#![allow(dead_code)]

// 457. Circular Array Loop
// https://leetcode.com/problems/circular-array-loop/
// https://leetcode.cn/problems/circular-array-loop/
//
// You are playing a game involving a circular array of non-zero integers nums. Each nums[i] denotes the number of indices forward/backward you must move if you are located at index i:
//
// If nums[i] is positive, move nums[i] steps forward, and
// If nums[i] is negative, move nums[i] steps backward.
// Since the array is circular, you may assume that moving forward from the last element puts you on the first element, and moving backwards from the first element puts you on the last element.
//
// A cycle in the array consists of a sequence of indices seq of length k where:
//
// Following the movement rules above results in the repeating index sequence seq[0] -> seq[1] -> ... -> seq[k - 1] -> seq[0] -> ...
// Every nums[seq[j]] is either all positive or all negative.
// k > 1
// Return true if there is a cycle in nums, or false otherwise.
//
//
// Example 1:
//
// Input: nums = [2,-1,1,2,2]
// Output: true
// Explanation: The graph shows how the indices are connected. White nodes are jumping forward, while red is jumping backward.
// We can see the cycle 0 --> 2 --> 3 --> 0 --> ..., and all of its nodes are white (jumping in the same direction).
//
// Example 2:
//
// Input: nums = [-1,-2,-3,-4,-5,6]
// Output: false
// Explanation: The graph shows how the indices are connected. White nodes are jumping forward, while red is jumping backward.
// The only cycle is of size 1, so we return false.
//
// Example 3:
//
// Input: nums = [1,-1,5,1,4]
// Output: true
// Explanation: The graph shows how the indices are connected. White nodes are jumping forward, while red is jumping backward.
// We can see the cycle 0 --> 1 --> 0 --> ..., and while it is of size > 1, it has a node jumping forward and a node jumping backward, so it is not a cycle.
// We can see the cycle 3 --> 4 --> 3 --> ..., and all of its nodes are white (jumping in the same direction).
//
// Constraints:
//
// - 1 <= nums.length <= 5000
// - -1000 <= nums[i] <= 1000
// - nums[i] != 0
//
// Follow up: Could you solve it in O(n) time complexity and O(1) extra space complexity?
//

struct Solution;

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        pub fn _circular_array_loop(nums: Vec<i64>) -> bool {
            let mut nums = nums;
            for i in 0..nums.len() {
                nums[i] %= nums.len() as i64;
            }
            for i in 0..nums.len() {
                if nums[i] % nums.len() as i64 <= 0 {
                    continue;
                }
                let mut index = i;
                while nums[index] % nums.len() as i64 > 0 {
                    let next_index = (index as i64 + nums[index]) as usize % nums.len();
                    nums[index] = (i + 1) as i64 * nums.len() as i64;
                    index = next_index;
                }
                if nums[index] == (i + 1) as i64 * nums.len() as i64 {
                    return true;
                }
            }
            for i in 0..nums.len() {
                if nums[i] % nums.len() as i64 >= 0 {
                    continue;
                }
                let mut index = i;
                let mut feet = std::collections::HashMap::new();
                while (nums[index] % nums.len() as i64) < 0 {
                    feet.insert(index, ());
                    let next_index = (index as i64 + nums[index] + nums.len() as i64) as usize % nums.len();
                    nums[index] = -((i + 1) as i64) * nums.len() as i64;
                    index = next_index;
                }
                if nums[index] == -((i + 1) as i64) * nums.len() as i64 {
                    return true;
                }
            }
            false
        }
        _circular_array_loop(nums.into_iter().map(|v| v as i64).collect())
    }
}

#[test]
fn test() {
    assert!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]));
    assert!(!Solution::circular_array_loop(vec![-1, -2, -3, -4, -5, 6]));
    assert!(Solution::circular_array_loop(vec![1, -1, 5, 1, 4]));
}
