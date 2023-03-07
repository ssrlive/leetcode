#![allow(dead_code)]

/*

// 2344. Minimum Deletions to Make Array Divisible
// https://leetcode.com/problems/minimum-deletions-to-make-array-divisible/
// https://leetcode.cn/problems/minimum-deletions-to-make-array-divisible/
//
// Hard
//
// You are given two positive integer arrays nums and numsDivide. You can delete any number of elements from nums.

Return the minimum number of deletions such that the smallest element in nums divides all the elements of numsDivide. If this is not possible, return -1.

Note that an integer x divides y if y % x == 0.

Example 1:

Input: nums = [2,3,2,4,3], numsDivide = [9,6,9,3,15]
Output: 2
Explanation:
The smallest element in [2,3,2,4,3] is 2, which does not divide all the elements of numsDivide.
We use 2 deletions to delete the elements in nums that are equal to 2 which makes nums = [3,4,3].
The smallest element in [3,4,3] is 3, which divides all the elements of numsDivide.
It can be shown that 2 is the minimum number of deletions needed.

Example 2:

Input: nums = [4,3,6], numsDivide = [8,2,6,10]
Output: -1
Explanation:
We want the smallest element in nums to divide all the elements of numsDivide.
There is no way to delete elements from nums to allow this.

Constraints:

    1 <= nums.length, numsDivide.length <= 10^5
    1 <= nums[i], numsDivide[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            let (a, b) = (a.max(b), a.min(b));
            if b == 0 {
                return a;
            }
            gcd(b, a % b)
        }

        let mut d = nums_divide[0];
        for &nums_divide_i in nums_divide.iter().skip(1) {
            d = gcd(d, nums_divide_i);
        }
        let mut count = 0;
        let mut nums = nums;
        nums.sort();
        nums.reverse();
        while !nums.is_empty() {
            if nums[nums.len() - 1] > d || d % nums[nums.len() - 1] != 0 {
                nums.pop();
                count += 1;
            } else {
                break;
            }
        }
        if nums.is_empty() {
            return -1;
        }
        count
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 3, 2, 4, 3], vec![9, 6, 9, 3, 15], 2),
        (vec![4, 3, 6], vec![8, 2, 6, 10], -1),
    ];
    for (nums, nums_divide, expected) in cases {
        assert_eq!(Solution::min_operations(nums, nums_divide), expected);
    }
}
