#![allow(dead_code)]

// 3318. Find X-Sum of All K-Long Subarrays I
// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/
// https://leetcode.cn/problems/find-x-sum-of-all-k-long-subarrays-i/
//
// Easy
//
// You are given an array nums of n integers and two integers k and x.
//
// The x-sum of an array is calculated by the following procedure:
//
// - Count the occurrences of all elements in the array.
// - Keep only the occurrences of the top x most frequent elements. If two elements have the same number of occurrences,
//   the element with the bigger value is considered more frequent.
// - Calculate the sum of the resulting array.
//
// Note that if an array has less than x distinct elements, its x-sum is the sum of the array.
//
// Return an integer array answer of length n - k + 1 where answer[i] is the x-sum of the subarray nums[i..i + k - 1].
//
// Example 1:
//
// Input: nums = [1,1,2,2,3,4,2,3], k = 6, x = 2
//
// Output: [6,10,12]
//
// Explanation:
//
// - For subarray [1, 1, 2, 2, 3, 4], only elements 1 and 2 will be kept in the resulting array. Hence, answer[0] = 1 + 1 + 2 + 2.
// - For subarray [1, 2, 2, 3, 4, 2], only elements 2 and 4 will be kept in the resulting array. Hence, answer[1] = 2 + 2 + 2 + 4.
//   Note that 4 is kept in the array since it is bigger than 3 and 1 which occur the same number of times.
// - For subarray [2, 2, 3, 4, 2, 3], only elements 2 and 3 are kept in the resulting array. Hence, answer[2] = 2 + 2 + 2 + 3 + 3.
//
// Example 2:
//
// Input: nums = [3,8,7,8,7,5], k = 2, x = 2
//
// Output: [11,15,15,15,12]
//
// Explanation:
//
// Since k == x, answer[i] is equal to the sum of the subarray nums[i..i + k - 1].
//
// Constraints:
//
//     1 <= n == nums.length <= 50
//     1 <= nums[i] <= 50
//     1 <= x <= k <= nums.length
//

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        use std::collections::BTreeMap;
        let n = nums.len();
        let mut res = Vec::new();
        let k = k as usize - 1;

        let mut track = BTreeMap::new();
        fn get_sum(track: &mut BTreeMap<i32, i32>, x: i32) -> i32 {
            let mut r = Vec::new();
            for (x, count) in track {
                r.push((*count, *x));
            }
            r.sort_by(|a, b| b.cmp(a));

            let mut sum = 0;
            for r_j in r.iter().take(std::cmp::min(r.len(), x as usize)) {
                sum += r_j.0 * r_j.1;
            }
            sum
        }

        for i in 0..n {
            *track.entry(nums[i]).or_insert(0) += 1;
            if i >= k {
                res.push(get_sum(&mut track, x));
                if let Some(count) = track.get_mut(&nums[i - k]) {
                    *count -= 1;
                    if *count == 0 {
                        track.remove(&nums[i - k]);
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
    let k = 6;
    let x = 2;
    let res = vec![6, 10, 12];
    assert_eq!(Solution::find_x_sum(nums, k, x), res);

    let nums = vec![3, 8, 7, 8, 7, 5];
    let k = 2;
    let x = 2;
    let res = vec![11, 15, 15, 15, 12];
    assert_eq!(Solution::find_x_sum(nums, k, x), res);
}
