mod printer;

use std::{cell::{Ref, RefCell}, fmt::Display, rc::{Rc, Weak}};

pub struct TreeNode<T> where T: Display{
    value: T,
    parent: RefCell<Weak<TreeNode<T>>>,
    children: RefCell<Vec<Rc<TreeNode<T>>>>,
    depth_level: RefCell<usize>,
}

impl<T> TreeNode<T> where T: Display{
    /// Constructor, this struct doesn't make sense to be used without a counted reference
    pub fn new(value: T) -> Rc<TreeNode<T>> {
        return Rc::new(TreeNode {
            value,
            parent: RefCell::new(Weak::new()), 
            children: RefCell::new(vec![]),
            depth_level: RefCell::new(0),
        });
    }
    
    pub fn get_printable_value(&self) -> String {
        return format!("{}", self.value);
    }

    /// Borrows non mutable reference to this node children
    pub fn get_children(&self) -> Ref<Vec<Rc<TreeNode<T>>>> {
        return self.children.borrow();
    }
    
    /// Gets Option to this node parent reference
    pub fn get_parent(&self) -> Option<Rc<TreeNode<T>>> {
        return self.parent.borrow().upgrade();
    }
    
    /// Sets parent/child references on the correspondant nodes\n
    /// The references used are cloned incrementing the reference counters on the nodes
    pub fn set_relationship(intended_parent: &Rc<TreeNode<T>>, intended_child: &Rc<TreeNode<T>>) -> Result<String, String>{
        if let Some(_) = intended_child.get_parent() {
            return Err(String::from("This node is already the child of another"));
        }
        intended_parent.add_child(intended_child);
        intended_child.set_parent(intended_parent);
        return Ok(String::from("No problemo"));
    }
    
    /// Sets multiple parent/children references for the same parent
    /// Maintains vector ownership over nodes doing a reference clone incrementing the reference counters
    pub fn set_multiple_relationships(intended_parent: &Rc<TreeNode<T>>, intended_children: &[&Rc<TreeNode<T>>]) -> Result<String, String> {
        let result_messages:Vec<String> =  intended_children.iter()
                                    .enumerate()
                                    .filter_map(|(i, node)| {
                                        if let Some(_) = node.get_parent() {
                                            return Some(format!("This node, index {}, is already the child of another", i));
                                        } else {
                                            return None;
                                        }
                                    })
                                    .collect();

        if result_messages.len() != 0 {
            return Err(result_messages.join(", "));
        }

        intended_children.iter().for_each(|node| { node.set_parent(&intended_parent); });
        intended_parent.add_children(intended_children);
        intended_parent.update_children_depth();
        return Ok(String::from("No problemo"));
    }

    /// Deletes the relationships that tie one node to a tree.
    /// If a reference is mantained to this node it will not hold more than the value
    pub fn remove_node(node: Rc<TreeNode<T>>) {
        if let Some(parent) = node.get_parent() {
            let node_children = node.children.take().to_vec();
            node_children.iter().for_each(|child| { child.set_parent(&parent); });
            parent.remove_child(&node);

            parent.transfer_children(node_children);
            parent.update_children_depth();

            *node.depth_level.borrow_mut() = 0;
            node.parent.take(); // We "take" the references to other nodes, this way the counters (Rc) are decremented
        } else {
            // The node doesn't have parent, so this node' children are left orfans
            // If no other variables hold a counted reference to them they will be disposed
            // Pretty sad shit
            node.get_children().iter().for_each(|child| { child.parent.take(); });
            node.children.take();
        }
    }

    /// Removes only the connection with the parent node, this way this subtree can still be used
    pub fn remove_subtree(origin_node: &Rc<TreeNode<T>>) {
        if let Some(parent) = origin_node.get_parent() {
            origin_node.parent.take();
            parent.remove_child(&origin_node);
            *origin_node.depth_level.borrow_mut() = 0;
            origin_node.update_children_depth();
        }
    }

    /// Sets new child reference on the current node 
    /// Privated to prevent forgotten parent references
    fn add_child(&self, node: &Rc<TreeNode<T>>) -> &TreeNode<T> {
        self.children.borrow_mut().push(Rc::clone(node));
        self.update_children_depth();
        return self;
    }

    /// Uses node references to set multiple children to the current node.
    /// Privated to prevent forgotten parent references.
    fn add_children(&self, nodes: &[&Rc<TreeNode<T>>]) -> &TreeNode<T> {
        let cloned_references:Vec<Rc<TreeNode<T>>> = nodes.iter().map(|node| Rc::clone(node)).collect();
        self.children.borrow_mut().extend(cloned_references);
        return self;
    }

    /// Takes ownership of a vector of reference counters to nodes to extend the current node children.
    /// This ownership transfer means that no reference counters are increased.
    /// Privated to prevent forgotten parent references.
    fn transfer_children(&self, nodes: Vec<Rc<TreeNode<T>>>) -> &TreeNode<T> {
        self.children.borrow_mut().extend(nodes);
        return self;
    }

    /// Compares the pointer used as counted reference to check node equality and delete the node that matches
    fn remove_child(&self, node: &Rc<TreeNode<T>>) {
        self.children.borrow_mut().retain(|child| !Rc::ptr_eq(node, child));
    }

    /// Sets parent reference on the current node. 
    /// Privated to prevent forgotten child references.
    fn set_parent(&self, node: &Rc<TreeNode<T>>) -> &TreeNode<T> {
        *self.parent.borrow_mut() = Rc::downgrade(node);
        return self;
    }

    /// Changes the values of all the children depth level until the last layer is reached
    /// To be used only when new nodes and sub trees are inserted 
    fn update_children_depth(&self) {
        for child in self.get_children().iter() {
            *child.depth_level.borrow_mut() = *self.depth_level.borrow() + 1;
            if child.get_children().len() != 0 {
                child.update_children_depth();
            }
        }
    }
}