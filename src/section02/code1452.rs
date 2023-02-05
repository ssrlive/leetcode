#![allow(dead_code)]

/*
// 1452. People Whose List of Favorite Companies Is Not a Subset of Another List
Medium

Given the array favoriteCompanies where favoriteCompanies[i] is the list of favorites companies for the ith person (indexed from 0).

Return the indices of people whose list of favorite companies is not a subset of any other list of favorites companies. You must return the indices in increasing order.

Example 1:

Input: favoriteCompanies = [["leetcode","google","facebook"],["google","microsoft"],["google","facebook"],["google"],["amazon"]]
Output: [0,1,4]
Explanation:
Person with index=2 has favoriteCompanies[2]=["google","facebook"] which is a subset of favoriteCompanies[0]=["leetcode","google","facebook"] corresponding to the person with index 0.
Person with index=3 has favoriteCompanies[3]=["google"] which is a subset of favoriteCompanies[0]=["leetcode","google","facebook"] and favoriteCompanies[1]=["google","microsoft"].
Other lists of favorite companies are not a subset of another list, therefore, the answer is [0,1,4].

Example 2:

Input: favoriteCompanies = [["leetcode","google","facebook"],["leetcode","amazon"],["facebook","google"]]
Output: [0,1]
Explanation: In this case favoriteCompanies[2]=["facebook","google"] is a subset of favoriteCompanies[0]=["leetcode","google","facebook"], therefore, the answer is [0,1].

Example 3:

Input: favoriteCompanies = [["leetcode"],["google"],["facebook"],["amazon"]]
Output: [0,1,2,3]

Constraints:

    1 <= favoriteCompanies.length <= 100
    1 <= favoriteCompanies[i].length <= 500
    1 <= favoriteCompanies[i][j].length <= 20
    All strings in favoriteCompanies[i] are distinct.
    All lists of favorite companies are distinct, that is, If we sort alphabetically each list then favoriteCompanies[i] != favoriteCompanies[j].
    All strings consist of lowercase English letters only.
*/

struct Solution;

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let mut result = Vec::new();
        let favorite_companies = favorite_companies
            .into_iter()
            .map(|v| v.into_iter().collect::<std::collections::HashSet<_>>())
            .collect::<Vec<_>>();
        for i in 0..favorite_companies.len() {
            let mut is_subset = false;
            for j in 0..favorite_companies.len() {
                if i == j {
                    continue;
                }
                if favorite_companies[i].is_subset(&favorite_companies[j]) {
                    is_subset = true;
                    break;
                }
            }
            if !is_subset {
                result.push(i as i32);
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec!["leetcode", "google", "facebook"],
                vec!["google", "microsoft"],
                vec!["google", "facebook"],
                vec!["google"],
                vec!["amazon"],
            ],
            vec![0, 1, 4],
        ),
        (
            vec![
                vec!["leetcode", "google", "facebook"],
                vec!["leetcode", "amazon"],
                vec!["facebook", "google"],
            ],
            vec![0, 1],
        ),
        (
            vec![vec!["leetcode"], vec!["google"], vec!["facebook"], vec!["amazon"]],
            vec![0, 1, 2, 3],
        ),
    ];
    for (favorite_companies, expected) in cases {
        let favorite_companies = favorite_companies
            .into_iter()
            .map(|v| v.into_iter().map(|s| s.to_string()).collect())
            .collect();
        let result = Solution::people_indexes(favorite_companies);
        assert_eq!(result, expected);
    }
}
