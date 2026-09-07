// balanced_factor is between -1 and 1
// an AVL Tree is a balanced binary search tree
// after an insert, a rebalancing check is performed to maintain the AVL property

#[derive(Debug, Clone, PartialEq)]
struct Node {
    pub key: i32,
    height: i8, // 0-based, meaning leaf nodes have height 0
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key,
            height: 0,
            left: None,
            right: None,
        }
    }
}

struct AvlTree {
    pub root: Option<Box<Node>>,
    stack: Vec<Node>,
}

impl Iterator for AvlTree {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        // Push all the way down the left side
        // the stack will contain something like:
        // [parent, parent_left_child], left_child is popped, root becomes None
        // the parent is popped([]) and the root becomes the parent right child
        // stack gets updated [parent_right_child] and the stack is popped again leaving the stack empty
        while let Some(mut node) = self.root.take() {
            self.root = node.left.take();
            self.stack.push(*node);
        }

        let mut node = self.stack.pop()?; // This is the point that returns None if no item is found in the stack

        // move to right
        self.root = node.right.take();

        Some(node)
    }
}

// TODOS:
// 1. Search
// 2. Delete

impl AvlTree {
    pub fn new() -> Self {
        Self {
            root: None,
            stack: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: i32) {
        Self::insert_node(&mut self.root, key);
    }

    fn ll_rotation(node: &mut Option<Box<Node>>) {
        let mut old_root = node.take().unwrap();
        let mut new_root = old_root.left.take().unwrap();

        // Preserve the middle subtree
        old_root.left = new_root.right.take();

        // Put old root under new root
        new_root.right = Some(old_root);

        Self::update_height(new_root.right.as_mut().unwrap());
        Self::update_height(&mut new_root);

        *node = Some(new_root);
    }

    fn lr_rotation(node: &mut Option<Box<Node>>) {
        let mut old_root = node.take().unwrap();
        let mut left = old_root.left.take().unwrap();
        let mut new_root = left.right.take().unwrap();

        // preserve the subtrees of the new root
        left.right = new_root.left.take();
        old_root.left = new_root.right.take();

        new_root.left = Some(left);
        new_root.right = Some(old_root);

        Self::update_height(new_root.left.as_mut().unwrap());
        Self::update_height(new_root.right.as_mut().unwrap());
        Self::update_height(&mut new_root);

        *node = Some(new_root);
    }

    fn rr_rotation(node: &mut Option<Box<Node>>) {
        let mut old_root = node.take().unwrap();
        let mut new_root = old_root.right.take().unwrap();

        // Preserve the middle subtree
        old_root.right = new_root.left.take();

        // Put old root under new root
        new_root.left = Some(old_root);

        Self::update_height(new_root.left.as_mut().unwrap());
        Self::update_height(&mut new_root);

        *node = Some(new_root);
    }

    fn rl_rotation(node: &mut Option<Box<Node>>) {
        let mut old_root = node.take().unwrap();
        let mut right = old_root.right.take().unwrap();
        let mut new_root = right.left.take().unwrap();

        // Preserve both middle subtrees
        old_root.right = new_root.left.take();
        right.left = new_root.right.take();

        new_root.left = Some(old_root);
        new_root.right = Some(right);

        Self::update_height(new_root.left.as_mut().unwrap());
        Self::update_height(new_root.right.as_mut().unwrap());
        Self::update_height(&mut new_root);

        *node = Some(new_root);
    }

    fn balance_node(node: &mut Option<Box<Node>>) {
        if let Some(n) = node {
            // left-heavy
            if Self::bf(n) > 1 {
                if Self::bf(n.left.as_ref().unwrap()) >= 0 {
                    Self::ll_rotation(node);
                } else {
                    Self::lr_rotation(node);
                }
            } else if Self::bf(n) < -1 {
                // right-heavy
                if Self::bf(n.right.as_ref().unwrap()) <= 0 {
                    Self::rr_rotation(node);
                } else {
                    Self::rl_rotation(node);
                }
            }
        }
    }

    fn insert_node(node: &mut Option<Box<Node>>, key: i32) {
        match node {
            None => {
                *node = Some(Box::new(Node::new(key)));
            }

            Some(n) => {
                if n.key == key {
                    return;
                }

                if n.key > key {
                    Self::insert_node(&mut n.left, key);
                } else {
                    Self::insert_node(&mut n.right, key);
                }

                Self::update_height(n);

                Self::balance_node(node);
            }
        }
    }

    pub fn search(&self, key: i32) -> Option<&Node> {
        let mut curr = self.root.as_ref();

        while let Some(node) = curr {
            if node.key == key {
                return Some(node);
            }

            if node.key > key {
                // move left
                curr = node.left.as_ref();
            } else {
                // move right
                curr = node.right.as_ref();
            }
        }

        None
    }

    /// Calculates balanced factor of a node
    /// a balanced factor is between -1 and 1
    ///
    /// bf = height(left subtree) - height(right subtree)
    fn bf(node: &Node) -> i8 {
        Self::height(node.left.as_deref()) - Self::height(node.right.as_deref())
    }

    /// The height is 0-based,
    /// so a leaf node's height is 0
    fn height(node: Option<&Node>) -> i8 {
        // height of a None node is -1
        node.as_ref().map_or(-1, |node| node.height)
    }

    fn update_height(node: &mut Node) {
        // h = max(left, right) + 1
        node.height = std::cmp::max(
            Self::height(node.left.as_deref()),
            Self::height(node.right.as_deref()),
        ) + 1;
    }

    pub fn in_order_traversal(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::traversal_in_order(&self.root, &mut result);
        result
    }

    pub fn root_node(&self) -> Option<&Node> {
        self.root.as_deref()
    }

    /// Performs an in-order traversal of the subtree rooted at the given node,
    /// appending the keys to the provided result vector.
    ///
    /// `LEFT -> NODE(ROOT) -> RIGHT`
    fn traversal_in_order(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            Self::traversal_in_order(&n.left, result);
            result.push(n.key);
            Self::traversal_in_order(&n.right, result);
        }
    }

