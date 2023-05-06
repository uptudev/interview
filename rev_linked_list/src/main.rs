// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        let mut prev: Option<Box<ListNode>> = None;

        while let Some(mut curr) = head.take() {
            head = std::mem::replace(&mut curr.next, prev);
            prev = Some(curr);
        }

        prev
    }
}

fn main() {

}
