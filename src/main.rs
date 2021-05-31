extern crate tree;

use tree::TreeNode;

fn main() {
    let node1 = TreeNode::<i32>::new(1);
    let node2 = TreeNode::new(2);
    let node3 = TreeNode::new(3);
    let node4 = TreeNode::new(4);
    let node5 = TreeNode::new(5);
    let node6 = TreeNode::new(6);
    let node7 = TreeNode::new(7);
    let node8 = TreeNode::new(8);
    let node9 = TreeNode::new(9);

    TreeNode::set_multiple_relationships(&node1, &vec![&node2, &node3]).unwrap();
    TreeNode::set_relationship(&node3, &node4).unwrap();
    TreeNode::set_multiple_relationships(&node4, &vec![&node5, &node6, &node7]).unwrap();
    TreeNode::set_multiple_relationships(&node2, &vec![&node8, &node9]).unwrap();

    let node7_value = node1.get_children()[1].get_children()[0].get_children()[2].get_printable_value();

    println!("node7 value scanned from the origin node: {}", node7_value);
}
