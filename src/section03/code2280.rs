#![allow(dead_code)]

/*

// 2280. Minimum Lines to Represent a Line Chart
// https://leetcode.com/problems/minimum-lines-to-represent-a-line-chart/
// https://leetcode.cn/problems/minimum-lines-to-represent-a-line-chart/
//
// Medium
//
// You are given a 2D integer array stockPrices where stockPrices[i] = [dayi, pricei] indicates the price of the stock on day dayi is pricei. A line chart is created from the array by plotting the points on an XY plane with the X-axis representing the day and the Y-axis representing the price and connecting adjacent points. One such example is shown below:

Return the minimum number of lines needed to represent the line chart.

Example 1:

Input: stockPrices = [[1,7],[2,6],[3,5],[4,4],[5,4],[6,3],[7,2],[8,1]]
Output: 3
Explanation:
The diagram above represents the input, with the X-axis representing the day and Y-axis representing the price.
The following 3 lines can be drawn to represent the line chart:
- Line 1 (in red) from (1,7) to (4,4) passing through (1,7), (2,6), (3,5), and (4,4).
- Line 2 (in blue) from (4,4) to (5,4).
- Line 3 (in green) from (5,4) to (8,1) passing through (5,4), (6,3), (7,2), and (8,1).
It can be shown that it is not possible to represent the line chart using less than 3 lines.

Example 2:

Input: stockPrices = [[3,4],[1,2],[7,8],[2,3]]
Output: 1
Explanation:
As shown in the diagram above, the line chart can be represented with a single line.

Constraints:

    1 <= stockPrices.length <= 10^5
    stockPrices[i].length == 2
    1 <= dayi, pricei <= 10^9
    All dayi are distinct.
*/

struct Solution;

impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        let mut stock_prices = stock_prices;
        if stock_prices.len() == 1 {
            return 0;
        }
        if stock_prices.len() == 2 {
            return 1;
        }
        stock_prices.sort_unstable();
        let mut res = 1;
        for i in stock_prices.windows(3) {
            let left = (i[1][1] - i[0][1]) as i64 * (i[2][0] - i[1][0]) as i64;
            let right = (i[1][0] - i[0][0]) as i64 * (i[2][1] - i[1][1]) as i64;
            if left != right {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                vec![1, 7],
                vec![2, 6],
                vec![3, 5],
                vec![4, 4],
                vec![5, 4],
                vec![6, 3],
                vec![7, 2],
                vec![8, 1],
            ],
            3,
        ),
        (vec![vec![3, 4], vec![1, 2], vec![7, 8], vec![2, 3]], 1),
    ];
    for (stock_prices, expected) in cases {
        assert_eq!(Solution::minimum_lines(stock_prices), expected);
    }
}
