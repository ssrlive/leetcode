#![allow(dead_code)]

/*

// 1787. Make the XOR of All Segments Equal to Zero
Hard
334
19
Companies

You are given an array nums​​​ and an integer k​​​​​. The XOR of a segment [left, right] where left <= right is the XOR of all the elements with indices between left and right, inclusive: nums[left] XOR nums[left+1] XOR ... XOR nums[right].

Return the minimum number of elements to change in the array such that the XOR of all segments of size k​​​​​​ is equal to zero.

Example 1:

Input: nums = [1,2,0,3,0], k = 1
Output: 3
Explanation: Modify the array from [1,2,0,3,0] to from [0,0,0,0,0].

Example 2:

Input: nums = [3,4,5,2,1,7,3,4,7], k = 3
Output: 3
Explanation: Modify the array from [3,4,5,2,1,7,3,4,7] to [3,4,7,3,4,7,3,4,7].

Example 3:

Input: nums = [1,2,4,1,2,5,1,2,6], k = 3
Output: 3
Explanation: Modify the array from [1,2,4,1,2,5,1,2,6] to [1,2,3,1,2,3,1,2,3].

Constraints:

    1 <= k <= nums.length <= 2000
    ​​​​​​0 <= nums[i] < 2^10
*/

struct Solution;

impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut dp = vec![vec![n + 1; 1025]; k];
        let mut freq = vec![vec![0; 1025]; k];
        let mut numbers = vec![vec![]; k];
        for i in 0..n {
            freq[i % k][nums[i] as usize] += 1;
            numbers[i % k].push(nums[i]);
        }
        let mut mnm = 0;
        for i in 0..k {
            let total = numbers[i].len();
            for j in 0..1024 {
                if i == 0 {
                    dp[i][j] = total - freq[i][j];
                } else {
                    for it in &numbers[i] {
                        dp[i][j] = dp[i][j].min(dp[i - 1][j ^ *it as usize] + total - freq[i][*it as usize]);
                    }
                    dp[i][j] = dp[i][j].min(mnm + total);
                }
            }
            mnm = *dp[i].iter().min().unwrap();
        }
        dp[k - 1][0] as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 0, 3, 0], 1, 3),
        (vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3, 3),
        (vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3, 3),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::min_changes(nums, k), expected);
    }
}
