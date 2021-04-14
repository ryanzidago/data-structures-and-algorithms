fn main() {
    println!("Hello, world!");
}

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub value: i32,
    pub left_child: Option<Rc<RefCell<TreeNode>>>,
    pub right_child: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(value: i32) -> TreeNode {
        TreeNode {
            value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn contains(&self, searched_value: i32) -> bool {
        match self.value.cmp(&searched_value) {
            Ordering::Equal => true,
            Ordering::Greater => match &self.left_child {
                Some(left_child) => left_child.borrow().contains(searched_value),
                None => false,
            },
            Ordering::Less => match &self.right_child {
                Some(right_child) => right_child.borrow().contains(searched_value),
                None => false,
            },
        }
    }

    pub fn insert(&mut self, value_to_be_inserted: i32) {
        match value_to_be_inserted.cmp(&self.value) {
            Ordering::Less => match &mut self.left_child {
                Some(left_child) => left_child.borrow_mut().insert(value_to_be_inserted),
                None => {
                    self.left_child =
                        Some(Rc::new(RefCell::new(TreeNode::new(value_to_be_inserted))))
                }
            },
            Ordering::Greater => match &mut self.right_child {
                Some(right_child) => right_child.borrow_mut().insert(value_to_be_inserted),
                None => {
                    self.right_child =
                        Some(Rc::new(RefCell::new(TreeNode::new(value_to_be_inserted))))
                }
            },
            Ordering::Equal => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_returns_a_tree_node_with_the_created_value_wrapped_in_the_option_enum() {
        let bst = TreeNode::new(100);
        let expected = TreeNode {
            value: 100,
            left_child: None,
            right_child: None,
        };

        assert_eq!(bst, expected);
    }

    #[test]
    fn contains_return_true_if_binary_search_tree_contains_the_searched_value() {
        let bst = TreeNode::new(100);
        assert!(bst.contains(100));
    }

    #[test]
    fn insert_inserts_a_value_into_a_binary_search_tree() {
        let mut bst = TreeNode::new(3);
        bst.insert(2);

        let expected = TreeNode {
            value: 3,
            left_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 2,
                left_child: None,
                right_child: None,
            }))),
            right_child: None,
        };

        assert_eq!(bst, expected);

        let mut bst = TreeNode::new(50);
        bst.insert(25);
        bst.insert(75);
        bst.insert(10);
        bst.insert(56);
        bst.insert(33);
        bst.insert(89);

        let expected = TreeNode {
            value: 50,
            left_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 25,
                left_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 10,
                    left_child: None,
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 33,
                    left_child: None,
                    right_child: None,
                }))),
            }))),
            right_child: Some(Rc::new(RefCell::new(TreeNode {
                value: 75,
                left_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 56,
                    left_child: None,
                    right_child: None,
                }))),
                right_child: Some(Rc::new(RefCell::new(TreeNode {
                    value: 89,
                    left_child: None,
                    right_child: None,
                }))),
            }))),
        };

        assert_eq!(bst, expected);
    }
}
