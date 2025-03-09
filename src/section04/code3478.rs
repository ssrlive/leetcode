#![allow(dead_code)]

// 3478. Choose K Elements With Maximum Sum
// https://leetcode.com/problems/choose-k-elements-with-maximum-sum/
// https://leetcode.cn/problems/choose-k-elements-with-maximum-sum/
//
// Medium
//
// You are given two integer arrays, nums1 and nums2, both of length n, along with a positive integer k.
//
// For each index i from 0 to n - 1, perform the following:
//
// Find all indices j where nums1[j] is less than nums1[i].
// Choose at most k values of nums2[j] at these indices to maximize the total sum.
// Return an array answer of size n, where answer[i] represents the result for the corresponding index i.
//
// Example 1:
//
// Input: nums1 = [4,2,1,5,3], nums2 = [10,20,30,40,50], k = 2
//
// Output: [80,30,0,80,50]
//
// Explanation:
//
// For i = 0: Select the 2 largest values from nums2 at indices [1, 2, 4] where nums1[j] < nums1[0], resulting in 50 + 30 = 80.
// For i = 1: Select the 2 largest values from nums2 at index [2] where nums1[j] < nums1[1], resulting in 30.
// For i = 2: No indices satisfy nums1[j] < nums1[2], resulting in 0.
// For i = 3: Select the 2 largest values from nums2 at indices [0, 1, 2, 4] where nums1[j] < nums1[3], resulting in 50 + 30 = 80.
// For i = 4: Select the 2 largest values from nums2 at indices [1, 2] where nums1[j] < nums1[4], resulting in 30 + 20 = 50.
//
// Example 2:
//
// Input: nums1 = [2,2,2,2], nums2 = [3,1,2,3], k = 1
//
// Output: [0,0,0,0]
//
// Explanation:
//
// Since all elements in nums1 are equal, no indices satisfy the condition nums1[j] < nums1[i] for any i, resulting in 0 for all positions.
//
// Constraints:
//
// n == nums1.length == nums2.length
// 1 <= n <= 10^5
// 1 <= nums1[i], nums2[i] <= 10^6
// 1 <= k <= n
//

struct Solution;

impl Solution {
    pub fn find_max_sum(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i64> {
        let n = nums1.len();
        let mut vec = vec![(0, (0, 0)); n];

        for i in 0..n {
            vec[i] = (nums1[i] as i64, (i as i64, nums2[i] as i64));
        }
        vec.sort();

        let mut pq = std::collections::BinaryHeap::new();
        let mut curr_sum = 0;
        let mut ans = vec![0; n];

        let mut i = 0;
        while i < n {
            let curr_num = vec[i].0;
            let mut j = i;

            while j < n && vec[j].0 == curr_num {
                ans[vec[j].1.0 as usize] = curr_sum;
                j += 1;
            }

            for vec_l in vec.iter().take(j).skip(i) {
                pq.push(std::cmp::Reverse(vec_l.1.1));
                curr_sum += vec_l.1.1;

                if pq.len() > k as usize {
                    curr_sum -= pq.pop().unwrap().0;
                }
            }
            i = j;
        }
        ans
    }
}

#[test]
fn test() {
    let nums1 = vec![4, 2, 1, 5, 3];
    let nums2 = vec![10, 20, 30, 40, 50];
    let k = 2;
    let output = vec![80, 30, 0, 80, 50];
    assert_eq!(Solution::find_max_sum(nums1, nums2, k), output);

    let nums1 = vec![2, 2, 2, 2];
    let nums2 = vec![3, 1, 2, 3];
    let k = 1;
    let output = vec![0, 0, 0, 0];
    assert_eq!(Solution::find_max_sum(nums1, nums2, k), output);
}
