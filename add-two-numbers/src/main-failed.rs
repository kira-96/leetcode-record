// 提交失败
// 思路：将链表转成整数相加，再转成链表
// 失败原因：遇到超级大的数后，整数溢出

use std::convert::{ From, Into };

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn insert(&mut self, val: i32) {
        let mut node = & mut self.next;

        loop {
            if let Some(subnode) = node {
                node = &mut subnode.next;
            } else {
                *node = Some(Box::new(ListNode {
                    val,
                    next: None
                }));
                break;
            }
        }
    }
}

impl From<i32> for ListNode {
    fn from(iter: i32) -> Self {
        if iter == 0 {
            return Self::new(0);
        }

        let mut node = ListNode::new(iter % 10);
        let mut val = iter / 10;

        while val > 0 {
            node.insert(val % 10);
            val = val / 10;
        }

        node
    }
}

impl Into<i32> for Box<ListNode> {
    fn into(self) -> i32 {
        let mut flag = 10;
        let mut integer = self.val;
        let mut node = &self.next;

        loop {
            if let Some(val) = node {
                integer = integer + flag * val.val;
                node = &val.next;
                flag *= 10;
            } else {
                break;
            }
        }

        integer
    }
}

// fn main() {
//     println!("{:?}", add_two_numbers(Some(Box::new(ListNode::from(9))), Some(Box::new(ListNode::from(99999991)))));
// }

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(list1), Some(list2)) => {
            let num1: i32 = list1.into();
            let num2: i32 = list2.into();
            Some(Box::new(ListNode::from(num1 + num2)))
        },
        _ => None
    }
}
