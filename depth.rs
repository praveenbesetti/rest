// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Constructor to create a new TreeNode
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        },
        None => 0,
    }
}

fn main() {
    // Example binary tree
    let mut root = TreeNode::new(3);
    root.left = Some(Box::new(TreeNode::new(9)));
    root.right = Some(Box::new(TreeNode::new(20)));
    root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
    root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));

    let depth = max_depth(Some(Box::new(root)));
    println!("Maximum depth of the tree: {}", depth);
}
