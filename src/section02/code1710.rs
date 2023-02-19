#![allow(dead_code)]

/*

// 1710. Maximum Units on a Truck
// https://leetcode.com/problems/maximum-units-on-a-truck/
// https://leetcode.cn/problems/maximum-units-on-a-truck/
//
// Easy
//
// You are assigned to put some amount of boxes onto one truck. You are given a 2D array boxTypes, where boxTypes[i] = [numberOfBoxesi, numberOfUnitsPerBoxi]:

    numberOfBoxesi is the number of boxes of type i.
    numberOfUnitsPerBoxi is the number of units in each box of the type i.

You are also given an integer truckSize, which is the maximum number of boxes that can be put on the truck. You can choose any boxes to put on the truck as long as the number of boxes does not exceed truckSize.

Return the maximum total number of units that can be put on the truck.

Example 1:

Input: boxTypes = [[1,3],[2,2],[3,1]], truckSize = 4
Output: 8
Explanation: There are:
- 1 box of the first type that contains 3 units.
- 2 boxes of the second type that contain 2 units each.
- 3 boxes of the third type that contain 1 unit each.
You can take all the boxes of the first and second types, and one box of the third type.
The total number of units will be = (1 * 3) + (2 * 2) + (1 * 1) = 8.

Example 2:

Input: boxTypes = [[5,10],[2,5],[4,7],[3,9]], truckSize = 10
Output: 91

Constraints:

    1 <= boxTypes.length <= 1000
    1 <= numberOfBoxesi, numberOfUnitsPerBoxi <= 1000
    1 <= truckSize <= 10^6
*/

struct Solution;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut result = 0;
        let mut truck_size = truck_size;
        for box_type in box_types {
            let n = box_type[0];
            let u = box_type[1];
            if n <= truck_size {
                result += n * u;
                truck_size -= n;
            } else {
                result += truck_size * u;
                break;
            }
        }
        result
    }
}

#[test]
fn test() {
    let box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
    let truck_size = 4;
    let result = 8;
    assert_eq!(Solution::maximum_units(box_types, truck_size), result);
    let box_types = vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]];
    let truck_size = 10;
    let result = 91;
    assert_eq!(Solution::maximum_units(box_types, truck_size), result);
}
