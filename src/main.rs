use std::collections::HashMap;

fn main() {
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

pub fn tow_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, value) in nums.iter().enumerate() {
        let inner_vec = nums[index + 1..].iter();

        for (inner_index, inner_value) in inner_vec.enumerate() {
            if value + inner_value == target {
                println!("{},{}", value, inner_value);

                return vec![index as i32, (index + 1 + inner_index) as i32];
            }
        }
    }
    return vec![];
}

pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        hash.insert(value, index);
    }

    println!("{:?}", hash);

    for (index, value) in nums.iter().enumerate() {
        let need_value_index = hash.get(&(target - value));

        println!("{:?}", need_value_index);

        if need_value_index.is_some() {
            if *need_value_index.unwrap() == index {
                continue;
            }
            return vec![index as i32, *need_value_index.unwrap() as i32];
        }
    }

    return vec![];
}

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
    let mut plus_one: bool = false;
    let mut node: Option<Box<ListNode>> = None;

    if l1.is_none() && l2.is_none() {
        return None;
    } else if l1.is_some() && l2.is_none() {
        return l1;
    } else if l1.is_none() && l2.is_some() {
        return l2;
    }

    let mut l1_next = l1;
    let mut l2_next = l2;

    let mut first_new_node: Option<Box<ListNode>> = None;
    let mut new_node: Option<Box<ListNode>> = None;

    let mut new_val = String::from("");
    loop {
        // println!("l1_node {:?},l2_node {:?}", l1_next, l2_next);
        if l1_next.is_none() && l2_next.is_none() && plus_one {
            new_val.push_str("1");
        }

        if l1_next.is_none() && l2_next.is_none() {
            break;
        }

        let l1_val = l1_next
            .clone()
            .unwrap_or(Box::new(ListNode { val: 0, next: None }))
            .val;
        let l2_val = l2_next
            .clone()
            .unwrap_or(Box::new(ListNode { val: 0, next: None }))
            .val;

        let mut sum = l1_val + l2_val;

        if plus_one {
            sum = sum + 1;
        }

        if sum >= 10 {
            plus_one = true;

            //获取个位
            sum = sum % 10;
        } else {
            plus_one = false;
        }
        new_val.push_str(&sum.to_string());

        println!("l1_val {:?},l2_val {:?}", l1_val, l2_val);

        l1_next = (l1_next.unwrap_or(Box::new(ListNode { val: 0, next: None }))).next;
        l2_next = (l2_next.unwrap_or(Box::new(ListNode { val: 0, next: None }))).next;
    }

    println!("{:?}", new_val);

    for i in new_val.chars().rev() {
        let val = i.to_digit(10).unwrap() as i32;

        new_node = Some(Box::new(ListNode {
            val,
            next: new_node,
        }));
    }

    println!("{:?}", new_node);

    return new_node;
}
