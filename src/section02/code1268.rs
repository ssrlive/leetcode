#![allow(dead_code)]

// 1268. Search Suggestions System
// https://leetcode.com/problems/search-suggestions-system/
// https://leetcode.cn/problems/search-suggestions-system/
//
// Medium
//
// You are given an array of strings products and a string searchWord.
//
// Design a system that suggests at most three product names from products after each character of searchWord is typed.
// Suggested products should have common prefix with searchWord.
// If there are more than three products with a common prefix return the three lexicographically minimums products.
//
// Return a list of lists of the suggested products after each character of searchWord is typed.
//
// Example 1:
//
// Input: products = ["mobile","mouse","moneypot","monitor","mousepad"], searchWord = "mouse"
// Output: [["mobile","moneypot","monitor"],["mobile","moneypot","monitor"],["mouse","mousepad"],["mouse","mousepad"],["mouse","mousepad"]]
// Explanation: products sorted lexicographically = ["mobile","moneypot","monitor","mouse","mousepad"].
// After typing m and mo all products match and we show user ["mobile","moneypot","monitor"].
// After typing mou, mous and mouse the system suggests ["mouse","mousepad"].
//
// Example 2:
//
// Input: products = ["havana"], searchWord = "havana"
// Output: [["havana"],["havana"],["havana"],["havana"],["havana"],["havana"]]
// Explanation: The only word "havana" will be always suggested while typing the search word.
//
// Constraints:
//
// -    1 <= products.length <= 1000
// -    1 <= products[i].length <= 3000
// -    1 <= sum(products[i].length) <= 2 * 10^4
// -    All the strings of products are unique.
// -    products[i] consists of lowercase English letters.
// -    1 <= searchWord.length <= 1000
// -    searchWord consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort();
        let mut result = vec![];
        let mut prefix = String::new();
        for c in search_word.chars() {
            prefix.push(c);
            let mut r = vec![];
            for product in products.iter() {
                if product.starts_with(&prefix) {
                    r.push(product.clone());
                    if r.len() == 3 {
                        break;
                    }
                }
            }
            result.push(r);
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"],
            "mouse",
            vec![
                vec!["mobile", "moneypot", "monitor"],
                vec!["mobile", "moneypot", "monitor"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
            ],
        ),
        (
            vec!["havana"],
            "havana",
            vec![
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
            ],
        ),
    ];
    for (products, search_word, expected) in cases {
        let products = products.iter().map(|s| s.to_string()).collect();
        let search_word = search_word.to_string();
        let result = Solution::suggested_products(products, search_word);
        assert_eq!(result, expected);
    }
}
