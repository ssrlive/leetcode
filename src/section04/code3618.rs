#![allow(dead_code)]

// 3618. Split Array by Prime Indices
// https://leetcode.com/problems/split-array-by-prime-indices/
// https://leetcode.cn/problems/split-array-by-prime-indices/
//
// Medium
//
// You are given an integer array nums.
//
// Split nums into two arrays A and B using the following rule:
//
// Elements at prime indices in nums must go into array A.
// All other elements must go into array B.
// Return the absolute difference between the sums of the two arrays: |sum(A) - sum(B)|.
//
// Note: An empty array has a sum of 0.
//
// Example 1:
//
// Input: nums = [2,3,4]
//
// Output: 1
//
// Explanation:
//
// The only prime index in the array is 2, so nums[2] = 4 is placed in array A.
// The remaining elements, nums[0] = 2 and nums[1] = 3 are placed in array B.
// sum(A) = 4, sum(B) = 2 + 3 = 5.
// The absolute difference is |4 - 5| = 1.
//
// Example 2:
//
// Input: nums = [-1,5,7,0]
//
// Output: 3
//
// Explanation:
//
// The prime indices in the array are 2 and 3, so nums[2] = 7 and nums[3] = 0 are placed in array A.
// The remaining elements, nums[0] = -1 and nums[1] = 5 are placed in array B.
// sum(A) = 7 + 0 = 7, sum(B) = -1 + 5 = 4.
// The absolute difference is |7 - 4| = 3.
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// -10^9 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        fn is_prime(n: i64) -> bool {
            if n < 2 {
                return false;
            }
            if n == 2 {
                return true;
            }
            if n % 2 == 0 {
                return false;
            }
            let mut i = 3;
            while i * i <= n {
                if n % i == 0 {
                    return false;
                }
                i += 2;
            }
            true
        }

        let (mut sum_a, mut sum_b) = (0, 0);
        for (i, &val) in nums.iter().enumerate() {
            if is_prime(i as i64) {
                sum_a += val as i64;
            } else {
                sum_b += val as i64;
            }
        }
        (sum_a - sum_b).abs()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::split_array(vec![2, 3, 4]), 1);
    assert_eq!(Solution::split_array(vec![-1, 5, 7, 0]), 3);
}
