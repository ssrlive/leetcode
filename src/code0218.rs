#![allow(dead_code)]

// 218. The Skyline Problem
// https://leetcode.com/problems/the-skyline-problem/
// https://leetcode.cn/problems/the-skyline-problem/
//
// A city's skyline is the outer contour of the silhouette formed by all the buildings in that city
// when viewed from a distance. Given the locations and heights of all the buildings,
// return the skyline formed by these buildings collectively.
//
// The geometric information of each building is given in the array buildings
// where buildings[i] = [lefti, righti, heighti]:
//
// - lefti is the x coordinate of the left edge of the ith building.
// - righti is the x coordinate of the right edge of the ith building.
// - heighti is the height of the ith building.
//
// You may assume all buildings are perfect rectangles grounded on an absolutely flat surface at height 0.
//
// The skyline should be represented as a list of "key points" sorted by their x-coordinate
// in the form [[x1,y1],[x2,y2],...]. Each key point is the left endpoint of some horizontal segment
// in the skyline except the last point in the list, which always has a y-coordinate 0 and is used
// to mark the skyline's termination where the rightmost building ends.
// Any ground between the leftmost and rightmost buildings should be part of the skyline's contour.
//
// Note: There must be no consecutive horizontal lines of equal height in the output skyline.
// For instance, [...,[2 3],[4 5],[7 5],[11 5],[12 7],...] is not acceptable;
// the three lines of height 5 should be merged into one in the final output as such:
// [...,[2 3],[4 5],[12 7],...]
//
// Example 1:
// Input: buildings = [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]
// Output: [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]
// Explanation:
// Figure A shows the buildings of the input.
// Figure B shows the skyline formed by those buildings. The red points in figure B represent the key points in the output list.
//
// Example 2:
// Input: buildings = [[0,2,3],[2,5,3]]
// Output: [[0,3],[5,0]]
//
// Constraints:
// 1 <= buildings.length <= 104
// 0 <= lefti < righti <= 231 - 1
// 1 <= heighti <= 231 - 1
// buildings is sorted by lefti in non-decreasing order.
//

struct Solution;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = buildings
            .into_iter()
            .flat_map(|v| match &v[..3] {
                [start, end, height] => [(*start, -height), (*end, *height)],
                _ => unreachable!(),
            })
            .collect::<Vec<(_, _)>>();
        points.sort();

        let mut result = Vec::new();

        let mut max_heights = std::collections::btree_map::BTreeMap::new();
        max_heights.insert(0, 1);
        let mut prev_max = 0;
        for (x, h) in points {
            if h > 0 {
                // is end-poiont
                if let std::collections::btree_map::Entry::Occupied(mut entry) = max_heights.entry(h) {
                    *entry.get_mut() -= 1;
                    if *entry.get() == 0 {
                        entry.remove();
                    }
                }
            } else {
                max_heights.entry(-h).and_modify(|count| *count += 1).or_insert(1);
            }

            let (&curr_max, _) = max_heights.iter().next_back().unwrap(); // next_back gets and removes last entry
            if prev_max != curr_max {
                result.push(vec![x, curr_max]);
                prev_max = curr_max;
            }
        }

        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::get_skyline(vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8]
        ]),
        vec![
            vec![2, 10],
            vec![3, 15],
            vec![7, 12],
            vec![12, 0],
            vec![15, 10],
            vec![20, 8],
            vec![24, 0]
        ]
    );
    assert_eq!(
        Solution::get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]),
        vec![vec![0, 3], vec![5, 0]]
    );
    assert_eq!(
        Solution::get_skyline(vec![vec![2, 9, 10], vec![12, 15, 10]]),
        vec![vec![2, 10], vec![9, 0], vec![12, 10], vec![15, 0]]
    );
}
