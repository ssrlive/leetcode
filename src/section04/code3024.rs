#![allow(dead_code)]

// 3024. Type of Triangle II
// https://leetcode.com/problems/type-of-triangle-ii/
// https://leetcode.cn/problems/type-of-triangle-ii/
//
// Easy
//
// You are given a 0-indexed integer array nums of size 3 which can form the sides of a triangle.
//
// A triangle is called equilateral if it has all sides of equal length.
// A triangle is called isosceles if it has exactly two sides of equal length.
// A triangle is called scalene if all its sides are of different lengths.
// Return a string representing the type of triangle that can be formed or "none" if it cannot form a triangle.
//
// Example 1:
//
// Input: nums = [3,3,3]
// Output: "equilateral"
// Explanation: Since all the sides are of equal length, therefore, it will form an equilateral triangle.
//
// Example 2:
//
// Input: nums = [3,4,5]
// Output: "scalene"
// Explanation:
// nums[0] + nums[1] = 3 + 4 = 7, which is greater than nums[2] = 5.
// nums[0] + nums[2] = 3 + 5 = 8, which is greater than nums[1] = 4.
// nums[1] + nums[2] = 4 + 5 = 9, which is greater than nums[0] = 3.
// Since the sum of the two sides is greater than the third side for all three cases, therefore, it can form a triangle.
// As all the sides are of different lengths, it will form a scalene triangle.
//
// Constraints:
//
// nums.length == 3
// 1 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort();
        if let [x, y, z] = nums.iter().as_slice() {
            match (x + y).cmp(z) {
                std::cmp::Ordering::Greater => {}
                _ => return String::from("none"),
            }
        }
        match nums.iter().collect::<std::collections::HashSet<&i32>>().len() {
            1 => String::from("equilateral"),
            2 => String::from("isosceles"),
            3 => String::from("scalene"),
            _ => unreachable!(),
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::triangle_type(vec![3, 3, 3]), "equilateral");
    assert_eq!(Solution::triangle_type(vec![3, 4, 5]), "scalene");
    assert_eq!(Solution::triangle_type(vec![3, 3, 4]), "isosceles");
}
