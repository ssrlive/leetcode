#![allow(dead_code)]

// 3507. Minimum Pair Removal to Sort Array I
// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-i
// https://leetcode.cn/problems/minimum-pair-removal-to-sort-array-i
//
// Easy
//
// Given an array nums, you can perform the following operation any number of times:
//
//     Select the adjacent pair with the minimum sum in nums. If multiple such pairs exist, choose the leftmost one.
//     Replace the pair with their sum.
//
// Return the minimum number of operations needed to make the array non-decreasing.
//
// An array is said to be non-decreasing if each element is greater than or equal to its previous element (if it exists).
//
// Example 1:
//
// Input: nums = [5,2,3,1]
//
// Output: 2
//
// Explanation:
//
//     The pair (3,1) has the minimum sum of 4. After replacement, nums = [5,2,4].
//     The pair (2,4) has the minimum sum of 6. After replacement, nums = [5,6].
//
// The array nums became non-decreasing in two operations.
//
// Example 2:
//
// Input: nums = [1,2,2]
//
// Output: 0
//
// Explanation:
//
// The array nums is already sorted.
//
// Constraints:
//
//     1 <= nums.length <= 50
//     -1000 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        while nums.len() > 1 {
            if nums.windows(2).all(|w| w[0] <= w[1]) {
                break;
            };
            let sums = nums.windows(2).map(|w| w[0] + w[1]).enumerate();
            let (minn, minx) = sums.clone().min_by_key(|(_, x)| *x).unwrap();
            nums = nums
                .iter()
                .enumerate()
                .filter(|&(i, _x)| i != minn)
                .map(|(i, &x)| if i == minn + 1 { minx } else { x })
                .collect::<Vec<i32>>();
            ans += 1;
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
    assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
    assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 3]), 0);
    assert_eq!(Solution::minimum_pair_removal(vec![3, 2, 1]), 1);
}
