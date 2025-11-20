#![allow(dead_code)]

// 3690. Split and Merge Array Transformation
// https://leetcode.com/problems/split-and-merge-array-transformation/
// https://leetcode.cn/problems/split-and-merge-array-transformation/
//
// Medium
//
// You are given two integer arrays nums1 and nums2, each of length n. You may perform the following split-and-merge operation on nums1 any number of times:
//
// Choose a subarray nums1[L..R].
// Remove that subarray, leaving the prefix nums1[0..L-1] (empty if L = 0) and the suffix nums1[R+1..n-1] (empty if R = n - 1).
// Re-insert the removed subarray (in its original order) at any position in the remaining array (i.e., between any two elements, at the very start, or at the very end).
// Return the minimum number of split-and-merge operations needed to transform nums1 into nums2.
//
// Example 1:
//
// Input: nums1 = [3,1,2], nums2 = [1,2,3]
//
// Output: 1
//
// Explanation:
//
// Split out the subarray [3] (L = 0, R = 0); the remaining array is [1,2].
// Insert [3] at the end; the array becomes [1,2,3].
//
// Example 2:
//
// Input: nums1 = [1,1,2,3,4,5], nums2 = [5,4,3,2,1,1]
//
// Output: 3
//
// Explanation:
//
// Remove [1,1,2] at indices 0 - 2; remaining is [3,4,5]; insert [1,1,2] at position 2, resulting in [3,4,1,1,2,5].
// Remove [4,1,1] at indices 1 - 3; remaining is [3,2,5]; insert [4,1,1] at position 3, resulting in [3,2,5,4,1,1].
// Remove [3,2] at indices 0 - 1; remaining is [5,4,1,1]; insert [3,2] at position 2, resulting in [5,4,3,2,1,1].
//
// Constraints:
//
// 2 <= n == nums1.length == nums2.length <= 6
// -10^5 <= nums1[i], nums2[i] <= 10^5
// nums2 is a permutation of nums1.
//

struct Solution;

impl Solution {
    pub fn min_split_merge(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        const N: usize = 6;
        const POWERS: [usize; 7] = [1, N, N * N, N * N * N, N * N * N * N, N * N * N * N * N, N * N * N * N * N * N];

        fn compress(arr: &[i32]) -> i32 {
            let mut compressed = 0;
            for &num in arr {
                compressed = compressed * N as i32 + num;
            }
            compressed
        }

        fn convert_to_rank(nums: &mut [i32]) {
            let mut buffer_storage = [0; N];
            let buffer = &mut buffer_storage[..nums.len()];
            for (i, &v) in nums.iter().enumerate() {
                buffer[i] = v;
            }
            buffer.sort_unstable();
            for n in nums.iter_mut() {
                for (i, &v) in buffer.iter().enumerate() {
                    if *n == v {
                        *n = i as i32;
                        break;
                    }
                }
            }
        }

        if nums1 == nums2 {
            return 0;
        }
        let n = nums1.len();
        convert_to_rank(&mut nums1);
        convert_to_rank(&mut nums2);
        let start = compress(&nums1);
        let target = compress(&nums2);
        let mut st = std::collections::HashSet::with_capacity(720);
        let mut q = std::collections::VecDeque::with_capacity(720);
        let mut next_layer = Vec::with_capacity(720 * 6);
        st.insert(start);
        q.push_back(start);
        let mut steps = 0;
        loop {
            let current_layer_size = q.len();
            for _ in 0..current_layer_size {
                let Some(curr) = q.pop_front() else {
                    unsafe { std::hint::unreachable_unchecked() }
                };
                for l in 0..n {
                    for r in l..n {
                        let (a, b) = (curr / POWERS[r + 1] as i32, curr % POWERS[r + 1] as i32);
                        let (c, d) = (b / POWERS[l] as i32, b % POWERS[l] as i32);
                        let (newli, rem) = (c, a * POWERS[l] as i32 + d);
                        for idx in 0..=(n - (r - l + 1)) {
                            let (a, b) = (rem / POWERS[idx] as i32, rem % POWERS[idx] as i32);
                            let new = a * POWERS[idx + (r - l + 1)] as i32 + newli * POWERS[idx] as i32 + b;
                            if new == target {
                                return steps + 1;
                            }
                            next_layer.push(new);
                        }
                    }
                }
                next_layer.sort_unstable();
                next_layer.dedup();
                for &next in &next_layer {
                    if st.insert(next) {
                        q.push_back(next);
                    }
                }
                next_layer.clear();
            }
            steps += 1;
        }
    }
}

#[test]
fn test() {
    let nums1 = vec![3, 1, 2];
    let nums2 = vec![1, 2, 3];
    assert_eq!(Solution::min_split_merge(nums1, nums2), 1);

    let nums1 = vec![1, 1, 2, 3, 4, 5];
    let nums2 = vec![5, 4, 3, 2, 1, 1];
    assert_eq!(Solution::min_split_merge(nums1, nums2), 3);
}
