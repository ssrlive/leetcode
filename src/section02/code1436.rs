#![allow(dead_code)]

// 1436. Destination City
// https://leetcode.com/problems/destination-city/
// https://leetcode.cn/problems/destination-city/
//
// Easy
//
// You are given the array paths, where paths[i] = [cityAi, cityBi] means there exists a direct path going from cityAi to cityBi.
// Return the destination city, that is, the city without any path outgoing to another city.
//
// It is guaranteed that the graph of paths forms a line without any loop, therefore, there will be exactly one destination city.
//
// Example 1:
//
// Input: paths = [["London","New York"],["New York","Lima"],["Lima","Sao Paulo"]]
// Output: "Sao Paulo"
// Explanation: Starting at "London" city you will reach "Sao Paulo" city which is the destination city.
// Your trip consist of: "London" -> "New York" -> "Lima" -> "Sao Paulo".
//
// Example 2:
//
// Input: paths = [["B","C"],["D","B"],["C","A"]]
// Output: "A"
// Explanation: All possible trips are:
// "D" -> "B" -> "C" -> "A".
// "B" -> "C" -> "A".
// "C" -> "A".
// "A".
// Clearly the destination city is "A".
//
// Example 3:
//
// Input: paths = [["A","Z"]]
// Output: "Z"
//
// Constraints:
//
// -    1 <= paths.length <= 100
// -    paths[i].length == 2
// -    1 <= cityAi.length, cityBi.length <= 10
// -    cityAi != cityBi
// -    All strings consist of lowercase and uppercase English letters and the space character.
//

struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashSet;
        let out_going_cities: HashSet<_> = paths.iter().map(|path| &path[0]).collect();
        paths.iter().find(|path| !out_going_cities.contains(&path[1])).unwrap()[1].to_owned()
    }
}

#[test]
fn test() {
    let paths = vec![
        vec!["London".to_string(), "New York".to_string()],
        vec!["New York".to_string(), "Lima".to_string()],
        vec!["Lima".to_string(), "Sao Paulo".to_string()],
    ];
    assert_eq!(Solution::dest_city(paths), "Sao Paulo".to_string());
}
