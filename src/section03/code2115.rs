#![allow(dead_code)]

/*

// 2115. Find All Possible Recipes from Given Supplies
// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/
// https://leetcode.cn/problems/find-all-possible-recipes-from-given-supplies/
//
// Medium
//
// You have information about n different recipes. You are given a string array recipes and a 2D string array ingredients. The ith recipe has the name recipes[i], and you can create it if you have all the needed ingredients from ingredients[i]. Ingredients to a recipe may need to be created from other recipes, i.e., ingredients[i] may contain a string that is in recipes.

You are also given a string array supplies containing all the ingredients that you initially have, and you have an infinite supply of all of them.

Return a list of all the recipes that you can create. You may return the answer in any order.

Note that two recipes may contain each other in their ingredients.

Example 1:

Input: recipes = ["bread"], ingredients = [["yeast","flour"]], supplies = ["yeast","flour","corn"]
Output: ["bread"]
Explanation:
We can create "bread" since we have the ingredients "yeast" and "flour".

Example 2:

Input: recipes = ["bread","sandwich"], ingredients = [["yeast","flour"],["bread","meat"]], supplies = ["yeast","flour","meat"]
Output: ["bread","sandwich"]
Explanation:
We can create "bread" since we have the ingredients "yeast" and "flour".
We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".

Example 3:

Input: recipes = ["bread","sandwich","burger"], ingredients = [["yeast","flour"],["bread","meat"],["sandwich","meat","bread"]], supplies = ["yeast","flour","meat"]
Output: ["bread","sandwich","burger"]
Explanation:
We can create "bread" since we have the ingredients "yeast" and "flour".
We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
We can create "burger" since we have the ingredient "meat" and can create the ingredients "bread" and "sandwich".

Constraints:

    n == recipes.length == ingredients.length
    1 <= n <= 100
    1 <= ingredients[i].length, supplies.length <= 100
    1 <= recipes[i].length, ingredients[i][j].length, supplies[k].length <= 10
    recipes[i], ingredients[i][j], and supplies[k] consist only of lowercase English letters.
    All the values of recipes and supplies combined are unique.
    Each ingredients[i] does not contain any duplicate values.
*/

struct Solution;

impl Solution {
    pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        use std::collections::*;
        let mut result = vec![];
        let mut s = supplies.into_iter().collect::<HashSet<String>>();
        let mut map = HashMap::new();
        for i in 0..recipes.len() {
            map.insert(&recipes[i], &ingredients[i]);
        }
        let mut flag = true;
        while flag {
            flag = false;
            let mut target = None;
            for (key, arr) in map.iter() {
                let mut inner_flag = true;
                for v in *arr {
                    if !s.contains(v) {
                        inner_flag = false;
                    }
                }
                if inner_flag {
                    s.insert(key.to_string());
                    result.push(key.to_string());
                    target = Some(key);
                    break;
                }
            }
            if let Some(key) = target {
                map.remove(*key);
                flag = true;
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["bread"],
            vec![vec!["yeast", "flour"]],
            vec!["yeast", "flour", "corn"],
            vec!["bread"],
        ),
        (
            vec!["bread", "sandwich"],
            vec![vec!["yeast", "flour"], vec!["bread", "meat"]],
            vec!["yeast", "flour", "meat"],
            vec!["bread", "sandwich"],
        ),
        (
            vec!["bread", "sandwich", "burger"],
            vec![vec!["yeast", "flour"], vec!["bread", "meat"], vec!["sandwich", "meat", "bread"]],
            vec!["yeast", "flour", "meat"],
            vec!["bread", "sandwich", "burger"],
        ),
    ];
    for (recipes, ingredients, supplies, expect) in cases {
        let recipes = recipes.into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let ingredients = ingredients
            .into_iter()
            .map(|arr| arr.into_iter().map(|s| s.to_string()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        let supplies = supplies.into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expect = expect.into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let result = Solution::find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(result, expect);
    }
}
