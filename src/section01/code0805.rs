#![allow(dead_code)]

// 805. Split Array With Same Average
// https://leetcode.com/problems/split-array-with-same-average/
// https://leetcode.cn/problems/split-array-with-same-average/
//
// You are given an integer array nums.
//
// You should move each element of nums into one of the two arrays A and B such that A and B are non-empty, and average(A) == average(B).
//
// Return true if it is possible to achieve that and false otherwise.
//
// Note that for an array arr, average(arr) is the sum of all the elements of arr over the length of arr.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5,6,7,8]
// Output: true
// Explanation: We can split the array into [1,4,5,8] and [2,3,6,7], and both of them have an average of 4.5.
//
// Example 2:
//
// Input: nums = [3,1]
// Output: false
//
// Constraints:
//
// - 1 <= nums.length <= 30
// - 0 <= nums[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;

        fn sums(i: i32, j: i32, target: i32, nums: &[i32], mem: &mut HashMap<(i32, i32, i32), bool>) -> bool {
            if let Some(verasity) = mem.get(&(i, j, target)) {
                return *verasity;
            }
            let mut _i = i as usize;
            let mut _j = j as usize;
            while _i < _j {
                match target.cmp(&(nums[_i] + nums[_j])) {
                    std::cmp::Ordering::Equal => {
                        mem.insert((i, j, target), true);
                        return true;
                    }
                    std::cmp::Ordering::Greater => {
                        _i += 1;
                    }
                    std::cmp::Ordering::Less => {
                        _j -= 1;
                    }
                }
            }
            mem.insert((i, j, target), false);
            false
        }

        fn nsums(
            ct: i32,
            i: i32,
            j: i32,
            target: i32,
            nums: &Vec<i32>,
            mem: &mut HashMap<(i32, i32, i32), bool>,
            mem2: &mut HashMap<(i32, i32, i32, i32), bool>,
        ) -> bool {
            if let Some(verasity) = mem2.get(&(ct, i, j, target)) {
                return *verasity;
            }
            if ct == 2 {
                return sums(i, j, target, nums, mem);
            } else {
                for k in (i as usize)..(nums.len() - 2) {
                    if nsums(ct - 1, (k + 1) as i32, j, target - nums[k], nums, mem, mem2) {
                        mem2.insert((ct, i, j, target), true);
                        return true;
                    }
                }
            }
            mem2.insert((ct, i, j, target), false);
            false
        }

        let mut snums = nums.to_owned();
        if nums.len() == 1 {
            return false;
        }

        let sm: i32 = nums.iter().sum();

        let mean: i32 = ((nums.iter().sum::<i32>() as f64) / nums.len() as f64) as i32;

        if sm % (nums.len() as i32) == 0 && nums.contains(&mean) {
            return true;
        }

        if nums.len() == 2 {
            return false;
        }
        snums.sort();

        let mut mem = HashMap::new();
        let mut mem2 = HashMap::new();

        for ct in 2..=((nums.len() / 2 + 1) as i32) {
            if (sm * ct) % (nums.len() as i32) == 0
                && nsums(
                    ct,
                    0,
                    (nums.len() - 1) as i32,
                    (sm * ct) / (nums.len() as i32),
                    &snums,
                    &mut mem,
                    &mut mem2,
                )
            {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(Solution::split_array_same_average(nums), true);

    let nums = vec![3, 1];
    assert_eq!(Solution::split_array_same_average(nums), false);

    let nums = vec![18, 10, 5, 3];
    assert_eq!(Solution::split_array_same_average(nums), false);
}
