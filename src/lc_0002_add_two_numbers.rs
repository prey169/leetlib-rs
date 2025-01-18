#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
        
        l1 = l1.and_then(|n| n.next);
        l2 = l2.and_then(|n| n.next);


    }
    head
}

#[test]
fn ex1() {
    let mut l1 = Box::new(ListNode::new(2))
    l1.inse ListNode::new(4, ListNode::new(3))));
    let l2 = [5,6,4];
    assert_eq!(add_two_numbers(l1, l2), [7,0,8]) 
}
