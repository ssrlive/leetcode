#![allow(dead_code)]

/*

// 2150. Find All Lonely Numbers in the Array
// https://leetcode.com/problems/find-all-lonely-numbers-in-the-array/
// https://leetcode.cn/problems/find-all-lonely-numbers-in-the-array/
//
// Medium
//
// You are given an integer array nums. A number x is lonely when it appears only once, and no adjacent numbers (i.e. x + 1 and x - 1) appear in the array.

Return all lonely numbers in nums. You may return the answer in any order.

Example 1:

Input: nums = [10,6,5,8]
Output: [10,8]
Explanation:
- 10 is a lonely number since it appears exactly once and 9 and 11 does not appear in nums.
- 8 is a lonely number since it appears exactly once and 7 and 9 does not appear in nums.
- 5 is not a lonely number since 6 appears in nums and vice versa.
Hence, the lonely numbers in nums are [10, 8].
Note that [8, 10] may also be returned.

Example 2:

Input: nums = [1,3,5,3]
Output: [1,5]
Explanation:
- 1 is a lonely number since it appears exactly once and 0 and 2 does not appear in nums.
- 5 is a lonely number since it appears exactly once and 4 and 6 does not appear in nums.
- 3 is not a lonely number since it appears twice.
Hence, the lonely numbers in nums are [1, 5].
Note that [5, 1] may also be returned.

Constraints:

    1 <= nums.length <= 10^5
    0 <= nums[i] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for num in &nums {
            *map.entry(num).or_insert(0) += 1;
        }
        map.iter()
            .filter(|&(&key, &val)| {
                let (prev, next) = (key - 1, key + 1);
                val == 1 && !map.contains_key(&prev) && !map.contains_key(&next)
            })
            .map(|(&key, _)| *key)
            .collect()
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![10, 6, 5, 8], vec![8, 10]),
        (vec![1, 3, 5, 3], vec![1, 5]),
    ];
    for (nums, expect) in cases {
        let mut v = Solution::find_lonely(nums);
        v.sort();
        assert_eq!(v, expect);
    }
}
