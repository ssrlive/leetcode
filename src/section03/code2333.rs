#![allow(dead_code)]

/*

// 2333. Minimum Sum of Squared Difference
// https://leetcode.com/problems/minimum-sum-of-squared-difference/
// https://leetcode.cn/problems/minimum-sum-of-squared-difference/
//
// Medium
//
// You are given two positive 0-indexed integer arrays nums1 and nums2, both of length n.

The sum of squared difference of arrays nums1 and nums2 is defined as the sum of (nums1[i] - nums2[i])2 for each 0 <= i < n.

You are also given two positive integers k1 and k2. You can modify any of the elements of nums1 by +1 or -1 at most k1 times. Similarly, you can modify any of the elements of nums2 by +1 or -1 at most k2 times.

Return the minimum sum of squared difference after modifying array nums1 at most k1 times and modifying array nums2 at most k2 times.

Note: You are allowed to modify the array elements to become negative integers.

Example 1:

Input: nums1 = [1,2,3,4], nums2 = [2,10,20,19], k1 = 0, k2 = 0
Output: 579
Explanation: The elements in nums1 and nums2 cannot be modified because k1 = 0 and k2 = 0.
The sum of square difference will be: (1 - 2)2 + (2 - 10)2 + (3 - 20)2 + (4 - 19)2 = 579.

Example 2:

Input: nums1 = [1,4,10,12], nums2 = [5,8,6,9], k1 = 1, k2 = 1
Output: 43
Explanation: One way to obtain the minimum sum of square difference is:
- Increase nums1[0] once.
- Increase nums2[2] once.
The minimum of the sum of square difference will be:
(2 - 5)2 + (4 - 8)2 + (10 - 7)2 + (12 - 9)2 = 43.
Note that, there are other ways to obtain the minimum of the sum of square difference, but there is no way to obtain a sum smaller than 43.

Constraints:

    n == nums1.length == nums2.length
    1 <= n <= 10^5
    0 <= nums1[i], nums2[i] <= 10^5
    0 <= k1, k2 <= 10^9
*/

struct Solution;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let n = nums1.len();
        let mut data = vec![0i32; n];
        for i in 0..n {
            data[i] = i32::abs(nums1[i] - nums2[i]);
        }
        data.sort();
        data.reverse();

        let mut sum = vec![0i64; n + 1];
        for i in 1..=n {
            if i < n {
                sum[i] = sum[i - 1] + i as i64 * (data[i - 1] as i64 - data[i] as i64);
            } else {
                sum[i] = sum[i - 1] + i as i64 * data[i - 1] as i64;
            }
        }

        let (mut k, mut ret) = (k1 as i64 + k2 as i64, 0i64);
        let (mut left, mut right) = (1, sum.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if sum[mid] <= k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left == sum.len() {
            return 0;
        }

        for i in 0..left - 1 {
            k -= data[i] as i64 - data[left - 1] as i64;
            data[i] = data[left - 1];
        }

        let m = k / left as i64;
        let mut n1 = (k - left as i64 * m) as usize;
        let mut n2 = left - n1;

        for d in data {
            let mut temp = d as i64;
            if n1 > 0 {
                temp -= m + 1;
                n1 -= 1;
            } else if n2 > 0 {
                temp -= m;
                n2 -= 1;
            }
            ret += temp * temp;
        }

        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0, 579),
        (vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1, 43),
        (vec![1, 10, 4, 4, 2, 7], vec![9, 3, 5, 1, 7, 4], 5, 2, 76),
        (vec![1, 10, 4, 4, 2, 7], vec![9, 3, 5, 1, 7, 4], 5, 3, 67),
    ];
    for (nums1, nums2, k1, k2, expect) in cases {
        assert_eq!(Solution::min_sum_square_diff(nums1, nums2, k1, k2), expect);
    }
}
