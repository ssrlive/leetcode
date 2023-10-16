#![allow(dead_code)]

// 2784. Check if Array is Good
// https://leetcode.com/problems/check-if-array-is-good/
// https://leetcode.cn/problems/check-if-array-is-good/
//
// Easy
//
// You are given an integer array nums. We consider an array good if it is a permutation of an array base[n].
//
// base[n] = [1, 2, ..., n - 1, n, n] (in other words, it is an array of length n + 1 which contains 1 to n - 1 exactly once,
// plus two occurrences of n). For example, base[1] = [1, 1] and base[3] = [1, 2, 3, 3].
//
// Return true if the given array is good, otherwise return false.
//
// Note: A permutation of integers represents an arrangement of these numbers.
//
// Example 1:
//
// Input: nums = [2, 1, 3]
// Output: false
// Explanation: Since the maximum element of the array is 3, the only candidate n for which this array
// could be a permutation of base[n], is n = 3. However, base[3] has four elements but array nums has three.
// Therefore, it can not be a permutation of base[3] = [1, 2, 3, 3]. So the answer is false.
//
// Example 2:
//
// Input: nums = [1, 3, 3, 2]
// Output: true
// Explanation: Since the maximum element of the array is 3, the only candidate n for which this array
// could be a permutation of base[n], is n = 3. It can be seen that nums is a permutation of base[3] = [1, 2, 3, 3]
// (by swapping the second and fourth elements in nums, we reach base[3]). Therefore, the answer is true.
//
// Example 3:
//
// Input: nums = [1, 1]
// Output: true
// Explanation: Since the maximum element of the array is 1, the only candidate n for which this array
// could be a permutation of base[n], is n = 1. It can be seen that nums is a permutation of base[1] = [1, 1].
// Therefore, the answer is true.
//
// Example 4:
//
// Input: nums = [3, 4, 4, 1, 2, 1]
// Output: false
// Explanation: Since the maximum element of the array is 4, the only candidate n for which this array
// could be a permutation of base[n], is n = 4. However, base[4] has five elements but array nums has six.
// Therefore, it can not be a permutation of base[4] = [1, 2, 3, 4, 4]. So the answer is false.
//
// Constraints:
//
//     1 <= nums.length <= 100
//     1 <= num[i] <= 200
//

struct Solution;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = *nums.iter().max().unwrap();
        let mut freq_map = vec![0; (n + 1) as usize];
        for num in nums {
            freq_map[num as usize] += 1;
        }
        if freq_map[n as usize] != 2 {
            return false;
        }
        for i in 1..n {
            if freq_map[i as usize] != 1 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(!Solution::is_good(vec![2, 1, 3]));
    assert!(Solution::is_good(vec![1, 3, 3, 2]));
    assert!(Solution::is_good(vec![1, 1]));
    assert!(!Solution::is_good(vec![3, 4, 4, 1, 2, 1]));
}
