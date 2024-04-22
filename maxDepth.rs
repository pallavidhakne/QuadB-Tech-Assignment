use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left_depth = Solution::max_depth(node.borrow().left.clone());
                let right_depth = Solution::max_depth(node.borrow().right.clone());
                1 + left_depth.max(right_depth)
            },
            None => 0,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

struct Solution;

fn main() {
   

    let root = TreeNode::new(3);
    let left = TreeNode::new(9);
    let right = TreeNode::new(20);
    let right_left = TreeNode::new(15);
    let right_right = TreeNode::new(7);

    root.borrow_mut().left = Some(left);
   
    root.borrow_mut().right = Some(right.clone());  
    right.borrow_mut().left = Some(right_left);
    right.borrow_mut().right = Some(right_right);

    let depth = Solution::max_depth(Some(root));
    println!("The maximum depth of the tree is {}", depth);
}
