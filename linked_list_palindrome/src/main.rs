    // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
    impl Solution {
        pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
            /*
             * Linked List -> Vec Parser
             * -------------------------
             *
             * Since the link is given as an Option-wrapped Box, nodes can be checked for validity with a simple `match` statement. As such, there is no real problem with implementing a recursive, indeterminate, and infinite loop with a simple `break` call when None is encountered. Despite recursion, the algorithm retains O(n) linear complexity as the recursion is only used to push the values into a Vector for easier sorting and comparison.
             */

            // The node currently being checked for validity
            let mut curr_node = head;

            // The Vector of `i32` values held in each respective node.
            let mut v: Vec<i32> = Vec::new();

            loop {
                match curr_node {
                    Some(node) => {
                        v.push(node.val); // Push the node's value `v`.
                        curr_node = node.next; // Set the current node to be the next node to push.
                    }
                    None => break // Exit the loop, as the entire linked list has been pushed to the vector.
                }
            }
            // `v` is now populated with all of the values in the singly linked list.
            // Odd lengthed lists require special midpoint handling, and must be handled through an `if` statement somewhere. I chose here for minimal footprint.
            if v.len() % 2 == 1 {
                // This vector holds the latter half of the vector.
                let mut rev_last_half = Vec::new();
                rev_last_half = v[  // Let `rev_last_half` be the values in `v` from...
                    v.len() / 2         // Half of an odd number is quantized down to the floor...
                        + 1                 // so we add 1 here to ensure the range for checking is correct; this is our first bound.
                        ..v.len()       // The total length is the final bound, making a range encapsulating the final half of the vector, not including the middle value.
                    ].to_vec(); // Convert the [i32] to a Vec
                rev_last_half.reverse(); // Reverse it.

                // Then compare to the first half (which doesn't need a `+ 1` because array/vector indexing begins at 0, not 1 like `len()` does).
                if v[0..v.len() / 2] == rev_last_half {return true;}

                false
            } else {
                // This vector holds the latter half of the vector.
                let mut rev_last_half = Vec::new();
                rev_last_half = v[  // Let `rev_last_half` be the values in `v` from...
                    v.len() / 2     // The halfway point...
                        ..v.len()       // ...to the end.
                    ].to_vec();     // Convert the [i32] to a Vec
                rev_last_half.reverse(); // Reverse it.

                // Then compare to the first half (see line 53).
                if v[0..(v.len() / 2)] == rev_last_half {return true;}

                false
            }
        }
    }
