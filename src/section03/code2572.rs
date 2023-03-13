#![allow(dead_code)]

/*

// 2572. Count the Number of Square-Free Subsets
// https://leetcode.com/problems/count-the-number-of-square-free-subsets/
// https://leetcode.cn/problems/count-the-number-of-square-free-subsets/
//
// Medium
//
// You are given a positive integer 0-indexed array nums.

A subset of the array nums is square-free if the product of its elements is a square-free integer.

A square-free integer is an integer that is divisible by no square number other than 1.

Return the number of square-free non-empty subsets of the array nums. Since the answer may be too large, return it modulo 109 + 7.

A non-empty subset of nums is an array that can be obtained by deleting some (possibly none but not all) elements from nums. Two subsets are different if and only if the chosen indices to delete are different.

Example 1:

Input: nums = [3,4,4,5]
Output: 3
Explanation: There are 3 square-free subsets in this example:
- The subset consisting of the 0th element [3]. The product of its elements is 3, which is a square-free integer.
- The subset consisting of the 3rd element [5]. The product of its elements is 5, which is a square-free integer.
- The subset consisting of 0th and 3rd elements [3,5]. The product of its elements is 15, which is a square-free integer.
It can be proven that there are no more than 3 square-free subsets in the given array.

Example 2:

Input: nums = [1]
Output: 1
Explanation: There is 1 square-free subset in this example:
- The subset consisting of the 0th element [1]. The product of its elements is 1, which is a square-free integer.
It can be proven that there is no more than 1 square-free subset in the given array.

Constraints:

    1 <= nums.length <= 1000
    1 <= nums[i] <= 30
*/

struct Solution;

impl Solution {
    pub fn square_free_subsets(nums: Vec<i32>) -> i32 {
        fn dp(nums: &Vec<i64>, i: usize, mask: i64, memo: &mut Vec<Vec<i64>>) -> i64 {
            let mod_num = 1e9 as i64 + 7;
            let vec = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
            if i == nums.len() {
                return 1;
            }
            if memo[i][mask as usize] != -1 {
                return memo[i][mask as usize];
            }
            let mut temp = mask;
            let mut flag = 1;
            for (j, &item_j) in vec.iter().enumerate() {
                let mut num = nums[i];
                let mut count = 0;
                while num % item_j == 0 {
                    num /= item_j;
                    count += 1;
                }
                if count >= 2 || ((count == 1) && ((mask >> j) & 1) == 1) {
                    flag = 0;
                    break;
                }
                if count > 0 {
                    temp |= 1 << j;
                }
            }
            if flag == 1 {
                memo[i][mask as usize] = (dp(nums, i + 1, mask, memo) + dp(nums, i + 1, temp, memo)) % mod_num;
            } else {
                memo[i][mask as usize] = dp(nums, i + 1, mask, memo);
            }
            memo[i][mask as usize]
        }

        let nums = nums.iter().map(|&x| x as i64).collect();
        let mut memo = vec![vec![-1; 1024]; 1001];
        (dp(&nums, 0, 0, &mut memo) - 1) as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::square_free_subsets(vec![3, 4, 4, 5]), 3);
    assert_eq!(Solution::square_free_subsets(vec![1]), 1);
}
