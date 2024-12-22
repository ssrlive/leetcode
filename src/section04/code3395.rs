#![allow(dead_code)]

// 3395. Subsequences with a Unique Middle Mode I
// https://leetcode.com/problems/subsequences-with-a-unique-middle-mode-i/
// https://leetcode.cn/problems/subsequences-with-a-unique-middle-mode-i/
//
// Hard
//
// Given an integer array nums, find the number of subsequences of size 5 of nums with a unique middle mode.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// A mode of a sequence of numbers is defined as the element that appears the maximum number of times in the sequence.
//
// A sequence of numbers contains a unique mode if it has only one mode.
//
// A sequence of numbers seq of size 5 contains a unique middle mode if the middle element (seq[2]) is a unique mode.
//
// A subsequence is a non-empty array that can be derived from another array by deleting some
// or no elements without changing the order of the remaining elements.
//
// Example 1:
//
// Input: nums = [1,1,1,1,1,1]
//
// Output: 6
//
// Explanation:
//
// [1, 1, 1, 1, 1] is the only subsequence of size 5 that can be formed, and it has a unique middle mode of 1.
// This subsequence can be formed in 6 different ways, so the output is 6.
//
// Example 2:
//
// Input: nums = [1,2,2,3,3,4]
//
// Output: 4
//
// Explanation:
//
// [1, 2, 2, 3, 4] and [1, 2, 3, 3, 4] each have a unique middle mode because the number at index 2 has the greatest
// frequency in the subsequence. [1, 2, 2, 3, 3] does not have a unique middle mode because 2 and 3 appear twice.
//
// Example 3:
//
// Input: nums = [0,1,2,3,4,5,6,7,8]
//
// Output: 0
//
// Explanation:
//
// There is no subsequence of length 5 with a unique middle mode.
//
// Constraints:
//
// 5 <= nums.length <= 1000
// -10^9 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn subsequences_with_middle_mode(nums: Vec<i32>) -> i32 {
        fn choose2(n: i64) -> i64 {
            n * (n - 1) / 2
        }

        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();

        let mut lmap = std::collections::HashMap::new();
        let mut rmap = std::collections::HashMap::new();
        for x in &nums {
            *rmap.entry(*x).or_insert(0) += 1;
        }
        let n = nums.len() as i64;
        let mut res = 0;
        for mid in 0..n {
            let x = nums[mid as usize];
            *rmap.entry(x).or_insert(0) -= 1;
            let mut add = 0;
            let can_l = mid - lmap.get(&x).unwrap_or(&0);
            let can_r = (n - 1) - mid - rmap.get(&x).unwrap_or(&0);
            let mut tot_r_choose2 = 0;
            for (&e, &f) in &rmap {
                if e != x {
                    tot_r_choose2 += choose2(f);
                }
            }
            for (&e, &_f) in &lmap {
                if e != x {
                    let rchoices = choose2(can_r - rmap.get(&e).unwrap_or(&0)) - (tot_r_choose2 - choose2(*rmap.get(&e).unwrap_or(&0)));
                    add += (lmap.get(&e).unwrap_or(&0) * lmap.get(&x).unwrap_or(&0) * rchoices) % 1000000007;
                    add %= 1000000007;
                }
            }
            res = (res + add) % 1000000007;
            let mut tot_l_choose2 = 0;
            add = 0;
            for (&e, &f) in &lmap {
                if e != x {
                    tot_l_choose2 += choose2(f);
                }
            }
            for (&e, &_f) in &rmap {
                if e != x {
                    let lchoices = choose2(can_l - lmap.get(&e).unwrap_or(&0)) - (tot_l_choose2 - choose2(*lmap.get(&e).unwrap_or(&0)));
                    add += (rmap.get(&e).unwrap_or(&0) * rmap.get(&x).unwrap_or(&0) * lchoices) % 1000000007;
                    add %= 1000000007;
                }
            }
            res = (res + add) % 1000000007;

            add = (lmap.get(&x).unwrap_or(&0) * rmap.get(&x).unwrap_or(&0) * can_l * can_r) % 1000000007;
            res = (res + add) % 1000000007;

            add = (choose2(*lmap.get(&x).unwrap_or(&0)) * choose2(can_r)) % 1000000007;
            res = (res + add) % 1000000007;

            add = (choose2(can_l) * choose2(*rmap.get(&x).unwrap_or(&0))) % 1000000007;
            res = (res + add) % 1000000007;

            add = (choose2(*lmap.get(&x).unwrap_or(&0)) * rmap.get(&x).unwrap_or(&0) * can_r) % 1000000007;
            res = (res + add) % 1000000007;

            add = (can_l * lmap.get(&x).unwrap_or(&0) * choose2(*rmap.get(&x).unwrap_or(&0))) % 1000000007;
            res = (res + add) % 1000000007;

            add = (choose2(*lmap.get(&x).unwrap_or(&0)) * choose2(*rmap.get(&x).unwrap_or(&0))) % 1000000007;
            res = (res + add) % 1000000007;

            *lmap.entry(x).or_insert(0) += 1;
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1, 1, 1, 1];
    let res = 6;
    assert_eq!(Solution::subsequences_with_middle_mode(nums), res);

    let nums = vec![1, 2, 2, 3, 3, 4];
    let res = 4;
    assert_eq!(Solution::subsequences_with_middle_mode(nums), res);

    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let res = 0;
    assert_eq!(Solution::subsequences_with_middle_mode(nums), res);
}
