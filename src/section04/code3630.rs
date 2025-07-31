#![allow(dead_code)]

// 3630. Partition Array for Maximum XOR and AND
// https://leetcode.com/problems/partition-array-for-maximum-xor-and-and/
// https://leetcode.cn/problems/partition-array-for-maximum-xor-and-and/
//
// Hard
//
// You are given an integer array nums.
//
// Partition the array into three (possibly empty) subsequences A, B, and C such that every element of nums belongs to exactly one subsequence.
//
// Your goal is to maximize the value of: XOR(A) + AND(B) + XOR(C)
//
// where:
//
// XOR(arr) denotes the bitwise XOR of all elements in arr. If arr is empty, its value is defined as 0.
// AND(arr) denotes the bitwise AND of all elements in arr. If arr is empty, its value is defined as 0.
// Return the maximum value achievable.
//
// Note: If multiple partitions result in the same maximum sum, you can consider any one of them.
//
// Example 1:
//
// Input: nums = [2,3]
//
// Output: 5
//
// Explanation:
//
// One optimal partition is:
//
// A = [3], XOR(A) = 3
// B = [2], AND(B) = 2
// C = [], XOR(C) = 0
// The maximum value of: XOR(A) + AND(B) + XOR(C) = 3 + 2 + 0 = 5. Thus, the answer is 5.
//
// Example 2:
//
// Input: nums = [1,3,2]
//
// Output: 6
//
// Explanation:
//
// One optimal partition is:
//
// A = [1], XOR(A) = 1
// B = [2], AND(B) = 2
// C = [3], XOR(C) = 3
// The maximum value of: XOR(A) + AND(B) + XOR(C) = 1 + 2 + 3 = 6. Thus, the answer is 6.
//
// Example 3:
//
// Input: nums = [2,3,6,7]
//
// Output: 15
//
// Explanation:
//
// One optimal partition is:
//
// A = [7], XOR(A) = 7
// B = [2,3], AND(B) = 2
// C = [6], XOR(C) = 6
// The maximum value of: XOR(A) + AND(B) + XOR(C) = 7 + 2 + 6 = 15. Thus, the answer is 15.
//
// Constraints:
//
// 1 <= nums.length <= 19
// 1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn maximize_xor_and_xor(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let total_xor = nums.iter().fold(0, |acc, v| acc ^ v);
        let mut res = 0;
        for mask in 0..1 << n {
            let mut select_and = i32::MAX;
            let mut select_xor = 0;
            for (i, &num) in nums.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    select_and &= num;
                    select_xor ^= num;
                }
            }
            if select_and == i32::MAX {
                select_and = 0;
            }
            // With fixed select_and, we want max of
            // x + unselect_xor ^ x == unselect_xor + 2*(x & inverted)
            let unselect_xor = total_xor ^ select_xor;
            // bit_not, i.e 0=>1 1=>0
            // These are the bits that could be used
            let inverted = !unselect_xor;
            // Linear independence
            // i.e for any a, b, c in basis, a^b != c
            // Here it records all bit patterns of (x&inverted)
            let mut basis = vec![];
            for (i, &num) in nums.iter().enumerate() {
                // For each number not in select_and group
                if (mask >> i) & 1 == 0 {
                    // Simplify its bit form
                    let mut reduced = num & inverted;
                    // And further reduce any bit already recorded in basis
                    for b in &basis {
                        reduced = reduced.min(reduced ^ b);
                    }
                    if reduced > 0 {
                        basis.push(reduced);
                    }
                }
            }
            // To find max(x&inverted)
            let mut max_xor = 0;
            for b in &basis {
                max_xor = max_xor.max(max_xor ^ b);
            }
            let curr = i64::from(select_and) + i64::from(unselect_xor) + 2 * i64::from(max_xor);
            res = res.max(curr);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 3];
    assert_eq!(Solution::maximize_xor_and_xor(nums), 5);

    let nums = vec![1, 3, 2];
    assert_eq!(Solution::maximize_xor_and_xor(nums), 6);

    let nums = vec![2, 3, 6, 7];
    assert_eq!(Solution::maximize_xor_and_xor(nums), 15);
}
