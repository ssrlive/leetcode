#![allow(dead_code)]

// 3510. Minimum Pair Removal to Sort Array II
// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-ii/
// https://leetcode.cn/problems/minimum-pair-removal-to-sort-array-ii/
//
// Hard
//
// Given an array nums, you can perform the following operation any number of times:
//
//     Select the adjacent pair with the minimum sum in nums. If multiple such pairs exist, choose the leftmost one.
//     Replace the pair with their sum.
//
// Return the minimum number of operations needed to make the array non-decreasing.
//
// An array is said to be non-decreasing if each element is greater than or equal to its previous element (if it exists).
//
// Example 1:
//
// Input: nums = [5,2,3,1]
//
// Output: 2
//
// Explanation:
//
//     The pair (3,1) has the minimum sum of 4. After replacement, nums = [5,2,4].
//     The pair (2,4) has the minimum sum of 6. After replacement, nums = [5,6].
//
// The array nums became non-decreasing in two operations.
//
// Example 2:
//
// Input: nums = [1,2,2]
//
// Output: 0
//
// Explanation:
//
// The array nums is already sorted.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     -10^9 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let n = nums.len() as i64;
        let mut a = nums.clone();
        let mut s = std::collections::BTreeSet::new();
        let mut nxt = vec![0; n as usize];
        let mut pre = vec![0; n as usize];
        for i in 0..n {
            nxt[i as usize] = i + 1;
        }
        for i in 0..n {
            pre[i as usize] = i - 1;
        }
        let mut cnt = 0;
        for i in 0..n - 1 {
            if a[i as usize] > a[i as usize + 1] {
                cnt += 1;
            }
            s.insert((a[i as usize] + a[i as usize + 1], i));
        }
        let mut ans = 0;
        while cnt > 0 {
            let (_sum, i) = s.pop_first().unwrap();
            let j = nxt[i as usize];
            let p = pre[i as usize];
            let q = nxt[j as usize];
            if a[i as usize] > a[j as usize] {
                cnt -= 1;
            }
            if p >= 0 {
                if a[p as usize] > a[i as usize] && a[p as usize] <= a[i as usize] + a[j as usize] {
                    cnt -= 1;
                } else if a[p as usize] <= a[i as usize] && a[p as usize] > a[i as usize] + a[j as usize] {
                    cnt += 1;
                }
            }
            if q < n {
                if a[q as usize] >= a[j as usize] && a[q as usize] < a[i as usize] + a[j as usize] {
                    cnt += 1;
                } else if a[q as usize] < a[j as usize] && a[q as usize] >= a[i as usize] + a[j as usize] {
                    cnt -= 1;
                }
            }
            if p >= 0 {
                s.remove(&(a[p as usize] + a[i as usize], p));
                s.insert((a[p as usize] + a[i as usize] + a[j as usize], p));
            }
            if q < n {
                s.remove(&(a[j as usize] + a[q as usize], j));
                s.insert((a[i as usize] + a[j as usize] + a[q as usize], i));
                pre[q as usize] = i;
            }
            nxt[i as usize] = q;
            a[i as usize] += a[j as usize];
            ans += 1;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
    assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
}
