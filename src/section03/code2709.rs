#![allow(dead_code)]

// 2709. Greatest Common Divisor Traversal
// https://leetcode.com/problems/greatest-common-divisor-traversal/
// https://leetcode.cn/problems/greatest-common-divisor-traversal/
//
// Hard
//
// You are given a 0-indexed integer array nums, and you are allowed to traverse
// between its indices. You can traverse between index i and index j, i != j, if and
// only if gcd(nums[i], nums[j]) > 1, where gcd is the greatest common divisor.
//
// Your task is to determine if for every pair of indices i and j in nums, where i < j,
// there exists a sequence of traversals that can take us from i to j.
//
// Return true if it is possible to traverse between all such pairs of indices, or false otherwise.
//
// Example 1:
//
// Input: nums = [2,3,6]
// Output: true
// Explanation: In this example, there are 3 possible pairs of indices: (0, 1), (0, 2), and (1, 2).
// - To go from index 0 to index 1, we can use the sequence of traversals 0 -> 2 -> 1, where we move from
//   index 0 to index 2 because gcd(nums[0], nums[2]) = gcd(2, 6) = 2 > 1, and then move from index 2 to
//   index 1 because gcd(nums[2], nums[1]) = gcd(6, 3) = 3 > 1.
// - To go from index 0 to index 2, we can just go directly because gcd(nums[0], nums[2]) = gcd(2, 6) = 2 > 1.
//   Likewise, to go from index 1 to index 2, we can just go directly because gcd(nums[1], nums[2]) = gcd(3, 6) = 3 > 1.
//
// Example 2:
//
// Input: nums = [3,9,5]
// Output: false
// Explanation: No sequence of traversals can take us from index 0 to index 2 in this example. So, we return false.
//
// Example 3:
//
// Input: nums = [4,3,12,8]
// Output: true
// Explanation: There are 6 possible pairs of indices to traverse between: (0, 1), (0, 2), (0, 3), (1, 2), (1, 3), and (2, 3).
// A valid sequence of traversals exists for each pair, so we return true.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut parent = (0..n).collect::<Vec<usize>>();
        let mut mp = std::collections::HashMap::<i32, usize>::new();

        for (i, &num) in nums.iter().enumerate().take(n) {
            let mut a = num;
            if a == 1 {
                continue;
            }

            if let Some(j) = mp.get(&a) {
                Self::union_join(i, *j, &mut parent);
            }
            mp.insert(a, i);

            let x = a;
            for b in 2..x {
                if b as i64 * b as i64 > nums[i] as i64 {
                    break;
                }
                if a % b != 0 {
                    continue;
                }

                if let Some(j) = mp.get(&b) {
                    Self::union_join(i, *j, &mut parent);
                }
                mp.insert(b, i);
                while a % b == 0 {
                    a /= b;
                }
                if a == 1 {
                    break;
                }
                if let Some(j) = mp.get(&a) {
                    Self::union_join(i, *j, &mut parent);
                }
                mp.insert(a, i);
            }
        }

        let mut s = std::collections::HashSet::new();
        for i in 0..n {
            s.insert(Self::find(i, &mut parent));
        }

        s.len() == 1
    }

    fn find(i: usize, p: &mut Vec<usize>) -> usize {
        if p[i] == i {
            return i;
        }
        p[i] = Self::find(p[i], p);
        p[i]
    }

    fn union_join(i: usize, j: usize, p: &mut Vec<usize>) {
        let (u, v) = (Self::find(i, p), Self::find(j, p));
        p[u] = v;
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 6];
    assert!(Solution::can_traverse_all_pairs(nums));

    let nums = vec![3, 9, 5];
    assert!(!Solution::can_traverse_all_pairs(nums));

    let nums = vec![4, 3, 12, 8];
    assert!(Solution::can_traverse_all_pairs(nums));
}