    pub fn delete(&mut self, value: i32) {
        Self::delete_node(&mut self.root, value);
    }

    // recursive delete
    fn delete_node(node: &mut Option<Box<Node>>, value: i32) {
        let Some(n) = node else {
            return;
        };

        if n.key > value {
            Self::delete_node(&mut n.left, value);
        } else if n.key < value {
            Self::delete_node(&mut n.right, value);
        } else {
            // found the node to delete
            match (n.left.take(), n.right.take()) {
                // leaf node
                (None, None) => {
                    *node = None;
                    return;
                }

                // one child
                (Some(left), None) => {
                    *node = Some(left);
                    return;
                }
                (None, Some(right)) => {
                    *node = Some(right);
                    return;
                }

                // two children
                (Some(left), Some(right)) => {
                    // find the in-order successor: right subtree's leftmost node
                    let min_val = Self::min_value(&right);

                    n.key = min_val;
                    n.left = Some(left);
                    n.right = Some(right);

                    // delete the successor from the right subtree
                    Self::delete_node(&mut n.right, min_val);
                }
            }
        }

        Self::update_height(n);
        Self::balance_node(node);
    }

    /// Finds the leftmost leaf node in the given subtree and returns its key.
    fn min_value(node: &Node) -> i32 {
        let mut curr = node;
        while let Some(ref next) = curr.left {
            curr = next;
        }
        curr.key
    }
}

