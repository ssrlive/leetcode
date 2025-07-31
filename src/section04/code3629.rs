#![allow(dead_code)]

// 3629. Minimum Jumps to Reach End via Prime Teleportation
// https://leetcode.com/problems/minimum-jumps-to-reach-end-via-prime-teleportation/
// https://leetcode.cn/problems/minimum-jumps-to-reach-end-via-prime-teleportation/
//
// Medium
//
// You are given an integer array nums of length n.
//
// You start at index 0, and your goal is to reach index n - 1.
//
// From any index i, you may perform one of the following operations:
//
// Adjacent Step: Jump to index i + 1 or i - 1, if the index is within bounds.
// Prime Teleportation: If nums[i] is a prime number p, you may instantly jump to any index j != i such that nums[j] % p == 0.
// Return the minimum number of jumps required to reach index n - 1.
//
// Example 1:
//
// Input: nums = [1,2,4,6]
//
// Output: 2
//
// Explanation:
//
// One optimal sequence of jumps is:
//
// Start at index i = 0. Take an adjacent step to index 1.
// At index i = 1, nums[1] = 2 is a prime number. Therefore, we teleport to index i = 3 as nums[3] = 6 is divisible by 2.
// Thus, the answer is 2.
//
// Example 2:
//
// Input: nums = [2,3,4,7,9]
//
// Output: 2
//
// Explanation:
//
// One optimal sequence of jumps is:
//
// Start at index i = 0. Take an adjacent step to index i = 1.
// At index i = 1, nums[1] = 3 is a prime number. Therefore, we teleport to index i = 4 since nums[4] = 9 is divisible by 3.
// Thus, the answer is 2.
//
// Example 3:
//
// Input: nums = [4,6,5,8]
//
// Output: 3
//
// Explanation:
//
// Since no teleportation is possible, we move through 0 → 1 → 2 → 3. Thus, the answer is 3.
//
// Constraints:
//
// 1 <= n == nums.length <= 10^5
// 1 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        fn fill(is_prime: &mut [bool]) {
            is_prime[0] = false;
            is_prime[1] = false;
            let mut i = 2;
            while i * i <= 1_000_000 {
                if is_prime[i as usize] {
                    let mut j = i * i;
                    while j <= 1_000_000 {
                        is_prime[j as usize] = false;
                        j += i;
                    }
                }
                i += 1;
            }
        }

        let n = nums.len();
        let mut is_prime = vec![true; 1_000_001];
        fill(&mut is_prime);
        let maxi = *nums.iter().max().unwrap();
        let mut mp: std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            mp.entry(num).or_default().push(i);
        }
        let mut dist = vec![-1; n];
        let mut qu = std::collections::VecDeque::new();
        qu.push_back(0);
        dist[0] = 0;
        let mut used = std::collections::HashSet::new();
        while let Some(node) = qu.pop_front() {
            if node > 0 && dist[node - 1] == -1 {
                qu.push_back(node - 1);
                dist[node - 1] = dist[node] + 1;
            }
            if node + 1 < n && dist[node + 1] == -1 {
                qu.push_back(node + 1);
                dist[node + 1] = dist[node] + 1;
            }

            if !is_prime[nums[node] as usize] || used.contains(&nums[node]) {
                continue;
            }

            for tar in (nums[node]..=maxi).step_by(nums[node] as usize) {
                if let Some(indices) = mp.get(&tar) {
                    for &it in indices {
                        if dist[it] != -1 {
                            continue;
                        }
                        qu.push_back(it);
                        if it == n - 1 {
                            return dist[node] + 1;
                        }
                        dist[it] = dist[node] + 1;
                    }
                }
            }

            used.insert(nums[node]);
        }
        dist[n - 1]
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 2, 4, 6];
    assert_eq!(Solution::min_jumps(nums1), 2);

    let nums2 = vec![2, 3, 4, 7, 9];
    assert_eq!(Solution::min_jumps(nums2), 2);

    let nums3 = vec![4, 6, 5, 8];
    assert_eq!(Solution::min_jumps(nums3), 3);
}
