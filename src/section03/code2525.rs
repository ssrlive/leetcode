#![allow(dead_code)]

// 2525. Categorize Box According to Criteria
// https://leetcode.com/problems/categorize-box-according-to-criteria/
// https://leetcode.cn/problems/categorize-box-according-to-criteria/
//
// Given four integers length, width, height, and mass, representing the dimensions and mass of a box,
// respectively, return a string representing the category of the box.
//
// The box is "Bulky" if:
// Any of the dimensions of the box is greater or equal to 104.
// Or, the volume of the box is greater or equal to 109.
// If the mass of the box is greater or equal to 100, it is "Heavy".
// If the box is both "Bulky" and "Heavy", then its category is "Both".
// If the box is neither "Bulky" nor "Heavy", then its category is "Neither".
// If the box is "Bulky" but not "Heavy", then its category is "Bulky".
// If the box is "Heavy" but not "Bulky", then its category is "Heavy".
// Note that the volume of the box is the product of its length, width and height.
//
// Example 1:
//
// Input: length = 1000, width = 35, height = 700, mass = 300
// Output: "Heavy"
// Explanation:
// None of the dimensions of the box is greater or equal to 104.
// Its volume = 24500000 <= 109. So it cannot be categorized as "Bulky".
// However mass >= 100, so the box is "Heavy".
// Since the box is not "Bulky" but "Heavy", we return "Heavy".
//
// Example 2:
//
// Input: length = 200, width = 50, height = 800, mass = 50
// Output: "Neither"
// Explanation:
// None of the dimensions of the box is greater or equal to 104.
// Its volume = 8 * 106 <= 109. So it cannot be categorized as "Bulky".
// Its mass is also less than 100, so it cannot be categorized as "Heavy" either.
// Since its neither of the two above categories, we return "Neither".
//
// Constraints:
//
// - 1 <= length, width, height <= 10^5
// - 1 <= mass <= 10^3
//

struct Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        const MAX_MASS: i32 = 100;
        const MAX_DIM: i32 = 10_000;
        const MAX_VOL: i64 = 1_000_000_000;
        let is_bulky = length >= MAX_DIM
            || width >= MAX_DIM
            || height >= MAX_DIM
            || (length as i64) * (width as i64) * (height as i64) >= MAX_VOL;

        let is_heavy = mass >= MAX_MASS;
        match (is_bulky, is_heavy) {
            (true, true) => "Both".to_string(),
            (true, false) => "Bulky".to_string(),
            (false, true) => "Heavy".to_string(),
            _ => "Neither".to_string(),
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (1000, 35, 700, 300, "Heavy".to_string()),
        (200, 50, 800, 50, "Neither".to_string()),
    ];
    for (length, width, height, mass, expected) in cases {
        assert_eq!(Solution::categorize_box(length, width, height, mass), expected);
    }
}
