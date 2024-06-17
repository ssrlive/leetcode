#![allow(dead_code)]

/*

// 1515. Best Position for a Service Centre
// https://leetcode.com/problems/best-position-for-a-service-centre/
// https://leetcode.cn/problems/best-position-for-a-service-centre/
//
// Hard
//
// A delivery company wants to build a new service center in a new city. The company knows the positions of all the customers in this city on a 2D-Map and wants to build the new center in a position such that the sum of the euclidean distances to all customers is minimum.

Given an array positions where positions[i] = [xi, yi] is the position of the ith customer on the map, return the minimum sum of the euclidean distances to all customers.

In other words, you need to choose the position of the service center [xcentre, ycentre] such that the following formula is minimized:

Answers within 10-5 of the actual value will be accepted.

Example 1:

Input: positions = [[0,1],[1,0],[1,2],[2,1]]
Output: 4.00000
Explanation: As shown, you can see that choosing [xcentre, ycentre] = [1, 1] will make the distance to each customer = 1, the sum of all distances is 4 which is the minimum possible we can achieve.

Example 2:

Input: positions = [[1,1],[3,3]]
Output: 2.82843
Explanation: The minimum possible sum of distances = sqrt(2) + sqrt(2) = 2.82843

Constraints:

    1 <= positions.length <= 50
    positions[i].length == 2
    0 <= xi, yi <= 100
*/

struct Solution;

impl Solution {
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        fn get_dist_sum(positions: &Vec<Vec<i32>>, x: f64, y: f64) -> f64 {
            let mut sum = 0.0;
            for p in positions {
                sum += ((p[0] as f64 - x).powi(2) + (p[1] as f64 - y).powi(2)).sqrt();
            }
            sum
        }

        let mut x = 0.0;
        let mut y = 0.0;
        for p in &positions {
            x += p[0] as f64;
            y += p[1] as f64;
        }
        x /= positions.len() as f64;
        y /= positions.len() as f64;
        let mut step = 100.0;
        let mut min_dist = f64::MAX;
        while step > 1e-6 {
            let mut found = false;
            for dx in &[-1.0, 0.0, 1.0] {
                for dy in &[-1.0, 0.0, 1.0] {
                    let nx = x + dx * step;
                    let ny = y + dy * step;
                    let dist = get_dist_sum(&positions, nx, ny);
                    if dist < min_dist {
                        min_dist = dist;
                        x = nx;
                        y = ny;
                        found = true;
                    }
                }
            }
            if !found {
                step /= 2.0;
            }
        }
        min_dist
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]], 4.00000),
        (vec![vec![1, 1], vec![3, 3]], 2.82843),
    ];
    for (positions, expect) in cases {
        assert!((Solution::get_min_dist_sum(positions) - expect).abs() < 1e-5);
    }
}
