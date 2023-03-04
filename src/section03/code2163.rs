#![allow(dead_code)]

/*

// 2163. Minimum Difference in Sums After Removal of Elements
// https://leetcode.com/problems/minimum-difference-in-sums-after-removal-of-elements/
// https://leetcode.cn/problems/minimum-difference-in-sums-after-removal-of-elements/
//
// Hard
//
You are given a 0-indexed integer array nums consisting of 3 * n elements.

You are allowed to remove any subsequence of elements of size exactly n from nums. The remaining 2 * n elements will be divided into two equal parts:

    The first n elements belonging to the first part and their sum is sumfirst.
    The next n elements belonging to the second part and their sum is sumsecond.

The difference in sums of the two parts is denoted as sumfirst - sumsecond.

    For example, if sumfirst = 3 and sumsecond = 2, their difference is 1.
    Similarly, if sumfirst = 2 and sumsecond = 3, their difference is -1.

Return the minimum difference possible between the sums of the two parts after the removal of n elements.

Example 1:

Input: nums = [3,1,2]
Output: -1
Explanation: Here, nums has 3 elements, so n = 1.
Thus we have to remove 1 element from nums and divide the array into two equal parts.
- If we remove nums[0] = 3, the array will be [1,2]. The difference in sums of the two parts will be 1 - 2 = -1.
- If we remove nums[1] = 1, the array will be [3,2]. The difference in sums of the two parts will be 3 - 2 = 1.
- If we remove nums[2] = 2, the array will be [3,1]. The difference in sums of the two parts will be 3 - 1 = 2.
The minimum difference between sums of the two parts is min(-1,1,2) = -1.

Example 2:

Input: nums = [7,9,5,8,1,3]
Output: 1
Explanation: Here n = 2. So we must remove 2 elements and divide the remaining array into two parts containing two elements each.
If we remove nums[2] = 5 and nums[3] = 8, the resultant array will be [7,9,1,3]. The difference in sums will be (7+9) - (1+3) = 12.
To obtain the minimum difference, we should remove nums[1] = 9 and nums[4] = 1. The resultant array becomes [7,5,8,3]. The difference in sums of the two parts is (7+5) - (8+3) = 1.
It can be shown that it is not possible to obtain a difference smaller than 1.

Constraints:

    nums.length == 3 * n
    1 <= n <= 10^5
    1 <= nums[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let n = nums.len() as i64;
        let mut l = vec![0; n as usize];
        let mut r = vec![0; n as usize];
        let mut pq1 = BinaryHeap::new();
        let mut pq2 = BinaryHeap::<Reverse<i64>>::new();
        let mut sum = 0;
        for i in 0..n / 3 * 2 {
            sum += nums[i as usize];
            pq1.push(nums[i as usize]);
            if i >= n / 3 {
                sum -= pq1.pop().unwrap();
            }
            l[i as usize] = sum;
        }
        sum = 0;
        for i in (n / 3..n).rev() {
            sum += nums[i as usize];
            pq2.push(Reverse(nums[i as usize]));
            if i < n / 3 * 2 {
                sum -= pq2.pop().unwrap().0;
            }
            r[i as usize] = sum;
        }
        let mut ans = std::i64::MAX;
        for i in (n / 3 - 1..n / 3 * 2).rev() {
            ans = ans.min(l[i as usize] - r[i as usize + 1]);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![(vec![3, 1, 2], -1), (vec![7, 9, 5, 8, 1, 3], 1)];
    for (nums, expected) in cases {
        assert_eq!(Solution::minimum_difference(nums), expected);
    }
}
