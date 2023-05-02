#![allow(dead_code)]

/*
// 2659. Make Array Empty
// https://leetcode.com/problems/make-array-empty/
// https://leetcode.cn/problems/make-array-empty/
//
// Hard
//
// You are given an integer array nums containing distinct numbers, and you can perform the following operations until the array is empty:

    If the first element has the smallest value, remove it
    Otherwise, put the first element at the end of the array.

Return an integer denoting the number of operations it takes to make nums empty.

Example 1:

Input: nums = [3,4,-1]
Output: 5

Operation	Array
1	[4, -1, 3]
2	[-1, 3, 4]
3	[3, 4]
4	[4]
5	[]

Example 2:

Input: nums = [1,2,4,3]
Output: 5

Operation	Array
1	[2, 4, 3]
2	[4, 3]
3	[3, 4]
4	[4]
5	[]

Example 3:

Input: nums = [1,2,3]
Output: 3

Operation	Array
1	[2, 3]
2	[3]
3	[]

Constraints:

    1 <= nums.length <= 10^5
    -10^9 <= nums[i] <= 10^9
    All values in nums are distinct.
*/

struct Solution;

impl Solution {
    pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let mut nums = nums;
        let mut map = HashMap::new();
        let n = nums.len() as i64;
        let mut ans = n;

        for (i, &num) in nums.iter().enumerate() {
            map.insert(num, i);
        }

        nums.sort_unstable();
        for i in 1..nums.len() {
            if map[&nums[i]] < map[&nums[i - 1]] {
                ans += n - i as i64;
            }
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_operations_to_empty_array(vec![3, 4, -1]), 5);
    assert_eq!(Solution::count_operations_to_empty_array(vec![1, 2, 4, 3]), 5);
    assert_eq!(Solution::count_operations_to_empty_array(vec![1, 2, 3]), 3);
}
