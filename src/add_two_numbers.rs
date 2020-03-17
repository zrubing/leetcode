#[derive(Debug, PartialEq, Eq, Clone)]
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
    let mut dummy_head = ListNode::new(0);

    let mut last = &mut dummy_head;

    let mut carry = 0;
    let mut p = l1;
    let mut q = l2;
    while p.is_some() || q.is_some() || carry > 0 {
        p = Some(p.unwrap_or(Box::new(ListNode { val: 0, next: None })));
        q = Some(q.unwrap_or(Box::new(ListNode { val: 0, next: None })));

        let p_val = p.clone().unwrap().val;
        let q_val = q.clone().unwrap().val;

        let sum = p_val + q_val + carry;

        let val = sum % 10;
        carry = sum / 10;

        let new_node = Box::new(ListNode { val, next: None });
        last.next = Some(new_node);
        last = last.next.as_mut().unwrap();

        println!("val:{:?},carry:{:?}", val, carry);

        p = p.unwrap().next;
        q = q.unwrap().next;
    }

    return dummy_head.next;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let third = ListNode {
            val: 5,
            next: Option::None,
        };
        let second = ListNode {
            val: 6,
            next: Some(Box::new(third)),
        };
        let first = ListNode {
            val: 1,
            next: Some(Box::new(second)),
        };

        let clone = first.clone();

        let node = add_two_numbers(Some(Box::new(first)), Some(Box::new(clone)));

        println!("{:?}", node);
    }
}
