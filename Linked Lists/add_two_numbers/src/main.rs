fn main() {
}

#[test]
fn test_1() {
    // Test case 1: Adding two single-digit numbers
    let l1 = generate_linked_list(&[2, 4, 3]);
    let l2 = generate_linked_list(&[5, 6, 4]);
    let expected = generate_linked_list(&[7, 0, 8]);
    assert_eq!(add_two_numbers(l1, l2), expected);
}

#[test]
fn test_2() {
    // Test case 1: Adding two single-digit numbers
    let l1 = generate_linked_list(&[9, 9, 9, 9, 9, 9, 9]);
    let l2 = generate_linked_list(&[9, 9, 9, 9]);
    let expected = generate_linked_list(&[8, 9, 9, 9, 0, 0, 0, 1]);
    assert_eq!(add_two_numbers(l1, l2), expected);
}

#[test]
fn test_3() {
    // Test case 1: Adding two single-digit numbers
    let l1 = generate_linked_list(&[9, 9, 9, 9]);
    let l2 = generate_linked_list(&[1]);
    let expected = generate_linked_list(&[0, 0, 0, 0, 1]);
    assert_eq!(add_two_numbers(l1, l2), expected);
}

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

fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // The head of the result list
    let mut result = ListNode::new(0);

    // The integer used to store the carried value (`mod(sum, 10)`)
    let mut carry = 0;

    // A pointer to the current node in list `result`
    let mut curr = &mut result;

    // While an element in either list (or the carryover value) exists:
    while l1.is_some() || l2.is_some() || carry > 0 {
        // Create a new `sum` integer
        let mut sum = carry;

        // If `l1` is valid, get the inner node
        if let Some(mut node) = l1 {
            // Then add it to the sum
            sum += node.val;
            // Assign the next node as the new `l1` value, deleting the old one via replacing it with `None`
            l1 = std::mem::replace(&mut node.next, None);
        }

        // Same as above for `l2`
        if let Some(mut node) = l2 {
            sum += node.val;
            l2 = std::mem::replace(&mut node.next, None);
        }

        // Get integer div of carry / 10 to return antimodulo (dunno if that's a real word but whatever)
        carry = sum / 10;

        // Create a new node in the list with the modulo of `sum` and `10`
        curr.next = Some(Box::new(ListNode::new(sum % 10)));

        // Increment to the next position in the result list.
        curr = curr.next.as_mut().unwrap();
    }

    // `result` was initialized to a ListNode of value `0` which was then appended, so return the list starting from the second element.
    result.next
}

fn generate_linked_list(nums: &[i32]) -> Option<Box<ListNode>> {
    // Create the head node
    let mut head = Some(Box::new(ListNode::new(nums[0])));
    let mut curr = &mut head;

    // Iterate over the rest of the array and add each element as a new node in the list
    for &num in &nums[1..] {
        let new_node = Some(Box::new(ListNode::new(num)));
        curr.as_mut().unwrap().next = new_node;
        curr = &mut curr.as_mut().unwrap().next;
    }

    head
}
