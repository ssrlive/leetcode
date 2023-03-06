#![allow(dead_code)]

/*

// 2248. Intersection of Multiple Arrays
// https://leetcode.com/problems/intersection-of-multiple-arrays/
// https://leetcode.cn/problems/intersection-of-multiple-arrays/
//
// Easy
//
// Given a 2D integer array nums where nums[i] is a non-empty array of distinct positive integers, return the list of integers that are present in each array of nums sorted in ascending order.

Example 1:

Input: nums = [[3,1,2,4,5],[1,2,3,4],[3,4,5,6]]
Output: [3,4]
Explanation:
The only integers present in each of nums[0] = [3,1,2,4,5], nums[1] = [1,2,3,4], and nums[2] = [3,4,5,6] are 3 and 4, so we return [3,4].

Example 2:

Input: nums = [[1,2,3],[4,5,6]]
Output: []
Explanation:
There does not exist any integer present both in nums[0] and nums[1], so we return an empty list [].

Constraints:

    1 <= nums.length <= 1000
    1 <= sum(nums[i].length) <= 1000
    1 <= nums[i][j] <= 1000
    All the values of nums[i] are unique.
*/

struct Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let l = nums.len();
        let mut counter = [0; 1001];
        nums.iter()
            .flat_map(|v| v.iter())
            .for_each(|v| counter[*v as usize] += 1);
        counter
            .iter()
            .zip(0..)
            .filter_map(|(&num, idx)| if num == l { Some(idx) } else { None })
            .collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]],
            vec![3, 4],
        ),
        (vec![vec![1, 2, 3], vec![4, 5, 6]], vec![]),
    ];
    for (nums, expect) in cases {
        assert_eq!(Solution::intersection(nums), expect);
    }
}
