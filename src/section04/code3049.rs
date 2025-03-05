#![allow(dead_code)]

// 3049. Earliest Second to Mark Indices II
// https://leetcode.com/problems/earliest-second-to-mark-indices-ii/
// https://leetcode.cn/problems/earliest-second-to-mark-indices-ii/
//
// Hard
//
// You are given two 1-indexed integer arrays, nums and, changeIndices, having lengths n and m, respectively.
//
// Initially, all indices in nums are unmarked. Your task is to mark all indices in nums.
//
// In each second, s, in order from 1 to m (inclusive), you can perform one of the following operations:
//
// Choose an index i in the range [1, n] and decrement nums[i] by 1.
// Set nums[changeIndices[s]] to any non-negative value.
// Choose an index i in the range [1, n], where nums[i] is equal to 0, and mark index i.
// Do nothing.
// Return an integer denoting the earliest second in the range [1, m] when all indices
// in nums can be marked by choosing operations optimally, or -1 if it is impossible.
//
// Example 1:
//
// Input: nums = [3,2,3], changeIndices = [1,3,2,2,2,2,3]
// Output: 6
// Explanation: In this example, we have 7 seconds. The following operations can be performed to mark all indices:
// Second 1: Set nums[changeIndices[1]] to 0. nums becomes [0,2,3].
// Second 2: Set nums[changeIndices[2]] to 0. nums becomes [0,2,0].
// Second 3: Set nums[changeIndices[3]] to 0. nums becomes [0,0,0].
// Second 4: Mark index 1, since nums[1] is equal to 0.
// Second 5: Mark index 2, since nums[2] is equal to 0.
// Second 6: Mark index 3, since nums[3] is equal to 0.
// Now all indices have been marked.
// It can be shown that it is not possible to mark all indices earlier than the 6th second.
// Hence, the answer is 6.
//
// Example 2:
//
// Input: nums = [0,0,1,2], changeIndices = [1,2,1,2,1,2,1,2]
// Output: 7
// Explanation: In this example, we have 8 seconds. The following operations can be performed to mark all indices:
// Second 1: Mark index 1, since nums[1] is equal to 0.
// Second 2: Mark index 2, since nums[2] is equal to 0.
// Second 3: Decrement index 4 by one. nums becomes [0,0,1,1].
// Second 4: Decrement index 4 by one. nums becomes [0,0,1,0].
// Second 5: Decrement index 3 by one. nums becomes [0,0,0,0].
// Second 6: Mark index 3, since nums[3] is equal to 0.
// Second 7: Mark index 4, since nums[4] is equal to 0.
// Now all indices have been marked.
// It can be shown that it is not possible to mark all indices earlier than the 7th second.
// Hence, the answer is 7.
//
// Example 3:
//
// Input: nums = [1,2,3], changeIndices = [1,2,3]
// Output: -1
// Explanation: In this example, it can be shown that it is impossible to mark all indices, as we don't have enough seconds.
// Hence, the answer is -1.
//
// Constraints:
//
// 1 <= n == nums.length <= 5000
// 0 <= nums[i] <= 10^9
// 1 <= m == changeIndices.length <= 5000
// 1 <= changeIndices[i] <= n
//

struct Solution;

impl Solution {
    pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let change_indices = change_indices.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let _n = nums.len();
        let m = change_indices.len() as i64;
        let mut lo = 1_i64;
        let mut hi = m + 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let okval = Solution::ok(&nums, &change_indices, mid);
            if okval {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if lo == m + 1 { -1 } else { lo as _ }
    }

    fn ok(nums: &[i64], change_indices: &[i64], ans: i64) -> bool {
        let n = nums.len();
        let m = change_indices.len() as i64;
        if ans == m + 1 {
            return true;
        }

        let mut seen = vec![false; n];
        let mut ps = vec![];
        for j in 0..ans {
            if !seen[(change_indices[j as usize] - 1) as usize] {
                seen[(change_indices[j as usize] - 1) as usize] = true;
                ps.push((j, (change_indices[j as usize] - 1) as usize));
            }
        }
        let mut k = ps.len() as i32 - 1;

        let mut remaining = 0;
        let mut pq = std::collections::BinaryHeap::new();
        for j in (0..ans).rev() {
            if k >= 0 && ps[k as usize].0 == j {
                if nums[ps[k as usize].1] > 0 {
                    pq.push((-nums[ps[k as usize].1], ps[k as usize].1));
                } else {
                    remaining += 1;
                }
                k -= 1;
            } else {
                remaining += 1;
            }
            while remaining < pq.len() {
                pq.pop();
                remaining += 1;
            }
        }

        let mut needops = n as i64;
        let mut cnt = nums.to_vec();
        while let Some((_negcnt, i)) = pq.pop() {
            cnt[i] = 0;
            needops += 1;
        }
        needops += cnt.iter().sum::<i64>();
        needops <= ans
    }
}

#[test]
fn test() {
    let nums = vec![3, 2, 3];
    let change_indices = vec![1, 3, 2, 2, 2, 2, 3];
    let res = 6;
    assert_eq!(Solution::earliest_second_to_mark_indices(nums, change_indices), res);

    let nums = vec![0, 0, 1, 2];
    let change_indices = vec![1, 2, 1, 2, 1, 2, 1, 2];
    let res = 7;
    assert_eq!(Solution::earliest_second_to_mark_indices(nums, change_indices), res);

    let nums = vec![1, 2, 3];
    let change_indices = vec![1, 2, 3];
    let res = -1;
    assert_eq!(Solution::earliest_second_to_mark_indices(nums, change_indices), res);
}
