#![allow(dead_code)]

/*

// 2035. Partition Array Into Two Arrays to Minimize Sum Difference
// https://leetcode.com/problems/partition-array-into-two-arrays-to-minimize-sum-difference/
// https://leetcode.cn/problems/partition-array-into-two-arrays-to-minimize-sum-difference/
//
// Hard
//
// You are given an integer array nums of 2 * n integers. You need to partition nums into two arrays of length n to minimize the absolute difference of the sums of the arrays. To partition nums, put each element of nums into one of the two arrays.

Return the minimum possible absolute difference.

Example 1:
example-1

Input: nums = [3,9,7,3]
Output: 2
Explanation: One optimal partition is: [3,9] and [7,3].
The absolute difference between the sums of the arrays is abs((3 + 9) - (7 + 3)) = 2.

Example 2:

Input: nums = [-36,36]
Output: 72
Explanation: One optimal partition is: [-36] and [36].
The absolute difference between the sums of the arrays is abs((-36) - (36)) = 72.

Example 3:
example-3

Input: nums = [2,-1,0,4,-2,-9]
Output: 0
Explanation: One optimal partition is: [2,4,-9] and [-1,0,-2].
The absolute difference between the sums of the arrays is abs((2 + 4 + -9) - (-1 + 0 + -2)) = 0.

Constraints:

    1 <= n <= 15
    nums.length == 2 * n
    -10^7 <= nums[i] <= 10^7
*/

struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        fn binary_search(data: &Vec<i32>, a: i32) -> usize {
            if a > data[data.len() - 1] {
                return data.len();
            }
            let (mut left, mut right) = (0usize, data.len() - 1);
            while left < right {
                let mid = left + (right - left) / 2;
                if data[mid] >= a {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        }

        let n = nums.len() / 2;
        let m = (1 << n) as usize;
        let mut data1: Vec<Vec<i32>> = vec![vec![]; n + 1];
        let mut data2: Vec<Vec<i32>> = vec![vec![]; n + 1];
        for mask in 0..m {
            let (mut sum1, mut sum2, mut cnt) = (0, 0, 0);
            for i in 0..n {
                if (mask & (1 << i)) > 0 {
                    cnt += 1;
                }
                if (mask & (1 << i)) > 0 {
                    sum1 += nums[i];
                } else {
                    sum1 -= nums[i];
                }
                if (mask & (1 << i)) > 0 {
                    sum2 += nums[n + i];
                } else {
                    sum2 -= nums[n + i];
                }
            }
            data1[cnt].push(sum1);
            data2[cnt].push(sum2);
        }
        for data2_i in data2.iter_mut().take(n) {
            data2_i.sort_unstable();
        }
        let mut ret = i32::MAX;
        for i in 0..=n {
            for a in &data1[i] {
                let k = binary_search(&data2[n - i], -a);
                if k != data2[n - i].len() {
                    ret = ret.min(i32::abs(a + data2[n - i][k]));
                }
                if k > 0 {
                    ret = ret.min(i32::abs(a + data2[n - i][k - 1]));
                }
            }
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![3, 9, 7, 3], 2),
        (vec![-36, 36], 72),
        (vec![2, -1, 0, 4, -2, -9], 0),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::minimum_difference(nums), expected);
    }
}
