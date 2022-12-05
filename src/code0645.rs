#![allow(dead_code)]

// 645. Set Mismatch
// https://leetcode.com/problems/set-mismatch/
//
// You have a set of integers s, which originally contains all the numbers from 1 to n.
// Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set,
// which results in repetition of one number and loss of another number.
//
// You are given an integer array nums representing the data status of this set after the error.
//
// Find the number that occurs twice and the number that is missing and return them in the form of an array.
//
// Example 1:
//
// Input: nums = [1,2,2,4]
// Output: [2,3]
//
// Example 2:
//
// Input: nums = [1,1]
// Output: [1,2]
//
// Constraints:
//
// - 2 <= nums.length <= 10^4
// - 1 <= nums[i] <= 10^4
//
// Follow up: Could you implement a solution using only O(n) extra space complexity and O(1) runtime complexity?
//

struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut i = 0;
        while i < nums.len() {
            let j = nums[i] as usize - 1;
            if nums[i] != nums[j] {
                nums.swap(i, j);
            } else {
                i += 1;
            }
        }
        let mut res = vec![0; 2];
        for (i, &item) in nums.iter().enumerate() {
            if item != (i + 1) as i32 {
                res[0] = item;
                res[1] = (i + 1) as i32;
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
}
