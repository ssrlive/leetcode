#![allow(dead_code)]

// 3605. Minimum Stability Factor of Array
// https://leetcode.com/problems/minimum-stability-factor-of-array/
// https://leetcode.cn/problems/minimum-stability-factor-of-array/
//
// Hard
//
// You are given an integer array nums and an integer maxC.
//
// A subarray is called stable if the highest common factor (HCF) of all its elements is greater than or equal to 2.
//
// The stability factor of an array is defined as the length of its longest stable subarray.
//
// You may modify at most maxC elements of the array to any integer.
//
// Return the minimum possible stability factor of the array after at most maxC modifications. If no stable subarray remains, return 0.
//
// Note:
//
//     The highest common factor (HCF) of an array is the largest integer that evenly divides all the array elements.
//     A subarray of length 1 is stable if its only element is greater than or equal to 2, since HCF([x]) = x.
//
// Example 1:
//
// Input: nums = [3,5,10], maxC = 1
//
// Output: 1
//
// Explanation:
//
//     The stable subarray [5, 10] has HCF = 5, which has a stability factor of 2.
//     Since maxC = 1, one optimal strategy is to change nums[1] to 7, resulting in nums = [3, 7, 10].
//     Now, no subarray of length greater than 1 has HCF >= 2. Thus, the minimum possible stability factor is 1.
//
// Example 2:
//
// Input: nums = [2,6,8], maxC = 2
//
// Output: 1
//
// Explanation:
//
//     The subarray [2, 6, 8] has HCF = 2, which has a stability factor of 3.
//     Since maxC = 2, one optimal strategy is to change nums[1] to 3 and nums[2] to 5, resulting in nums = [2, 3, 5].
//     Now, no subarray of length greater than 1 has HCF >= 2. Thus, the minimum possible stability factor is 1.
//
// Example 3:
//
// Input: nums = [2,4,9,6], maxC = 1
//
// Output: 2
//
// Explanation:
//
//     The stable subarrays are:
//         [2, 4] with HCF = 2 and stability factor of 2.
//         [9, 6] with HCF = 3 and stability factor of 2.
//     Since maxC = 1, the stability factor of 2 cannot be reduced due to two separate stable subarrays. Thus, the minimum possible stability factor is 2.
//
// Constraints:
//
//     1 <= n == nums.length <= 10^5
//     1 <= nums[i] <= 10^9
//     0 <= maxC <= n
//

struct Solution;

impl Solution {
    pub fn min_stable(nums: Vec<i32>, max_c: i32) -> i32 {
        fn get_count(nums: &[i32], n: usize, len: usize) -> i32 {
            let mut req = 0;
            let mut i = 0;
            while i + len - 1 < n {
                let mut gcd = nums[i];
                for &nums_j in nums.iter().take(i + len).skip(i + 1) {
                    if gcd <= 1 {
                        break;
                    }
                    gcd = _gcd(gcd, nums_j);
                }

                if gcd > 1 {
                    req += 1;
                    i += len;
                } else {
                    i += 1;
                }
            }

            req
        }

        fn _gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }

        let n = nums.len();
        let mut l = 0;
        let mut r = n + 1;
        while l + 1 < r {
            let mid = (l + r) / 2;
            if get_count(&nums, n, mid) <= max_c {
                r = mid;
            } else {
                l = mid;
            }
        }

        l as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_stable(vec![3, 5, 10], 1), 1);
    assert_eq!(Solution::min_stable(vec![2, 6, 8], 2), 1);
    assert_eq!(Solution::min_stable(vec![2, 4, 9, 6], 1), 2);
}
