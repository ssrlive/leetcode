#![allow(dead_code)]

// 3321. Find X-Sum of All K-Long Subarrays II
// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-ii/
// https://leetcode.cn/problems/find-x-sum-of-all-k-long-subarrays-ii/
//
// Hard
//
// You are given an array nums of n integers and two integers k and x.
//
// The x-sum of an array is calculated by the following procedure:
//
// - Count the occurrences of all elements in the array.
// - Keep only the occurrences of the top x most frequent elements. If two elements have the same
//   number of occurrences, the element with the bigger value is considered more frequent.
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
//     nums.length == n
//     1 <= n <= 10^5
//     1 <= nums[i] <= 10^9
//     1 <= x <= k <= nums.length
//

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        use std::cmp::Reverse;
        use std::collections::{BTreeSet, HashMap};

        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<i64>>();
        let (k, x) = (k as i64, x as i64);
        let mut res = vec![];
        let mut cnt = HashMap::new();
        let mut top: BTreeSet<Reverse<(i64, i64)>> = BTreeSet::new();
        let mut bot: BTreeSet<Reverse<(i64, i64)>> = BTreeSet::new();

        let mut running_sum = 0;
        for i in 0..nums.len() {
            let count = cnt.get(&nums[i]).copied().unwrap_or(0);
            if count > 0 {
                if let Some(it) = bot.get(&Reverse((count, nums[i]))).copied() {
                    bot.remove(&it);
                } else {
                    top.remove(&Reverse((count, nums[i])));
                    running_sum -= count * nums[i];
                }
            }
            cnt.insert(nums[i], count + 1);
            top.insert(Reverse((count + 1, nums[i])));
            running_sum += (count + 1) * nums[i];
            if top.len() > x as usize {
                let it_top = top.iter().next_back().copied().unwrap();
                running_sum -= it_top.0.0 * it_top.0.1;
                bot.insert(it_top);
                top.remove(&it_top);
            }
            if i as i64 >= k {
                let count = cnt.get(&nums[i - k as usize]).copied().unwrap_or(0);
                if let Some(it) = bot.get(&Reverse((count, nums[i - k as usize]))).copied() {
                    bot.remove(&it);
                } else {
                    let it = top.get(&Reverse((count, nums[i - k as usize]))).copied().unwrap();
                    running_sum -= it.0.0 * it.0.1;
                    top.remove(&it);
                }
                if count > 1 {
                    bot.insert(Reverse((count - 1, nums[i - k as usize])));
                }
                *cnt.get_mut(&nums[i - k as usize]).unwrap() -= 1;
                if top.len() < x as usize {
                    if let Some(it) = bot.iter().next().copied() {
                        running_sum += it.0.0 * it.0.1;
                        top.insert(it);
                        bot.remove(&it);
                    }
                }
            }
            if i as i64 + 1 >= k {
                res.push(running_sum);
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
    let output = vec![6, 10, 12];
    assert_eq!(Solution::find_x_sum(nums, k, x), output);

    let nums = vec![3, 8, 7, 8, 7, 5];
    let k = 2;
    let x = 2;
    let output = vec![11, 15, 15, 15, 12];
    assert_eq!(Solution::find_x_sum(nums, k, x), output);
}
