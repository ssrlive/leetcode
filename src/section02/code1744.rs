#![allow(dead_code)]

/*

// 1744. Can You Eat Your Favorite Candy on Your Favorite Day?
Medium
103
293
Companies

You are given a (0-indexed) array of positive integers candiesCount where candiesCount[i] represents the number of candies of the ith type you have. You are also given a 2D array queries where queries[i] = [favoriteTypei, favoriteDayi, dailyCapi].

You play a game with the following rules:

    You start eating candies on day 0.
    You cannot eat any candy of type i unless you have eaten all candies of type i - 1.
    You must eat at least one candy per day until you have eaten all the candies.

Construct a boolean array answer such that answer.length == queries.length and answer[i] is true if you can eat a candy of type favoriteTypei on day favoriteDayi without eating more than dailyCapi candies on any day, and false otherwise. Note that you can eat different types of candy on the same day, provided that you follow rule 2.

Return the constructed array answer.

Example 1:

Input: candiesCount = [7,4,5,3,8], queries = [[0,2,2],[4,2,4],[2,13,1000000000]]
Output: [true,false,true]
Explanation:
1- If you eat 2 candies (type 0) on day 0 and 2 candies (type 0) on day 1, you will eat a candy of type 0 on day 2.
2- You can eat at most 4 candies each day.
   If you eat 4 candies every day, you will eat 4 candies (type 0) on day 0 and 4 candies (type 0 and type 1) on day 1.
   On day 2, you can only eat 4 candies (type 1 and type 2), so you cannot eat a candy of type 4 on day 2.
3- If you eat 1 candy each day, you will eat a candy of type 2 on day 13.

Example 2:

Input: candiesCount = [5,2,6,4,1], queries = [[3,1,2],[4,10,3],[3,10,100],[4,100,30],[1,3,1]]
Output: [false,true,true,false,false]

Constraints:

    1 <= candiesCount.length <= 10^5
    1 <= candiesCount[i] <= 10^5
    1 <= queries.length <= 10^5
    queries[i].length == 3
    0 <= favoriteTypei < candiesCount.length
    0 <= favoriteDayi <= 10^9
    1 <= dailyCapi <= 10^9
*/

struct Solution;

impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let (m, n) = (candies_count.len(), queries.len());
        let mut sum = vec![0i64; m + 1];
        for i in 0..m {
            sum[i + 1] = sum[i] + candies_count[i] as i64;
        }
        let mut ret = vec![false; n];
        for i in 0..n {
            let (u, v, w) = (queries[i][0], queries[i][1], queries[i][2]);
            if sum[u as usize + 1] <= v as i64 {
                continue;
            }
            if sum[u as usize] - w as i64 >= v as i64 * w as i64 {
                continue;
            }
            ret[i] = true;
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![7, 4, 5, 3, 8],
            vec![vec![0, 2, 2], vec![4, 2, 4], vec![2, 13, 1000000000]],
            vec![true, false, true],
        ),
        (
            vec![5, 2, 6, 4, 1],
            vec![
                vec![3, 1, 2],
                vec![4, 10, 3],
                vec![3, 10, 100],
                vec![4, 100, 30],
                vec![1, 3, 1],
            ],
            vec![false, true, true, false, false],
        ),
    ];
    for (c, q, e) in cases {
        assert_eq!(Solution::can_eat(c, q), e);
    }
}
