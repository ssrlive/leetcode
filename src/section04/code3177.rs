#![allow(dead_code)]

// 3177. Find the Maximum Length of a Good Subsequence II
// https://leetcode.com/problems/find-the-maximum-length-of-a-good-subsequence-ii/
// https://leetcode.cn/problems/find-the-maximum-length-of-a-good-subsequence-ii/
//
// Hard
//
// You are given an integer array nums and a non-negative integer k. A sequence of integers seq is called
// good if there are at most k indices i in the range [0, seq.length - 2] such that seq[i] != seq[i + 1].
//
// Return the maximum possible length of a good subsequence of nums.
//
// Example 1:
//
// Input: nums = [1,2,1,1,3], k = 2
//
// Output: 4
//
// Explanation:
//
// The maximum length subsequence is [1,2,1,1,3].
//
// Example 2:
//
// Input: nums = [1,2,3,4,5,1], k = 0
//
// Output: 2
//
// Explanation:
//
// The maximum length subsequence is [1,2,3,4,5,1].
//
// Constraints:
//
//     1 <= nums.length <= 5 * 10^3
//     1 <= nums[i] <= 10^9
//     0 <= k <= min(50, nums.length)
//

struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;
        let k = k as usize;
        let mut mm = vec![((-1, 0), -1); k + 1];
        let mut rec: HashMap<i32, Vec<i32>> = HashMap::new();
        for &n in &nums {
            if let std::collections::hash_map::Entry::Vacant(e) = rec.entry(n) {
                e.insert(vec![-1; k + 1]);
                rec.entry(n).and_modify(|x| x[0] = 0);
            }
            let recn = rec.get_mut(&n).unwrap();
            recn[0] += 1;
            if recn[0] > mm[0].0.0 {
                mm[0].1 = mm[0].0.0;
                mm[0].0 = (recn[0], n);
            } else if recn[0] > mm[0].1 {
                mm[0].1 = recn[0];
            }
            for i in 1..=k {
                let mut tmp = if recn[i] >= 0 { recn[i] + 1 } else { -1 };
                let m = if mm[i - 1].0.1 == n { mm[i - 1].1 } else { mm[i - 1].0.0 } + 1;
                tmp = max(m, tmp);
                if tmp <= 0 {
                    break;
                }
                recn[i] = tmp;
                if tmp > mm[i].0.0 {
                    mm[i].1 = mm[i].0.0;
                    mm[i].0 = (tmp, n);
                } else if tmp > mm[i].1 {
                    mm[i].1 = tmp;
                }
            }
        }
        mm.into_iter().map(|x| x.0.0).max().unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
    assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
    assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 0), 3);
}
