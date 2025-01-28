#![allow(dead_code)]

// 398. Random Pick Index
// https://leetcode.com/problems/random-pick-index/
// https://leetcode.cn/problems/random-pick-index/
//
// Given an integer array nums with possible duplicates, randomly output the index of a given target number.
// You can assume that the given target number must exist in the array.
//
// Implement the Solution class:
//
// Solution(int[] nums) Initializes the object with the array nums.
// int pick(int target) Picks a random index i from nums where nums[i] == target. If there are multiple valid i's,
// then each index should have an equal probability of returning.
//
// Example 1:
//
// Input
// ["Solution", "pick", "pick", "pick"]
// [[[1, 2, 3, 3, 3]], [3], [1], [3]]
// Output
// [null, 4, 0, 2]
//
// Explanation
// Solution solution = new Solution([1, 2, 3, 3, 3]);
// solution.pick(3); // It should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.
// solution.pick(1); // It should return 0. Since in the array only nums[0] is equal to 1.
// solution.pick(3); // It should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.
//
// Constraints:
//
// - 1 <= nums.length <= 2 * 10^4
// - -2^31 <= nums[i] <= 2^31 - 1
// - target is an integer from nums.
// - At most 10^4 calls will be made to pick.
//

struct Solution {
    m: std::collections::HashMap<i32, Vec<i32>>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut m = std::collections::HashMap::<i32, Vec<i32>>::new();
        for (i, &num) in nums.iter().enumerate() {
            m.entry(num).or_default().push(i as i32);
        }
        Self { m }
    }

    fn pick(&self, target: i32) -> i32 {
        let v = self.m.get(&target).unwrap();
        use rand::Rng;
        let x = rand::rng().random_range(0..v.len());
        v[x]
    }
}

// #[test]
// fn test() {
//     let solution = Solution::new(vec![1, 2, 3, 3, 3]);
//     assert_eq!(solution.pick(3), 4);
//     assert_eq!(solution.pick(1), 0);
//     assert_eq!(solution.pick(3), 2);
// }
