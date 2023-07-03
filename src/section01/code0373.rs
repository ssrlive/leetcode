#![allow(dead_code)]

// 373. Find K Pairs with Smallest Sums
// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
// https://leetcode.cn/problems/find-k-pairs-with-smallest-sums/
//
// You are given two integer arrays nums1 and nums2 sorted in ascending order and an integer k.
//
// Define a pair (u, v) which consists of one element from the first array and one element from the second array.
//
// Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.
//
// Example 1:
//
// Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
// Output: [[1,2],[1,4],[1,6]]
// Explanation: The first 3 pairs are returned from the sequence: [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
//
// Example 2:
//
// Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
// Output: [[1,1],[1,1]]
// Explanation: The first 2 pairs are returned from the sequence: [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
//
// Example 3:
//
// Input: nums1 = [1,2], nums2 = [3], k = 3
// Output: [[1,3],[2,3]]
// Explanation: All possible pairs are returned from the sequence: [1,3],[2,3]
//
// Constraints:
//
// - 1 <= nums1.length, nums2.length <= 10^5
// - -10^9 <= nums1[i], nums2[i] <= 10^9
// - nums1 and nums2 both are sorted in ascending order.
// - 1 <= k <= 10^4
//

struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let (n1, n2) = (nums1.len(), nums2.len());
        let mut heap =
            std::collections::BinaryHeap::from((0..n1).map(|x| (-nums1[x] - nums2[0], x, 0)).collect::<Vec<(i32, usize, usize)>>());
        while let Some((_, i, j)) = heap.pop() {
            res.push(vec![nums1[i], nums2[j]]);
            if res.len() == k as usize {
                break;
            }
            if j < n2 - 1 {
                heap.push((-nums1[i] - nums2[j + 1], i, j + 1));
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
        vec![vec![1, 2], vec![1, 4], vec![1, 6]]
    );
    assert_eq!(
        Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
        vec![vec![1, 1], vec![1, 1]]
    );
    assert_eq!(Solution::k_smallest_pairs(vec![1, 2], vec![3], 3), vec![vec![1, 3], vec![2, 3]]);
}
