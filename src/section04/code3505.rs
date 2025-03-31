#![allow(dead_code)]

// 3505. Minimum Operations to Make Elements Within K Subarrays Equal
// https://leetcode.com/problems/minimum-operations-to-make-elements-within-k-subarrays-equal/
// https://leetcode.cn/problems/minimum-operations-to-make-elements-within-k-subarrays-equal/
//
// Hard
//
// You are given an integer array nums and two integers, x and k. You can perform the following operation any number of times (including zero):
//
//     Increase or decrease any element of nums by 1.
//
// Return the minimum number of operations needed to have at least k non-overlapping of size exactly x in nums, where all elements within each subarray are equal.
//
// Example 1:
//
// Input: nums = [5,-2,1,3,7,3,6,4,-1], x = 3, k = 2
//
// Output: 8
//
// Explanation:
//
//     Use 3 operations to add 3 to nums[1] and use 2 operations to subtract 2 from nums[3]. The resulting array is [5, 1, 1, 1, 7, 3, 6, 4, -1].
//     Use 1 operation to add 1 to nums[5] and use 2 operations to subtract 2 from nums[6]. The resulting array is [5, 1, 1, 1, 7, 4, 4, 4, -1].
//     Now, all elements within each subarray [1, 1, 1] (from indices 1 to 3) and [4, 4, 4] (from indices 5 to 7) are equal. Since 8 total operations were used, 8 is the output.
//
// Example 2:
//
// Input: nums = [9,-2,-2,-2,1,5], x = 2, k = 2
//
// Output: 3
//
// Explanation:
//
// - Use 3 operations to subtract 3 from nums[4]. The resulting array is [9, -2, -2, -2, -2, 5].
// - Now, all elements within each subarray [-2, -2] (from indices 1 to 2) and [-2, -2] (from indices 3 to 4)
//   are equal. Since 3 operations were used, 3 is the output.
//
// Constraints:
//
//     2 <= nums.length <= 10^5
//     -10^6 <= nums[i] <= 10^6
//     2 <= x <= nums.length
//     1 <= k <= 15
//     2 <= k * x <= nums.length
//

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        struct Heap {
            size: usize,
            sum: i64,
            data: std::collections::BTreeMap<i64, i64>,
        }

        impl Heap {
            fn new() -> Self {
                Self {
                    size: 0,
                    sum: 0,
                    data: std::collections::BTreeMap::new(),
                }
            }

            fn add(&mut self, num: i64) {
                self.sum += num;
                self.size += 1;
                *self.data.entry(num).or_insert(0) += 1;
            }

            fn remove(&mut self, num: i64) {
                self.sum -= num;
                self.size -= 1;
                if let Some(count) = self.data.get_mut(&num) {
                    *count -= 1;
                    if *count == 0 {
                        self.data.remove(&num);
                    }
                }
            }

            fn get_first(&self) -> i64 {
                *self.data.keys().next().unwrap()
            }

            fn get_last(&self) -> i64 {
                *self.data.keys().last().unwrap()
            }
        }

        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let n = nums.len();
        let mut costs = vec![i64::MAX; n];
        let mut left_part = Heap::new();
        let mut right_part = Heap::new();
        for i in 0..n {
            let added = nums[i];
            if left_part.size == 0 && right_part.size == 0 {
                left_part.add(added);
                continue;
            }
            let transfer;
            if left_part.size <= right_part.size {
                transfer = right_part.get_first();
                right_part.remove(transfer);
            } else {
                transfer = left_part.get_last();
                left_part.remove(transfer);
            }
            left_part.add(added.min(transfer));
            right_part.add(added.max(transfer));

            if left_part.size + right_part.size == x as usize {
                let median = left_part.get_last();
                costs[i] = median * left_part.size as i64 - left_part.sum + right_part.sum - median * right_part.size as i64;
                let outed = nums[i - (x - 1) as usize];
                if outed <= median {
                    left_part.remove(outed);
                } else {
                    right_part.remove(outed);
                }
            }
        }
        let mut dp = vec![i64::MAX; n];
        for i in (x - 1) as usize..n {
            dp[i] = dp[i - 1].min(costs[i]);
        }
        for i in 1..k {
            let mut new_dp = vec![i64::MAX; n];
            for j in ((i + 1) * x - 1) as usize..n {
                new_dp[j] = new_dp[j - 1].min(costs[j] + dp[j - x as usize]);
            }
            dp = new_dp;
        }
        dp[n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2), 8);
    assert_eq!(Solution::min_operations(vec![9, -2, -2, -2, 1, 5], 2, 2), 3);
    assert_eq!(Solution::min_operations(vec![1, 2], 2, 1), 1);
}
