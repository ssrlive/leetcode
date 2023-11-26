#![allow(dead_code)]

// 2948. Make Lexicographically Smallest Array by Swapping Elements
// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/
// https://leetcode.cn/problems/make-lexicographically-smallest-array-by-swapping-elements/
//
// Medium
//
// You are given a 0-indexed array of positive integers nums and a positive integer limit.
//
// In one operation, you can choose any two indices i and j and swap nums[i] and nums[j] if |nums[i] - nums[j]| <= limit.
//
// Return the lexicographically smallest array that can be obtained by performing the operation any number of times.
//
// An array a is lexicographically smaller than an array b if in the first position where a and b differ,
// array a has an element that is less than the corresponding element in b.
// For example, the array [2,10,3] is lexicographically smaller than the array [10,2,3] because they differ at index 0 and 2 < 10.
//
// Example 1:
//
// Input: nums = [1,5,3,9,8], limit = 2
// Output: [1,3,5,8,9]
// Explanation: Apply the operation 2 times:
// - Swap nums[1] with nums[2]. The array becomes [1,3,5,9,8]
// - Swap nums[3] with nums[4]. The array becomes [1,3,5,8,9]
// We cannot obtain a lexicographically smaller array by applying any more operations.
// Note that it may be possible to get the same result by doing different operations.
//
// Example 2:
//
// Input: nums = [1,7,6,18,2,1], limit = 3
// Output: [1,6,7,18,1,2]
// Explanation: Apply the operation 3 times:
// - Swap nums[1] with nums[2]. The array becomes [1,6,7,18,2,1]
// - Swap nums[0] with nums[4]. The array becomes [2,6,7,18,1,1]
// - Swap nums[0] with nums[5]. The array becomes [1,6,7,18,1,2]
// We cannot obtain a lexicographically smaller array by applying any more operations.
//
// Example 3:
//
// Input: nums = [1,7,28,19,10], limit = 3
// Output: [1,7,28,19,10]
// Explanation: [1,7,28,19,10] is the lexicographically smallest array we can obtain because we cannot apply the operation on any two indices.
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^9
// 1 <= limit <= 10^9

struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut nums = nums;
        let mut b: Vec<(i32, usize)> = Vec::new();
        let n = nums.len();

        for (i, &num) in nums.iter().enumerate().take(n) {
            b.push((num, i));
        }

        b.sort_by(|x, y| x.0.cmp(&y.0));
        let mut c: Vec<Vec<(i32, usize)>> = vec![vec![b[0]]];

        for i in 1..n {
            if b[i].0 - b[i - 1].0 <= limit {
                c.last_mut().unwrap().push(b[i]);
            } else {
                c.push(vec![b[i]]);
            }
        }

        for t in &c {
            let mut ind: Vec<usize> = Vec::new();
            for p in t {
                ind.push(p.1);
            }
            ind.sort();

            for i in 0..ind.len() {
                nums[ind[i]] = t[i].0;
            }
        }
        nums
    }
}

#[test]
fn test() {
    let nums = vec![1, 5, 3, 9, 8];
    let limit = 2;
    let res = vec![1, 3, 5, 8, 9];
    assert_eq!(Solution::lexicographically_smallest_array(nums, limit), res);

    let nums = vec![1, 7, 6, 18, 2, 1];
    let limit = 3;
    let res = vec![1, 6, 7, 18, 1, 2];
    assert_eq!(Solution::lexicographically_smallest_array(nums, limit), res);

    let nums = vec![1, 7, 28, 19, 10];
    let limit = 3;
    let res = vec![1, 7, 28, 19, 10];
    assert_eq!(Solution::lexicographically_smallest_array(nums, limit), res);
}
