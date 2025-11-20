#![allow(dead_code)]

// 3691. Maximum Total Subarray Value II
// https://leetcode.com/problems/maximum-total-subarray-value-ii/
// https://leetcode.cn/problems/maximum-total-subarray-value-ii/
//
// Hard
//
// You are given an integer array nums of length n and an integer k.
//
// You must select exactly k distinct non-empty subarrays nums[l..r] of nums. Subarrays may overlap, but the exact same subarray (same l and r) cannot be chosen more than once.
//
// The value of a subarray nums[l..r] is defined as: max(nums[l..r]) - min(nums[l..r]).
//
// The total value is the sum of the values of all chosen subarrays.
//
// Return the maximum possible total value you can achieve.
//
// Example 1:
//
// Input: nums = [1,3,2], k = 2
//
// Output: 4
//
// Explanation:
//
// One optimal approach is:
//
// Choose nums[0..1] = [1, 3]. The maximum is 3 and the minimum is 1, giving a value of 3 - 1 = 2.
// Choose nums[0..2] = [1, 3, 2]. The maximum is still 3 and the minimum is still 1, so the value is also 3 - 1 = 2.
// Adding these gives 2 + 2 = 4.
//
// Example 2:
//
// Input: nums = [4,2,5,1], k = 3
//
// Output: 12
//
// Explanation:
//
// One optimal approach is:
//
// Choose nums[0..3] = [4, 2, 5, 1]. The maximum is 5 and the minimum is 1, giving a value of 5 - 1 = 4.
// Choose nums[1..3] = [2, 5, 1]. The maximum is 5 and the minimum is 1, so the value is also 4.
// Choose nums[2..3] = [5, 1]. The maximum is 5 and the minimum is 1, so the value is again 4.
// Adding these gives 4 + 4 + 4 = 12.
//
// Constraints:
//
// 1 <= n == nums.length <= 5 * 10​​​​​​​^4
// 0 <= nums[i] <= 10^9
// 1 <= k <= min(10^5, n * (n + 1) / 2)
//

struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        fn compute_val(
            i: i32,
            j: i32,
            l: usize,
            r: usize,
            nums: &[i32],
            ids: &[usize],
            vis: &mut std::collections::HashSet<(i32, i32)>,
        ) -> Option<(i64, i32, i32, usize, usize)> {
            if !vis.insert((i, j)) {
                return None;
            }
            let mut ll = l;
            let mut rr = r;
            while ids[ll] as i32 <= i || ids[ll] as i32 >= j {
                ll += 1;
            }
            while ids[rr] as i32 <= i || ids[rr] as i32 >= j {
                rr -= 1;
            }
            Some(((nums[ids[rr]] - nums[ids[ll]]) as i64, i, j, ll, rr))
        }

        let mut ids: Vec<usize> = (0..nums.len()).collect();
        ids.sort_by(|&i, &j| if nums[i] == nums[j] { i.cmp(&j) } else { nums[i].cmp(&nums[j]) });
        let mut res: i64 = 0;
        let mut pq: std::collections::BinaryHeap<(i64, i32, i32, usize, usize)> = std::collections::BinaryHeap::new();
        let mut vis: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
        if let Some(val) = compute_val(-1, nums.len() as i32, 0, nums.len() - 1, &nums, &ids, &mut vis) {
            pq.push(val);
        }
        let mut kk = k;
        while let Some((val, i, j, l, r)) = pq.pop() {
            if val == 0 || kk <= 0 {
                break;
            }
            let ni = std::cmp::min(ids[r], ids[l]) as i32;
            let nj = std::cmp::max(ids[r], ids[l]) as i32;
            let cnt = std::cmp::min((ni - i) as i64 * (j - nj) as i64, kk as i64);
            kk -= cnt as i32;
            res += val * cnt;
            if let Some(new_val) = compute_val(ni, j, l, r, &nums, &ids, &mut vis) {
                pq.push(new_val);
            }
            if let Some(new_val) = compute_val(i, nj, l, r, &nums, &ids, &mut vis) {
                pq.push(new_val);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 3, 2];
    let k1 = 2;
    assert_eq!(Solution::max_total_value(nums1, k1), 4);

    let nums2 = vec![4, 2, 5, 1];
    let k2 = 3;
    assert_eq!(Solution::max_total_value(nums2, k2), 12);
}