pub fn run_avl() {
    let mut tree = AvlTree::new();

    for i in [45, 40, 30, 41, 35, 46, 60, 50, 20, 42, 70] {
        tree.insert(i);
    }

    let search = tree.search(35);
    println!("search result for 35: {:?}", search);

    tree.delete(35);

    let search = tree.search(35);
    println!("search result for 35 after delete: {:?}", search);

    let in_order = tree.in_order_traversal();

    println!("in-order traversal: {:?}", in_order);
    println!("{:#?}", tree.root_node());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bst_ordering() {
        let mut tree = AvlTree::new();

        tree.insert(20);
        tree.insert(30);
        tree.insert(10);

        let mut root_node = Node::new(20);
        root_node.height = 1;
        root_node.left = Some(Box::new(Node::new(10)));
        root_node.right = Some(Box::new(Node::new(30)));

        assert_eq!(tree.root_node(), Some(&root_node));
    }

    // balanced factor check
    // This test ensures that the balanced factor (BF) of every node in the AVL tree is within the allowed range (-1, 0, 1).
    #[test]
    fn test_absolute_balanced_factor() {
        let mut tree = AvlTree::new();

        tree.insert(20);
        tree.insert(30);
        tree.insert(10);

        assert!(check_bf(tree.root.as_deref()));
    }

    #[test]
    fn test_bst_nodes_balanced_factor() {
        let mut tree = AvlTree::new();

        tree.insert(20);
        tree.insert(30);
        tree.insert(10);

        let root_node = tree.root_node().unwrap();
        let left_child = root_node.left.as_deref().unwrap();
        let right_child = root_node.right.as_deref().unwrap();

        assert_eq!(AvlTree::bf(root_node), 0);
        assert_eq!(AvlTree::bf(left_child), 0);
        assert_eq!(AvlTree::bf(right_child), 0);
    }

    // height check
    #[test]
    fn test_root_height_after_insertions() {
        let mut tree = AvlTree::new();

        tree.insert(20);
        tree.insert(40);
        tree.insert(15);
        tree.insert(25);
        tree.insert(45);

        assert_eq!(tree.root_node().unwrap().height, 2);
    }

    // in-order traversal check
    #[test]
    fn returns_in_order_traversal() {
        let mut tree = AvlTree::new();

        tree.insert(20);
        tree.insert(30);
        tree.insert(10);

        assert_eq!(tree.in_order_traversal(), vec![10, 20, 30]);
    }

    // duplicate insertion check
    #[test]
    fn ignore_duplicate_insertion() {
        let mut tree = AvlTree::new();

        tree.insert(20);
        tree.insert(20); // duplicate insertion

        assert_eq!(tree.in_order_traversal(), vec![20]);
    }

    // rotation check
    #[test]
    fn test_rotation_balance_after_insert() {
        let mut tree = AvlTree::new();

        tree.insert(10);
        tree.insert(20);
        tree.insert(30); // should trigger a rotation

        assert_eq!(tree.in_order_traversal(), vec![10, 20, 30]);
        assert_eq!(tree.root_node().unwrap().key, 20);
    }

    #[test]
    fn search_returns_matching_node() {
        let mut tree = AvlTree::new();

        tree.insert(10);
        tree.insert(20);
        tree.insert(30);
        tree.insert(65);
        tree.insert(15);

        let expected = Node::new(15);

        assert_eq!(tree.search(15), Some(&expected));
    }

    #[test]
    fn search_returns_none_when_key_does_not_exist() {
        let mut tree = AvlTree::new();

        tree.insert(10);
        tree.insert(20);
        tree.insert(30);
        tree.insert(65);
        tree.insert(15);

        assert_eq!(tree.search(99), None);
    }

    #[test]
    fn delete_rebalances() {
        let mut tree = AvlTree::new();
        for k in [10, 20, 30, 40, 50, 25] {
            tree.insert(k);
        }
        tree.delete(10);
        assert!(check_bf(tree.root.as_deref())); // reuse the helper
        assert_eq!(tree.in_order_traversal(), vec![20, 25, 30, 40, 50]);
    }

    fn check_bf(node: Option<&Node>) -> bool {
        if let Some(n) = node {
            let bf = AvlTree::bf(n);
            if bf.abs() > 1 {
                return false;
            }
            return check_bf(n.left.as_deref()) && check_bf(n.right.as_deref());
        }
        true
    }
}
