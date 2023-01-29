#![allow(dead_code)]

// 1418. Display Table of Food Orders in a Restaurant
// https://leetcode.com/problems/display-table-of-food-orders-in-a-restaurant/
// https://leetcode.cn/problems/display-table-of-food-orders-in-a-restaurant/
//
// Medium
//
// Given the array orders, which represents the orders that customers have done in a restaurant.
// More specifically orders[i]=[customerNamei,tableNumberi,foodItemi] where customerNamei is the name of the customer,
// tableNumberi is the table customer sit at, and foodItemi is the item customer orders.
//
// Return the restaurant's “display table”. The “display table” is a table whose row entries denote
// how many of each food item each table ordered. The first column is the table number and the remaining columns
// correspond to each food item in alphabetical order. The first row should be a header whose first column is “Table”,
// followed by the names of the food items. Note that the customer names are not part of the table.
// Additionally, the rows should be sorted in numerically increasing order.
//
// Example 1:
//
// Input: orders = [["David","3","Ceviche"],["Corina","10","Beef Burrito"],["David","3","Fried Chicken"],
//                  ["Carla","5","Water"],["Carla","5","Ceviche"],["Rous","3","Ceviche"]]
// Output: [["Table","Beef Burrito","Ceviche","Fried Chicken","Water"],["3","0","2","1","0"],["5","0","1","0","1"],["10","1","0","0","0"]]
// Explanation:
// The displaying table looks like:
// Table,Beef Burrito,Ceviche,Fried Chicken,Water
// 3    ,0           ,2      ,1            ,0
// 5    ,0           ,1      ,0            ,1
// 10   ,1           ,0      ,0            ,0
// For the table 3: David orders "Ceviche" and "Fried Chicken", and Rous orders "Ceviche".
// For the table 5: Carla orders "Water" and "Ceviche".
// For the table 10: Corina orders "Beef Burrito".
//
// Example 2:
//
// Input: orders = [["James","12","Fried Chicken"],["Ratesh","12","Fried Chicken"],["Amadeus","12","Fried Chicken"],
//                  ["Adam","1","Canadian Waffles"],["Brianna","1","Canadian Waffles"]]
// Output: [["Table","Canadian Waffles","Fried Chicken"],["1","2","0"],["12","0","3"]]
// Explanation:
// For the table 1: Adam and Brianna order "Canadian Waffles".
// For the table 12: James, Ratesh and Amadeus order "Fried Chicken".
//
// Example 3:
//
// Input: orders = [["Laura","2","Bean Burrito"],["Jhon","2","Beef Burrito"],["Melissa","2","Soda"]]
// Output: [["Table","Bean Burrito","Beef Burrito","Soda"],["2","1","1","1"]]
//
// Constraints:
//
// -    1 <= orders.length <= 5 * 10^4
// -    orders[i].length == 3
// -    1 <= customerNamei.length, foodItemi.length <= 20
// -    customerNamei and foodItemi consist of lowercase and uppercase English letters and the space character.
// -    tableNumberi is a valid integer between 1 and 500.
//

struct Solution;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        use std::collections::{BTreeMap, BTreeSet};
        let mut tables = BTreeMap::new();
        let mut foods = BTreeSet::new();
        for order in orders {
            let table = order[1].parse::<i32>().unwrap();
            let food = order[2].clone();
            foods.insert(food.clone());
            tables
                .entry(table)
                .or_insert(BTreeMap::new())
                .entry(food)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        let mut result = vec![vec!["Table".to_string()]];
        for food in foods {
            result[0].push(food);
        }
        for (table, mut food_count) in tables {
            let mut row = vec![table.to_string()];
            for food in &result[0][1..] {
                row.push(food_count.remove(food).unwrap_or(0).to_string());
            }
            result.push(row);
        }
        result
    }
}

#[test]
fn test() {
    let orders = vec![
        vec!["David".to_string(), "3".to_string(), "Ceviche".to_string()],
        vec!["Corina".to_string(), "10".to_string(), "Beef Burrito".to_string()],
        vec!["David".to_string(), "3".to_string(), "Fried Chicken".to_string()],
        vec!["Carla".to_string(), "5".to_string(), "Water".to_string()],
        vec!["Carla".to_string(), "5".to_string(), "Ceviche".to_string()],
        vec!["Rous".to_string(), "3".to_string(), "Ceviche".to_string()],
    ];
    let result = Solution::display_table(orders);
    let expected = vec![
        vec![
            "Table".to_string(),
            "Beef Burrito".to_string(),
            "Ceviche".to_string(),
            "Fried Chicken".to_string(),
            "Water".to_string(),
        ],
        vec![
            "3".to_string(),
            "0".to_string(),
            "2".to_string(),
            "1".to_string(),
            "0".to_string(),
        ],
        vec![
            "5".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
            "1".to_string(),
        ],
        vec![
            "10".to_string(),
            "1".to_string(),
            "0".to_string(),
            "0".to_string(),
            "0".to_string(),
        ],
    ];
    assert_eq!(result, expected);

    let orders = vec![
        vec!["James".to_string(), "12".to_string(), "Fried Chicken".to_string()],
        vec!["Ratesh".to_string(), "12".to_string(), "Fried Chicken".to_string()],
        vec!["Amadeus".to_string(), "12".to_string(), "Fried Chicken".to_string()],
        vec!["Adam".to_string(), "1".to_string(), "Canadian Waffles".to_string()],
        vec!["Brianna".to_string(), "1".to_string(), "Canadian Waffles".to_string()],
    ];
    let result = Solution::display_table(orders);
    let expected = vec![
        vec![
            "Table".to_string(),
            "Canadian Waffles".to_string(),
            "Fried Chicken".to_string(),
        ],
        vec!["1".to_string(), "2".to_string(), "0".to_string()],
        vec!["12".to_string(), "0".to_string(), "3".to_string()],
    ];
    assert_eq!(result, expected);

    let orders = vec![
        vec!["Laura".to_string(), "2".to_string(), "Bean Burrito".to_string()],
        vec!["Jhon".to_string(), "2".to_string(), "Beef Burrito".to_string()],
        vec!["Melissa".to_string(), "2".to_string(), "Soda".to_string()],
    ];
    let result = Solution::display_table(orders);
    let expected = vec![
        vec![
            "Table".to_string(),
            "Bean Burrito".to_string(),
            "Beef Burrito".to_string(),
            "Soda".to_string(),
        ],
        vec!["2".to_string(), "1".to_string(), "1".to_string(), "1".to_string()],
    ];
    assert_eq!(result, expected);
}
