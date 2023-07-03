#![allow(dead_code)]

/*

// 1687. Delivering Boxes from Storage to Ports
// https://leetcode.com/problems/delivering-boxes-from-storage-to-ports/
// https://leetcode.cn/problems/delivering-boxes-from-storage-to-ports/
//
// Hard
//
// You have the task of delivering some boxes from storage to their ports using only one ship. However, this ship has a limit on the number of boxes and the total weight that it can carry.

You are given an array boxes, where boxes[i] = [ports​​i​, weighti], and three integers portsCount, maxBoxes, and maxWeight.

    ports​​i is the port where you need to deliver the ith box and weightsi is the weight of the ith box.
    portsCount is the number of ports.
    maxBoxes and maxWeight are the respective box and weight limits of the ship.

The boxes need to be delivered in the order they are given. The ship will follow these steps:

    The ship will take some number of boxes from the boxes queue, not violating the maxBoxes and maxWeight constraints.
    For each loaded box in order, the ship will make a trip to the port the box needs to be delivered to and deliver it. If the ship is already at the correct port, no trip is needed, and the box can immediately be delivered.
    The ship then makes a return trip to storage to take more boxes from the queue.

The ship must end at storage after all the boxes have been delivered.

Return the minimum number of trips the ship needs to make to deliver all boxes to their respective ports.

Example 1:

Input: boxes = [[1,1],[2,1],[1,1]], portsCount = 2, maxBoxes = 3, maxWeight = 3
Output: 4
Explanation: The optimal strategy is as follows:
- The ship takes all the boxes in the queue, goes to port 1, then port 2, then port 1 again, then returns to storage. 4 trips.
So the total number of trips is 4.
Note that the first and third boxes cannot be delivered together because the boxes need to be delivered in order (i.e. the second box needs to be delivered at port 2 before the third box).

Example 2:

Input: boxes = [[1,2],[3,3],[3,1],[3,1],[2,4]], portsCount = 3, maxBoxes = 3, maxWeight = 6
Output: 6
Explanation: The optimal strategy is as follows:
- The ship takes the first box, goes to port 1, then returns to storage. 2 trips.
- The ship takes the second, third and fourth boxes, goes to port 3, then returns to storage. 2 trips.
- The ship takes the fifth box, goes to port 2, then returns to storage. 2 trips.
So the total number of trips is 2 + 2 + 2 = 6.

Example 3:

Input: boxes = [[1,4],[1,2],[2,1],[2,1],[3,2],[3,4]], portsCount = 3, maxBoxes = 6, maxWeight = 7
Output: 6
Explanation: The optimal strategy is as follows:
- The ship takes the first and second boxes, goes to port 1, then returns to storage. 2 trips.
- The ship takes the third and fourth boxes, goes to port 2, then returns to storage. 2 trips.
- The ship takes the fifth and sixth boxes, goes to port 3, then returns to storage. 2 trips.
So the total number of trips is 2 + 2 + 2 = 6.

Constraints:

    1 <= boxes.length <= 10^5
    1 <= portsCount, maxBoxes, maxWeight <= 10^5
    1 <= ports​​i <= portsCount
    1 <= weightsi <= maxWeight
*/

struct Solution;

impl Solution {
    pub fn box_delivering(boxes: Vec<Vec<i32>>, _ports_count: i32, max_boxes: i32, max_weight: i32) -> i32 {
        let n = boxes.len();
        let (mut need, mut j, mut lastj) = (0, 0, 0);
        let (mut max_boxes, mut max_weight) = (max_boxes, max_weight);
        let mut dp = vec![200000; n + 1];
        dp[0] = 0;
        for i in 0..n {
            while j < n && max_boxes > 0 && max_weight >= boxes[j][1] {
                max_boxes -= 1;
                max_weight -= boxes[j][1];
                if j == 0 || boxes[j][0] != boxes[j - 1][0] {
                    lastj = j;
                    need += 1;
                }
                j += 1;
            }
            dp[j] = dp[j].min(dp[i] + need + 1);
            dp[lastj] = dp[lastj].min(dp[i] + need);
            max_boxes += 1;
            max_weight += boxes[i][1];
            if i == n - 1 || boxes[i][0] != boxes[i + 1][0] {
                need -= 1;
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 1], vec![2, 1], vec![1, 1]], 2, 3, 3, 4),
        (vec![vec![1, 2], vec![3, 3], vec![3, 1], vec![3, 1], vec![2, 4]], 3, 3, 6, 6),
        (
            vec![vec![1, 4], vec![1, 2], vec![2, 1], vec![2, 1], vec![3, 2], vec![3, 4]],
            3,
            6,
            7,
            6,
        ),
    ];
    for (boxes, p, m, max_weight, ex) in cases {
        assert_eq!(Solution::box_delivering(boxes, p, m, max_weight), ex);
    }
}
