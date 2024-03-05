#![allow(dead_code)]

// 3072. Distribute Elements Into Two Arrays II
// https://leetcode.com/problems/distribute-elements-into-two-arrays-ii/
// https://leetcode.cn/problems/distribute-elements-into-two-arrays-ii/
//
// Hard
//
// You are given a 1-indexed array of integers nums of length n.
//
// We define a function greaterCount such that greaterCount(arr, val) returns the number of elements in arr that are strictly greater than val.
//
// You need to distribute all the elements of nums between two arrays arr1 and arr2 using n operations.
// In the first operation, append nums[1] to arr1. In the second operation, append nums[2] to arr2. Afterwards, in the ith operation:
//
// If greaterCount(arr1, nums[i]) > greaterCount(arr2, nums[i]), append nums[i] to arr1.
// If greaterCount(arr1, nums[i]) < greaterCount(arr2, nums[i]), append nums[i] to arr2.
// If greaterCount(arr1, nums[i]) == greaterCount(arr2, nums[i]), append nums[i] to the array with a lesser number of elements.
// If there is still a tie, append nums[i] to arr1.
// The array result is formed by concatenating the arrays arr1 and arr2. For example, if arr1 == [1,2,3] and arr2 == [4,5,6], then result = [1,2,3,4,5,6].
//
// Return the integer array result.
//
// Example 1:
//
// Input: nums = [2,1,3,3]
// Output: [2,3,1,3]
// Explanation: After the first 2 operations, arr1 = [2] and arr2 = [1].
// In the 3rd operation, the number of elements greater than 3 is zero in both arrays. Also, the lengths are equal, hence, append nums[3] to arr1.
// In the 4th operation, the number of elements greater than 3 is zero in both arrays. As the length of arr2 is lesser, hence, append nums[4] to arr2.
// After 4 operations, arr1 = [2,3] and arr2 = [1,3].
// Hence, the array result formed by concatenation is [2,3,1,3].
//
// Example 2:
//
// Input: nums = [5,14,3,1,2]
// Output: [5,3,1,2,14]
// Explanation: After the first 2 operations, arr1 = [5] and arr2 = [14].
// In the 3rd operation, the number of elements greater than 3 is one in both arrays. Also, the lengths are equal, hence, append nums[3] to arr1.
// In the 4th operation, the number of elements greater than 1 is greater in arr1 than arr2 (2 > 1). Hence, append nums[4] to arr1.
// In the 5th operation, the number of elements greater than 2 is greater in arr1 than arr2 (2 > 1). Hence, append nums[5] to arr1.
// After 5 operations, arr1 = [5,3,1,2] and arr2 = [14].
// Hence, the array result formed by concatenation is [5,3,1,2,14].
//
// Example 3:
//
// Input: nums = [3,3,3,3]
// Output: [3,3,3,3]
// Explanation: At the end of 4 operations, arr1 = [3,3] and arr2 = [3,3].
// Hence, the array result formed by concatenation is [3,3,3,3].
//
// Constraints:
//
// 3 <= n <= 10^5
// 1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left = vec![nums[0]];
        let mut right = vec![nums[1]];
        let mut ll = vec![nums[0]];
        let mut rr = vec![nums[1]];
        let mut l = 0;
        let mut r = 0;

        for &nums_i in nums.iter().take(n).skip(2) {
            let x = Self::greater_count(&left, nums_i);
            let y = Self::greater_count(&right, nums_i);

            match y.cmp(&x) {
                std::cmp::Ordering::Greater => {
                    right.insert(right.len() - y, nums_i);
                    rr.push(nums_i);
                    r += 1;
                }
                std::cmp::Ordering::Less => {
                    left.insert(left.len() - x, nums_i);
                    ll.push(nums_i);
                    l += 1;
                }
                std::cmp::Ordering::Equal => {
                    if l <= r {
                        left.insert(left.len() - x, nums_i);
                        ll.push(nums_i);
                        l += 1;
                    } else {
                        right.insert(right.len() - y, nums_i);
                        rr.push(nums_i);
                        r += 1;
                    }
                }
            }
        }
        ll.append(&mut rr);
        ll
    }

    fn greater_count(arr: &[i32], val: i32) -> usize {
        let index = arr.partition_point(|&x| x <= val);
        arr.len() - index
    }
}

#[test]
fn test() {
    assert_eq!(Solution::result_array(vec![2, 1, 3, 3]), vec![2, 3, 1, 3]);
    assert_eq!(Solution::result_array(vec![5, 14, 3, 1, 2]), vec![5, 3, 1, 2, 14]);
    assert_eq!(Solution::result_array(vec![3, 3, 3, 3]), vec![3, 3, 3, 3]);
}
