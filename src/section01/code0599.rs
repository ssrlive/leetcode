#![allow(dead_code)]

// 599. Minimum Index Sum of Two Lists
// https://leetcode.com/problems/minimum-index-sum-of-two-lists/
// https://leetcode.cn/problems/minimum-index-sum-of-two-lists/
//
// Given two arrays of strings list1 and list2, find the common strings with the least index sum.
//
// A common string is a string that appeared in both list1 and list2.
//
// A common string with the least index sum is a common string such that if it appeared at list1[i] and list2[j] then i + j should be the minimum value among all the other common strings.
//
// Return all the common strings with the least index sum. Return the answer in any order.
//
// Example 1:
//
// Input: list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"]
// Output: ["Shogun"]
// Explanation: The only common string is "Shogun".
//
// Example 2:
//
// Input: list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["KFC","Shogun","Burger King"]
// Output: ["Shogun"]
// Explanation: The common string with the least index sum is "Shogun" with index sum = (0 + 1) = 1.
//
// Example 3:
//
// Input: list1 = ["happy","sad","good"], list2 = ["sad","happy","good"]
// Output: ["sad","happy"]
// Explanation: There are three common strings:
// "happy" with index sum = (0 + 1) = 1.
// "sad" with index sum = (1 + 0) = 1.
// "good" with index sum = (2 + 2) = 4.
// The strings with the least index sum are "sad" and "happy".
//
// Constraints:
//
// - 1 <= list1.length, list2.length <= 1000
// - 1 <= list1[i].length, list2[i].length <= 30
// - list1[i] and list2[i] consist of spaces ' ' and English letters.
// - All the strings of list1 are unique.
// - All the strings of list2 are unique.
//

struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        for (i, s) in list1.iter().enumerate() {
            map.insert(s, i);
        }
        let mut min_sum = usize::MAX;
        let mut result = Vec::new();
        for (i, s) in list2.iter().enumerate() {
            if let Some(&j) = map.get(s) {
                let sum = i + j;
                match sum.cmp(&min_sum) {
                    std::cmp::Ordering::Less => {
                        min_sum = sum;
                        result.clear();
                        result.push(s.clone());
                    }
                    std::cmp::Ordering::Equal => {
                        result.push(s.clone());
                    }
                    _ => {}
                }
            }
        }
        result
    }
}

#[test]
fn test_find_restaurant() {
    assert_eq!(
        Solution::find_restaurant(
            vec![
                "Shogun".to_string(),
                "Tapioca Express".to_string(),
                "Burger King".to_string(),
                "KFC".to_string()
            ],
            vec![
                "Piatti".to_string(),
                "The Grill at Torrey Pines".to_string(),
                "Hungry Hunter Steakhouse".to_string(),
                "Shogun".to_string()
            ]
        ),
        vec!["Shogun".to_string()]
    );
    assert_eq!(
        Solution::find_restaurant(
            vec![
                "Shogun".to_string(),
                "Tapioca Express".to_string(),
                "Burger King".to_string(),
                "KFC".to_string()
            ],
            vec!["KFC".to_string(), "Shogun".to_string(), "Burger King".to_string()]
        ),
        vec!["Shogun".to_string()]
    );
    assert_eq!(
        Solution::find_restaurant(
            vec!["happy".to_string(), "sad".to_string(), "good".to_string()],
            vec!["sad".to_string(), "happy".to_string(), "good".to_string()]
        ),
        vec!["sad".to_string(), "happy".to_string()]
    );
}
