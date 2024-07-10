#![allow(dead_code)]

// 3209. Number of Subarrays With AND Value of K
// https://leetcode.com/problems/number-of-subarrays-with-and-value-of-k/
// https://leetcode.cn/problems/number-of-subarrays-with-and-value-of-k/
//
// Hard
//
// Given an array of integers nums and an integer k, return the number of
// subarrays of nums where the bitwise AND of the elements of the subarray equals k.
//
// Example 1:
//
// Input: nums = [1,1,1], k = 1
//
// Output: 6
//
// Explanation:
//
// All subarrays contain only 1's.
//
// Example 2:
//
// Input: nums = [1,1,2], k = 1
//
// Output: 3
//
// Explanation:
//
// Subarrays having an AND value of 1 are: [1,1,2], [1,1,2], [1,1,2].
//
// Example 3:
//
// Input: nums = [1,2,3], k = 2
//
// Output: 2
//
// Explanation:
//
// Subarrays having an AND value of 2 are: [1,2,3], [1,2,3].
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     0 <= nums[i], k <= 10^9
//

struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let bits: Vec<bool> = (0..30).map(|j| k & (1 << j) != 0).collect();

        let mut b = 0;
        for &i in &nums {
            let mut tb = 0;
            while (1 << tb) <= i {
                tb += 1;
            }
            b = b.max(tb);
        }
        let mut tb = 0;
        while (1 << tb) <= k {
            tb += 1;
        }
        b = b.max(tb);

        let mut z = vec![std::collections::VecDeque::new(); b];
        for (i, &nums_i) in nums.iter().enumerate().take(n) {
            for (j, z_j) in z.iter_mut().enumerate().take(b) {
                if nums_i & (1 << j) == 0 {
                    z_j.push_back(i);
                }
            }
        }

        let mut res = 0;
        for i in 0..n {
            let mut from = i;
            let mut to = n;
            for j in 0..b {
                if bits[j] {
                    to = to.min(*z[j].front().unwrap_or(&n));
                } else {
                    from = from.max(*z[j].front().unwrap_or(&n));
                }
                if to <= from {
                    break;
                }
            }
            if to > from {
                res += (to - from) as i64;
            }
            for z_j in z.iter_mut().take(b) {
                if !z_j.is_empty() && z_j[0] == i {
                    z_j.pop_front();
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1];
    let k = 1;
    let res = 6;
    assert_eq!(Solution::count_subarrays(nums, k), res);

    let nums = vec![1, 1, 2];
    let k = 1;
    let res = 3;
    assert_eq!(Solution::count_subarrays(nums, k), res);

    let nums = vec![1, 2, 3];
    let k = 2;
    let res = 2;
    assert_eq!(Solution::count_subarrays(nums, k), res);
}
