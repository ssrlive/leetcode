mod listnode;
mod listnode2;
mod narytree;
mod quadtree;
mod treenode;

mod section01;
mod section02;
mod section03;

fn main() {
    let mut problems = vec![
        (39, "Combination Sum", "https://leetcode.cn/problems/combination-sum/"),
        (
            40,
            "Combination Sum II",
            "https://leetcode.cn/problems/combination-sum-ii/",
        ),
        (78, "Subsets", "https://leetcode.cn/problems/subsets/"),
        (90, "Subsets II", "https://leetcode.cn/problems/subsets-ii/"),
        (46, "Permutations", "https://leetcode.cn/problems/permutations/"),
        (47, "Permutations II", "https://leetcode.cn/problems/permutations-ii/"),
        (133, "Clone Graph", "https://leetcode.cn/problems/clone-graph/"),
        (127, "Word Ladder", "https://leetcode.cn/problems/word-ladder/"),
        (490, "The Maze", "https://leetcode.cn/problems/the-maze/"),
        (
            210,
            "Course Schedule II",
            "https://leetcode.cn/problems/course-schedule-ii/",
        ),
        (
            269,
            "Alien-Dictionary",
            "https://leetcode.cn/problems/alien-dictionary/",
        ),
        (
            94,
            "Binary Tree Inorder Traversal",
            "https://leetcode.cn/problems/binary-tree-inorder-traversal/",
        ),
        (
            236,
            "Lowest Common Ancestor of a Binary Tree",
            "https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/",
        ),
        (
            297,
            "Serialize and Deserialize Binary Tree",
            "https://leetcode.cn/problems/serialize-and-deserialize-binary-tree/",
        ),
        (
            102,
            "Binary Tree Level Order Traversal",
            "https://leetcode.cn/problems/binary-tree-level-order-traversal/",
        ),
        (
            98,
            "Validate Binary Search Tree",
            "https://leetcode.cn/problems/validate-binary-search-tree/",
        ),
        (
            34,
            "Find First and Last Position of Element in Sorted Array",
            "https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/",
        ),
        (
            162,
            "Find Peak Element",
            "https://leetcode.cn/problems/find-peak-element/",
        ),
        (69, "Sqrt(x)", "https://leetcode.cn/problems/sqrtx/"),
        (242, "Valid Anagram", "https://leetcode.cn/problems/valid-anagram/"),
        (155, "Min Stack", "https://leetcode.cn/problems/min-stack/"),
        (
            225,
            "Implement Stack using Queues",
            "https://leetcode.cn/problems/implement-stack-using-queues/",
        ),
        (
            215,
            "Kth Largest Element in an Array",
            "https://leetcode.cn/problems/kth-largest-element-in-an-array/",
        ),
        (
            23,
            "Merge k Sorted Lists",
            "https://leetcode.cn/problems/merge-k-sorted-lists/",
        ),
        (
            237,
            "Delete Node in a Linked List",
            "https://leetcode.cn/problems/delete-node-in-a-linked-list/",
        ),
        (
            92,
            "Reverse Linked List II",
            "https://leetcode.cn/problems/reverse-linked-list-ii/",
        ),
        (
            876,
            "Middle of the Linked List",
            "https://leetcode.cn/problems/middle-of-the-linked-list/",
        ),
        (143, "Reorder List", "https://leetcode.cn/problems/reorder-list/"),
        (
            239,
            "Sliding Window Maximum",
            "https://leetcode.cn/problems/sliding-window-maximum/",
        ),
        (
            3,
            "Longest Substring Without Repeating Characters",
            "https://leetcode.cn/problems/longest-substring-without-repeating-characters/",
        ),
        (
            76,
            "Minimum Window Substring",
            "https://leetcode.cn/problems/minimum-window-substring/",
        ),
        (148, "Sort List", "https://leetcode.cn/problems/sort-list/"),
        (146, "LRU Cache", "https://leetcode.cn/problems/lru-cache/"),
        (
            1066,
            "Campus Bikes II",
            "https://leetcode.com/problems/campus-bikes-ii/",
        ),
    ];
    problems.sort_by(|a, b| a.0.cmp(&b.0));
    let max_lengh = problems.iter().map(|x| x.1.len()).max().unwrap();
    for (i, (num, name, url)) in problems.iter().enumerate() {
        let padding = " ".repeat(max_lengh - name.len());
        println!("{:>2?}: {:>4?} \"{}\" {}{}", i, num, name, padding, url);
    }
}
