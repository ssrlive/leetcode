#![allow(dead_code)]

// 3606. Coupon Code Validator
// https://leetcode.com/problems/coupon-code-validator/
// https://leetcode.cn/problems/coupon-code-validator/
//
// Easy
//
// You are given three arrays of length n that describe the properties of n coupons: code, businessLine, and isActive. The ith coupon has:
//
//     code[i]: a string representing the coupon identifier.
//     businessLine[i]: a string denoting the business category of the coupon.
//     isActive[i]: a boolean indicating whether the coupon is currently active.
//
// A coupon is considered valid if all of the following conditions hold:
//
//     code[i] is non-empty and consists only of alphanumeric characters (a-z, A-Z, 0-9) and underscores (_).
//     businessLine[i] is one of the following four categories: "electronics", "grocery", "pharmacy", "restaurant".
//     isActive[i] is true.
//
// Return an array of the codes of all valid coupons, sorted first by their businessLine in the order:
// "electronics", "grocery", "pharmacy", "restaurant", and then by code in lexicographical (ascending) order within each category.
//
// Example 1:
//
// Input: code = ["SAVE20","","PHARMA5","SAVE@20"], businessLine = ["restaurant","grocery","pharmacy","restaurant"], isActive = [true,true,true,true]
//
// Output: ["PHARMA5","SAVE20"]
//
// Explanation:
//
//     First coupon is valid.
//     Second coupon has empty code (invalid).
//     Third coupon is valid.
//     Fourth coupon has special character @ (invalid).
//
// Example 2:
//
// Input: code = ["GROCERY15","ELECTRONICS_50","DISCOUNT10"], businessLine = ["grocery","electronics","invalid"], isActive = [false,true,true]
//
// Output: ["ELECTRONICS_50"]
//
// Explanation:
//
//     First coupon is inactive (invalid).
//     Second coupon is valid.
//     Third coupon has invalid business line (invalid).
//
// Constraints:
//
//     n == code.length == businessLine.length == isActive.length
//     1 <= n <= 100
//     0 <= code[i].length, businessLine[i].length <= 100
//     code[i] and businessLine[i] consist of printable ASCII characters.
//     isActive[i] is either true or false.

struct Solution;

impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        fn is_valid(s: &str) -> bool {
            if s.is_empty() {
                return false;
            }
            for ch in s.chars() {
                if !ch.is_alphanumeric() && ch != '_' {
                    return false;
                }
            }
            true
        }

        let mut arr: Vec<(String, String)> = Vec::new();
        for (i, c) in code.iter().enumerate() {
            if is_valid(c)
                && (business_line[i] == "electronics"
                    || business_line[i] == "grocery"
                    || business_line[i] == "pharmacy"
                    || business_line[i] == "restaurant")
                && is_active[i]
            {
                arr.push((business_line[i].clone(), c.clone()));
            }
        }
        arr.sort();
        arr.into_iter().map(|(_, code)| code).collect()
    }
}

#[test]
fn test() {
    let code = ["SAVE20", "", "PHARMA5", "SAVE@20"].iter().map(|s| String::from(*s)).collect();
    let business_line = ["restaurant", "grocery", "pharmacy", "restaurant"]
        .iter()
        .map(|s| String::from(*s))
        .collect();
    let is_active = vec![true, true, true, true];
    let result = Solution::validate_coupons(code, business_line, is_active);
    assert_eq!(result, vec!["PHARMA5", "SAVE20"]);

    let code = ["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"]
        .iter()
        .map(|s| String::from(*s))
        .collect();
    let business_line = ["grocery", "electronics", "invalid"].iter().map(|s| String::from(*s)).collect();
    let is_active = vec![false, true, true];
    let result = Solution::validate_coupons(code, business_line, is_active);
    assert_eq!(result, vec!["ELECTRONICS_50"]);
}
