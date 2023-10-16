#![allow(dead_code)]

// 2762. Continuous Subarrays
// https://leetcode.com/problems/continuous-subarrays/
// https://leetcode.cn/problems/continuous-subarrays/
//
// Medium
//
// You are given a 0-indexed integer array nums. A subarray of nums is called continuous if:
//
//     Let i, i + 1, ..., j be the indices in the subarray.
//     Then, for each pair of indices i <= i1, i2 <= j, 0 <= |nums[i1] - nums[i2]| <= 2.
//
// Return the total number of continuous subarrays.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [5,4,2,4]
// Output: 8
// Explanation:
// Continuous subarray of size 1: [5], [4], [2], [4].
// Continuous subarray of size 2: [5,4], [4,2], [2,4].
// Continuous subarray of size 3: [4,2,4].
// Thereare no subarrys of size 4.
// Total continuous subarrays = 4 + 3 + 1 = 8.
// It can be shown that there are no more continuous subarrays.
//
// Example 2:
//
// Input: nums = [1,2,3]
// Output: 6
// Explanation:
// Continuous subarray of size 1: [1], [2], [3].
// Continuous subarray of size 2: [1,2], [2,3].
// Continuous subarray of size 3: [1,2,3].
// Total continuous subarrays = 3 + 2 + 1 = 6.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut mp = std::collections::BTreeMap::<i32, i32>::new();
        let mut ret = 0;
        let (mut r, n) = (0, nums.len());

        for i in 0..n {
            if i > 0 {
                if *mp.get(&nums[i - 1]).unwrap() == 1 {
                    mp.remove(&nums[i - 1]);
                } else {
                    *mp.entry(nums[i - 1]).or_insert(0) -= 1;
                }
            }

            while r < n {
                if mp.is_empty() {
                    *mp.entry(nums[r]).or_insert(0) += 1;
                    r += 1;
                    continue;
                }

                let (mut mn, mut mx) = (0, i32::MAX);
                if let Some((v, _)) = mp.iter().next() {
                    mn = *v;
                }
                if let Some((v, _)) = mp.iter().next_back() {
                    mx = *v;
                }

                if nums[r] - 2 > mn || nums[r] + 2 < mx {
                    break;
                }
                *mp.entry(nums[r]).or_insert(0) += 1;
                r += 1;
            }

            ret += r as i64 - i as i64;
        }

        ret
    }
}

#[test]
fn test() {
    let nums = vec![5, 4, 2, 4];
    let res = 8;
    assert_eq!(Solution::continuous_subarrays(nums), res);

    let nums = vec![1, 2, 3];
    let res = 6;
    assert_eq!(Solution::continuous_subarrays(nums), res);
}
