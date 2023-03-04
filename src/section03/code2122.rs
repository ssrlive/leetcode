#![allow(dead_code)]

/*

// 2122. Recover the Original Array
// https://leetcode.com/problems/recover-the-original-array/
// https://leetcode.cn/problems/recover-the-original-array/
//
// Hard
//
// Alice had a 0-indexed array arr consisting of n positive integers. She chose an arbitrary positive integer k and created two new 0-indexed integer arrays lower and higher in the following manner:

    lower[i] = arr[i] - k, for every index i where 0 <= i < n
    higher[i] = arr[i] + k, for every index i where 0 <= i < n

Unfortunately, Alice lost all three arrays. However, she remembers the integers that were present in the arrays lower and higher, but not the array each integer belonged to. Help Alice and recover the original array.

Given an array nums consisting of 2n integers, where exactly n of the integers were present in lower and the remaining in higher, return the original array arr. In case the answer is not unique, return any valid array.

Note: The test cases are generated such that there exists at least one valid array arr.

Example 1:

Input: nums = [2,10,6,4,8,12]
Output: [3,7,11]
Explanation:
If arr = [3,7,11] and k = 1, we get lower = [2,6,10] and higher = [4,8,12].
Combining lower and higher gives us [2,6,10,4,8,12], which is a permutation of nums.
Another valid possibility is that arr = [5,7,9] and k = 3. In that case, lower = [2,4,6] and higher = [8,10,12].

Example 2:

Input: nums = [1,1,3,3]
Output: [2,2]
Explanation:
If arr = [2,2] and k = 1, we get lower = [1,1] and higher = [3,3].
Combining lower and higher gives us [1,1,3,3], which is equal to nums.
Note that arr cannot be [1,3] because in that case, the only possible way to obtain [1,1,3,3] is with k = 0.
This is invalid since k must be positive.

Example 3:

Input: nums = [5,435]
Output: [220]
Explanation:
The only possible combination is arr = [220] and k = 215. Using them, we get lower = [5] and higher = [435].

Constraints:

    2 * n == nums.length
    1 <= n <= 1000
    1 <= nums[i] <= 10^9
    The test cases are generated such that there exists at least one valid array arr.
*/

struct Solution;

impl Solution {
    pub fn recover_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len() / 2;
        let a = nums[0];
        let mut v1 = Vec::with_capacity(n);
        let mut v2 = Vec::with_capacity(n);
        for i in 1..nums.len() {
            let k = nums[i] - a;
            if k % 2 == 1 || k == 0 || nums[i] == nums[i - 1] {
                continue;
            }
            v1.clear();
            v2.clear();
            v1.push(a);
            let mut x = 0;
            for &num in nums.iter().skip(1) {
                if x < v1.len() && num == v1[x] + k {
                    v2.push(num);
                    x += 1;
                } else {
                    v1.push(num);
                }
                if v1.len() > n || v2.len() > n {
                    break;
                }
            }
            if v1.len() != n || v2.len() != n {
                continue;
            }
            let mut ans = Vec::with_capacity(n);
            for i in 0..n {
                ans.push((v1[i] + v2[i]) / 2);
            }
            return ans;
        }
        vec![]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 10, 6, 4, 8, 12], vec![3, 7, 11]),
        (vec![1, 1, 3, 3], vec![2, 2]),
        (vec![5, 435], vec![220]),
    ];
    for (nums, want) in cases {
        assert_eq!(Solution::recover_array(nums), want);
    }
}
