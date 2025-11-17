#![allow(dead_code)]

// 3653. XOR After Range Multiplication Queries I
// https://leetcode.com/problems/xor-after-range-multiplication-queries-i/
// https://leetcode.cn/problems/xor-after-range-multiplication-queries-i/
//
// Medium
//
// You are given an integer array nums of length n and a 2D integer array queries of size q, where queries[i] = [li, ri, ki, vi].
//
// For each query, you must apply the following operations in order:
//
// Set idx = li.
// While idx <= ri:
// Update: nums[idx] = (nums[idx] * vi) % (10^9 + 7)
// Set idx += ki.
// Return the bitwise XOR of all elements in nums after processing all queries.
//
// Example 1:
//
// Input: nums = [1,1,1], queries = [[0,2,1,4]]
//
// Output: 4
//
// Explanation:
//
// A single query [0, 2, 1, 4] multiplies every element from index 0 through index 2 by 4.
// The array changes from [1, 1, 1] to [4, 4, 4].
// The XOR of all elements is 4 ^ 4 ^ 4 = 4.
//
// Example 2:
//
// Input: nums = [2,3,1,5,4], queries = [[1,4,2,3],[0,2,1,2]]
//
// Output: 31
//
// Explanation:
//
// The first query [1, 4, 2, 3] multiplies the elements at indices 1 and 3 by 3, transforming the array to [2, 9, 1, 15, 4].
// The second query [0, 2, 1, 2] multiplies the elements at indices 0, 1, and 2 by 2, resulting in [4, 18, 2, 15, 4].
// Finally, the XOR of all elements is 4 ^ 18 ^ 2 ^ 15 ^ 4 = 31.​​​​​​​​​​​​​​
//
// Constraints:
//
// 1 <= n == nums.length <= 10^3
// 1 <= nums[i] <= 10^9
// 1 <= q == queries.length <= 10^3
// queries[i] = [li, ri, ki, vi]
// 0 <= li <= ri < n
// 1 <= ki <= n
// 1 <= vi <= 10^5
//

struct Solution;

impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;

        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as usize;
            let v = q[3] as u64;

            let mut i = l;

            while i <= r {
                let x = nums[i] as u64 * v;
                let a = x % (1_000_000_000 + 7);
                nums[i] = a as i32;
                i += k;
            }
        }

        nums.iter().fold(0, |acc, &n| acc ^ n)
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1];
    let queries = vec![vec![0, 2, 1, 4]];
    assert_eq!(Solution::xor_after_queries(nums, queries), 4);

    let nums = vec![2, 3, 1, 5, 4];
    let queries = vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]];
    assert_eq!(Solution::xor_after_queries(nums, queries), 31);
}
