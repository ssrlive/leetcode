#![allow(dead_code)]

// 3447. Assign Elements to Groups with Constraints
// https://leetcode.com/problems/assign-elements-to-groups-with-constraints/
// https://leetcode.cn/problems/assign-elements-to-groups-with-constraints/
//
// Medium
//
// You are given an integer array groups, where groups[i] represents the size of the ith group. You are also given an integer array elements.
//
// Your task is to assign one element to each group based on the following rules:
//
//     An element j can be assigned to a group i if groups[i] is divisible by elements[j].
//     If there are multiple elements that can be assigned, assign the element with the smallest index j.
//     If no element satisfies the condition for a group, assign -1 to that group.
//
// Return an integer array assigned, where assigned[i] is the index of the element chosen for group i, or -1 if no suitable element exists.
//
// Note: An element may be assigned to more than one group.
//
// Example 1:
//
// Input: groups = [8,4,3,2,4], elements = [4,2]
//
// Output: [0,0,-1,1,0]
//
// Explanation:
//
//     elements[0] = 4 is assigned to groups 0, 1, and 4.
//     elements[1] = 2 is assigned to group 3.
//     Group 2 cannot be assigned any element.
//
// Example 2:
//
// Input: groups = [2,3,5,7], elements = [5,3,3]
//
// Output: [-1,1,0,-1]
//
// Explanation:
//
//     elements[1] = 3 is assigned to group 1.
//     elements[0] = 5 is assigned to group 2.
//     Groups 0 and 3 cannot be assigned any element.
//
// Example 3:
//
// Input: groups = [10,21,30,41], elements = [2,1]
//
// Output: [0,1,0,1]
//
// Explanation:
//
// elements[0] = 2 is assigned to the groups with even values, and elements[1] = 1 is assigned to the groups with odd values.
//
// Constraints:
//
//     1 <= groups.length <= 10^5
//     1 <= elements.length <= 10^5
//     1 <= groups[i] <= 10^5
//     1 <= elements[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn assign_elements(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
        let n = *groups.iter().max().unwrap() as usize;
        let mut element_map = vec![-1; n + 1];
        for (i, element) in elements.into_iter().enumerate() {
            let element = element as usize;
            if element > n {
                continue;
            }
            if element_map[element] >= 0 {
                continue;
            }
            for p in (element..=n).step_by(element) {
                if element_map[p] == -1 {
                    element_map[p] = i as i32;
                }
            }
        }
        groups.into_iter().map(|group| element_map[group as usize]).collect()
    }
}

#[test]
fn test() {
    let groups = vec![8, 4, 3, 2, 4];
    let elements = vec![4, 2];
    let output = vec![0, 0, -1, 1, 0];
    assert_eq!(Solution::assign_elements(groups, elements), output);

    let groups = vec![2, 3, 5, 7];
    let elements = vec![5, 3, 3];
    let output = vec![-1, 1, 0, -1];
    assert_eq!(Solution::assign_elements(groups, elements), output);

    let groups = vec![10, 21, 30, 41];
    let elements = vec![2, 1];
    let output = vec![0, 1, 0, 1];
    assert_eq!(Solution::assign_elements(groups, elements), output);
}
