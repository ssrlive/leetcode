#![allow(dead_code)]

// 3113. Find the Number of Subarrays Where Boundary Elements Are Maximum
// https://leetcode.com/problems/find-the-number-of-subarrays-where-boundary-elements-are-maximum/
// https://leetcode.cn/problems/find-the-number-of-subarrays-where-boundary-elements-are-maximum/
//
// Hard
//
// You are given an array of positive integers nums.
//
// Return the number of subarrays of nums, where the first and the last elements of the subarray are equal to the largest element in the subarray.
//
// Example 1:
//
// Input: nums = [1,4,3,3,2]
//
// Output: 6
//
// Explanation:
//
// There are 6 subarrays which have the first and the last elements equal to the largest element of the subarray:
//
//     subarray [1,4,3,3,2], with its largest element 1. The first element is 1 and the last element is also 1.
//     subarray [1,4,3,3,2], with its largest element 4. The first element is 4 and the last element is also 4.
//     subarray [1,4,3,3,2], with its largest element 3. The first element is 3 and the last element is also 3.
//     subarray [1,4,3,3,2], with its largest element 3. The first element is 3 and the last element is also 3.
//     subarray [1,4,3,3,2], with its largest element 2. The first element is 2 and the last element is also 2.
//     subarray [1,4,3,3,2], with its largest element 3. The first element is 3 and the last element is also 3.
//
// Hence, we return 6.
//
// Example 2:
//
// Input: nums = [3,3,3]
//
// Output: 6
//
// Explanation:
//
// There are 6 subarrays which have the first and the last elements equal to the largest element of the subarray:
//
//     subarray [3,3,3], with its largest element 3. The first element is 3 and the last element is also 3.
//     subarray [3,3,3], with its largest element 3. The first element is 3 and the last element is also 3.
//     subarray [3,3,3], with its largest element 3. The first element is 3 and the last element is also 3.
//     subarray [3,3,3], with its largest element 3. The first element is 3 and the last element is also 3.
//     subarray [3,3,3], with its largest element 3. The first element is 3 and the last element is also 3.
//     subarray [3,3,3], with its largest element 3. The first element is 3 and the last element is also 3.
//
// Hence, we return 6.
//
// Example 3:
//
// Input: nums = [1]
//
// Output: 1
//
// Explanation:
//
// There is a single subarray of nums which is [1], with its largest element 1. The first element is 1 and the last element is also 1.
//
// Hence, we return 1.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>) -> i64 {
        let mut st: Vec<Vec<i32>> = vec![];
        nums.iter().fold(0i64, |r, n| {
            while !st.is_empty() && *st.last().unwrap().last().unwrap() < *n {
                st.truncate(st.len() - 1)
            }
            if st.is_empty() || *st.last().unwrap().last().unwrap() != *n {
                st.push(vec![0, *n])
            }
            let v = st.last_mut().unwrap().first_mut().unwrap();
            *v += 1;
            r + *v as i64
        })
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_subarrays(vec![1, 4, 3, 3, 2]), 6);
    assert_eq!(Solution::number_of_subarrays(vec![3, 3, 3]), 6);
    assert_eq!(Solution::number_of_subarrays(vec![1]), 1);
}
