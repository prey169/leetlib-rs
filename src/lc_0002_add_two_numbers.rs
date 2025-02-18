use crate::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut curr = &mut head;
    let mut carry = 0;

    let mut l1 = l1;
    let mut l2 = l2;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let sum = l1.as_ref().map_or(0, |x| x.val) + l2.as_ref().map_or(0, |x| x.val) + carry;
        carry = sum / 10;

        let some_node = curr.insert(Box::new(ListNode::new(sum % 10)));

        curr = &mut some_node.next;

        l1 = l1.and_then(|n| n.next);
        l2 = l2.and_then(|n| n.next);
    }
    head
}

#[test]
fn ex1() {
    let mut l1: Option<Box<ListNode>> = None;
    let curr_l1 = &mut l1;
    let _ = curr_l1.insert(Box::new(ListNode::new(3)));
    let _ = curr_l1.insert(Box::new(ListNode::new(4)));
    let _ = curr_l1.insert(Box::new(ListNode::new(2)));

    let mut l2: Option<Box<ListNode>> = None;
    let curr_l2 = &mut l2;
    let _ = curr_l2.insert(Box::new(ListNode::new(4)));
    let _ = curr_l2.insert(Box::new(ListNode::new(6)));
    let _ = curr_l2.insert(Box::new(ListNode::new(5)));

    let mut l3: Option<Box<ListNode>> = None;
    let curr_l3 = &mut l3;
    let _ = curr_l3.insert(Box::new(ListNode::new(8)));
    let _ = curr_l3.insert(Box::new(ListNode::new(0)));
    let _ = curr_l3.insert(Box::new(ListNode::new(7)));

    assert_eq!(add_two_numbers(l1, l2), l3)
}
