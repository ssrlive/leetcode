#![allow(dead_code)]

// 3524. Find X Value of Array I
// https://leetcode.com/problems/find-x-value-of-array-i/
// https://leetcode.cn/problems/find-x-value-of-array-i/
//
// Medium
//
// You are given an array of positive integers nums, and a positive integer k.
//
// You are allowed to perform an operation once on nums, where in each operation you can remove any non-overlapping prefix and suffix from nums such that nums remains non-empty.
//
// You need to find the x-value of nums, which is the number of ways to perform this operation so that the product of the remaining elements leaves a remainder of x when divided by k.
//
// Return an array result of size k where result[x] is the x-value of nums for 0 <= x <= k - 1.
//
// A prefix of an array is a that starts from the beginning of the array and extends to any point within it.
//
// A suffix of an array is a that starts at any point within the array and extends to the end of the array.
//
// Note that the prefix and suffix to be chosen for the operation can be empty.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5], k = 3
//
// Output: [9,2,4]
//
// Explanation:
//
//     For x = 0, the possible operations include all possible ways to remove non-overlapping prefix/suffix that do not remove nums[2] == 3.
//     For x = 1, the possible operations are:
//         Remove the empty prefix and the suffix [2, 3, 4, 5]. nums becomes [1].
//         Remove the prefix [1, 2, 3] and the suffix [5]. nums becomes [4].
//     For x = 2, the possible operations are:
//         Remove the empty prefix and the suffix [3, 4, 5]. nums becomes [1, 2].
//         Remove the prefix [1] and the suffix [3, 4, 5]. nums becomes [2].
//         Remove the prefix [1, 2, 3] and the empty suffix. nums becomes [4, 5].
//         Remove the prefix [1, 2, 3, 4] and the empty suffix. nums becomes [5].
//
// Example 2:
//
// Input: nums = [1,2,4,8,16,32], k = 4
//
// Output: [18,1,2,0]
//
// Explanation:
//
//     For x = 0, the only operations that do not result in x = 0 are:
//         Remove the empty prefix and the suffix [4, 8, 16, 32]. nums becomes [1, 2].
//         Remove the empty prefix and the suffix [2, 4, 8, 16, 32]. nums becomes [1].
//         Remove the prefix [1] and the suffix [4, 8, 16, 32]. nums becomes [2].
//     For x = 1, the only possible operation is:
//         Remove the empty prefix and the suffix [2, 4, 8, 16, 32]. nums becomes [1].
//     For x = 2, the possible operations are:
//         Remove the empty prefix and the suffix [4, 8, 16, 32]. nums becomes [1, 2].
//         Remove the prefix [1] and the suffix [4, 8, 16, 32]. nums becomes [2].
//     For x = 3, there is no possible way to perform the operation.
//
// Example 3:
//
// Input: nums = [1,1,2,1,1], k = 2
//
// Output: [9,6]
//
// Constraints:
//
//     1 <= nums[i] <= 10^9
//     1 <= nums.length <= 10^5
//     1 <= k <= 5
//

struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let mut res = vec![0; k as usize];
        let mut cnt = vec![0; k as usize];
        for &a in &nums {
            let mut cnt2 = vec![0; k as usize];
            for i in 0..k {
                let v = ((i as i64 * a as i64) % k as i64) as usize;
                cnt2[v] += cnt[i as usize];
                res[v] += cnt[i as usize];
            }
            cnt = cnt2;
            cnt[(a % k) as usize] += 1;
            res[(a % k) as usize] += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::result_array(vec![1, 2, 3, 4, 5], 3), vec![9, 2, 4]);
    assert_eq!(Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4), vec![18, 1, 2, 0]);
    assert_eq!(Solution::result_array(vec![1, 1, 2, 1, 1], 2), vec![9, 6]);
}
