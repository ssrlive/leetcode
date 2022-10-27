#![allow(dead_code)]

// 147. Insertion Sort List
// https://leetcode.com/problems/insertion-sort-list/
//
// Given the head of a singly linked list, sort the list using insertion sort, and return the sorted list's head.
//
// The steps of the insertion sort algorithm:
// - Insertion sort iterates, consuming one input element each repetition and growing a sorted output list.
// - At each iteration, insertion sort removes one element from the input data,
//   finds the location it belongs within the sorted list and inserts it there.
// - It repeats until no input elements remain.
//
// The following is a graphical example of the insertion sort algorithm.
// The partially sorted list (black) initially contains only the first element in the list.
// One element (red) is removed from the input data and inserted in-place into the sorted list with each iteration.
//

use super::listnode::ListNode;

struct Solution;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        match head {
            None => None,
            Some(node) => {
                let mut unsorted = Some(node);
                let mut sorted = ListNode::new(std::i32::MIN);
                while let Some(mut node_to_insert) = unsorted {
                    unsorted = node_to_insert.next.take();
                    let mut sorted_ref = &mut sorted;
                    while sorted_ref.next.is_some()
                        && sorted_ref.next.as_ref().unwrap().val < node_to_insert.val
                    {
                        sorted_ref = sorted_ref.next.as_mut().unwrap()
                    }
                    node_to_insert.next = sorted_ref.next.take();
                    sorted_ref.next = Some(node_to_insert);
                }
                sorted.next
            }
        }
    }
}

#[test]
fn test_insertion_sort_list() {
    let head = ListNode::from_vec(&vec![4, 2, 1, 3]);
    let head = Solution::insertion_sort_list(head).unwrap();
    assert_eq!(head.to_vec(), vec![1, 2, 3, 4]);

    let head = ListNode::from_vec(&vec![-1, 5, 3, 4, 0]);
    let head = Solution::insertion_sort_list(head).unwrap();
    assert_eq!(head.to_vec(), vec![-1, 0, 3, 4, 5]);
}
