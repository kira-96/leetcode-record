// 通过
// 思路：递归方式累加
// 直到所有节点的next都为None为止
// 注意溢出问题，每个节点的值都不能大于 9
// 所以我在后面加了一个是否溢出的参数

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
}

fn main() {
    println!("{:?}", add_two_numbers(
        Some(Box::new(ListNode { 
            val: 2, 
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None
                }))
            }))
        })), 
        Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None
                }))
            }))
        }))));
    
        println!("{:?}", add_two_numbers(
            Some(Box::new(ListNode { 
                val: 9,
                next: None
            })), 
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode {
                                                val: 9,
                                                next: Some(Box::new(ListNode {
                                                    val: 9,
                                                    next: None
                                                }))
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))));
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two_numbers_internal(l1, l2, false)
}

pub fn add_two_numbers_internal(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, overflow: bool) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(list1), Some(list2)) => {
            let val = list1.val + list2.val + if overflow { 1 } else { 0 };
            let inter_overflow = val > 9;
            Some(Box::new(ListNode { 
                val: if inter_overflow { val - 10 } else { val },
                next: add_two_numbers_internal(list1.next, list2.next, inter_overflow)
            }))
        },
        (Some(list1), None) => {
            let val = list1.val + if overflow { 1 } else { 0 };
            let inter_overflow = val > 9;
            Some(Box::new(ListNode { 
                val: if inter_overflow { val - 10 } else { val }, 
                next: add_two_numbers_internal(list1.next, None, inter_overflow)
            }))
        },
        (None, Some(list2)) => {
            let val = list2.val + if overflow { 1 } else { 0 };
            let inter_overflow = val > 9;
            Some(Box::new(ListNode {
                val: if inter_overflow { val - 10 } else { val },
                next: add_two_numbers_internal(None, list2.next, inter_overflow) 
            }))
        },
        (None, None) => {
            if overflow {
                Some(Box::new(ListNode::new(1)))
            } else {
                None
            }
        }
    }
}