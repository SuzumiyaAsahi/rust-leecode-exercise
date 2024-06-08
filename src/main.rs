use core::mem::drop;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
struct Node {
    pub value: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<Node>>>) {
        let mut que = VecDeque::new();
        if let Some(root_node) = root {
            que.push_back(root_node);
        }

        while !que.is_empty() {
            let len = que.len();
            let mut previous: Option<Rc<RefCell<Node>>> = None;

            for _ in 0..len {
                if let Some(current) = que.pop_front() {
                    if let Some(previous_node) = previous {
                        previous_node.borrow_mut().next = Some(current.clone());
                        drop(previous_node)
                    }
                    previous = Some(current.clone());

                    let (left, right) = {
                        let current_node = current.borrow();
                        (current_node.left.clone(), current_node.right.clone())
                    };

                    if let Some(left_node) = left {
                        que.push_back(left_node);
                    }
                    if let Some(right_node) = right {
                        que.push_back(right_node);
                    }
                }
            }

            if let Some(last_node) = previous {
                last_node.borrow_mut().next = None;
                drop(last_node)
            }
        }
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Node {
        value: 1,
        left: None,
        right: None,
        next: None,
    }));
    let b = Rc::new(RefCell::new(Node {
        value: 2,
        left: None,
        right: None,
        next: None,
    }));
    let c = Rc::new(RefCell::new(Node {
        value: 3,
        left: None,
        right: None,
        next: None,
    }));

    // a.borrow_mut().left = Some(b);
    // a.borrow_mut().right = Some(c);
    let mut a_borrow = a.borrow_mut();
    a_borrow.left = Some(b);
    a_borrow.right = Some(c);

    drop(a_borrow);

    Solution::right_side_view(Some(a.clone()));

    println!("{:#?}", a);
}
