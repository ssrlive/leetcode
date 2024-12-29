#![allow(dead_code)]

// 3404. Count Special Subsequences
// https://leetcode.com/problems/count-special-subsequences/
// https://leetcode.cn/problems/count-special-subsequences/
//
// Medium
//
// You are given an array nums consisting of positive integers.
//
// A special subsequence is defined as a subsequence of length 4, represented by indices (p, q, r, s),
// where p < q < r < s. This subsequence must satisfy the following conditions:
//
// nums[p] * nums[r] == nums[q] * nums[s]
// There must be at least one element between each pair of indices. In other words, q - p > 1, r - q > 1 and s - r > 1.
// A subsequence is a sequence derived from the array by deleting zero or more elements
// without changing the order of the remaining elements.
//
// Return the number of different special subsequences in nums.
//
// Example 1:
//
// Input: nums = [1,2,3,4,3,6,1]
//
// Output: 1
//
// Explanation:
//
// There is one special subsequence in nums.
//
// (p, q, r, s) = (0, 2, 4, 6):
// This corresponds to elements (1, 3, 3, 1).
// nums[p] * nums[r] = nums[0] * nums[4] = 1 * 3 = 3
// nums[q] * nums[s] = nums[2] * nums[6] = 3 * 1 = 3
//
// Example 2:
//
// Input: nums = [3,4,3,4,3,4,3,4]
//
// Output: 3
//
// Explanation:
//
// There are three special subsequences in nums.
//
// (p, q, r, s) = (0, 2, 4, 6):
// This corresponds to elements (3, 3, 3, 3).
// nums[p] * nums[r] = nums[0] * nums[4] = 3 * 3 = 9
// nums[q] * nums[s] = nums[2] * nums[6] = 3 * 3 = 9
// (p, q, r, s) = (1, 3, 5, 7):
// This corresponds to elements (4, 4, 4, 4).
// nums[p] * nums[r] = nums[1] * nums[5] = 4 * 4 = 16
// nums[q] * nums[s] = nums[3] * nums[7] = 4 * 4 = 16
// (p, q, r, s) = (0, 2, 5, 7):
// This corresponds to elements (3, 3, 4, 4).
// nums[p] * nums[r] = nums[0] * nums[5] = 3 * 4 = 12
// nums[q] * nums[s] = nums[2] * nums[7] = 3 * 4 = 12
//
// Constraints:
//
// 7 <= nums.length <= 1000
// 1 <= nums[i] <= 1000
//

struct Solution;

#[derive(Debug, Clone, Copy)]
pub struct SortableF64(pub f64);

impl std::hash::Hash for SortableF64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialEq for SortableF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for SortableF64 {}

impl Solution {
    pub fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut res = 0;
        let mut cnt = std::collections::HashMap::new();
        for r in 3..n - 2 {
            let q = r - 2;
            for p in 0..q - 1 {
                *cnt.entry(SortableF64(1.0 * nums[p] as f64 / nums[q] as f64)).or_insert(0) += 1;
            }
            for s in r + 2..n {
                res += *cnt.get(&SortableF64(1.0 * nums[s] as f64 / nums[r] as f64)).unwrap_or(&0);
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_subsequences(vec![1, 2, 3, 4, 3, 6, 1]), 1);
    assert_eq!(Solution::number_of_subsequences(vec![3, 4, 3, 4, 3, 4, 3, 4]), 3);
}
