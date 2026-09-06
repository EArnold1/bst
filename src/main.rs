use std::println;

// All values in the left subtree of a node are less than the node's value,
// and all values in the right subtree are greater than the node's value.

#[derive(Clone, Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        // curr is a mutable ref to a Node slot
        let mut curr = &mut self.root;

        while let Some(node) = curr {
            if node.value > value {
                curr = &mut node.left;
            } else {
                curr = &mut node.right;
            }
        }
        *curr = Some(Box::new(Node::new(value)));
    }

    fn search(&self, value: i32) -> Option<&Node> {
        let mut curr = self.root.as_ref();

        while let Some(node) = curr {
            if node.value == value {
                return Some(node);
            }

            if node.value > value {
                // move left
                curr = node.left.as_ref();
            } else {
                // move right
                curr = node.right.as_ref();
            }
        }

        None
    }

    fn delete(&mut self, value: i32) {
        self.root = Self::delete_node(self.root.take(), value);
    }

    // recursive delete
    fn delete_node(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        let mut node = node?;

        if node.value > value {
            node.left = Self::delete_node(node.left.take(), value);
            Some(node)
        } else if node.value < value {
            node.right = Self::delete_node(node.right.take(), value);
            Some(node)
        } else {
            // found the node to delete
            match (node.left.take(), node.right.take()) {
                // leaf node
                (None, None) => None,

                // one child
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),

                // two children
                (Some(left), Some(right)) => {
                    // find the in-order successor
                    // walk down the right node's leftmost path
                    let mut right_node = right;
                    let min_val = Self::min_value(&right_node);

                    // delete successor
                    right_node = Self::delete_node(Some(right_node), min_val).unwrap();

                    Some(Box::new(Node {
                        value: min_val,
                        left: Some(left),
                        right: Some(right_node),
                    }))
                }
            }
        }
    }

    fn min_value(node: &Node) -> i32 {
        let mut curr = node;
        while let Some(ref next) = curr.left {
            curr = next;
        }
        curr.value
    }
}

// fn recursive_insert(root: &mut Option<Box<Node>>, value: i32) {
//     if root.is_none() {
//         let node = Node::new(value);
//         *root = Some(Box::new(node));
//     }

//     if let Some(node) = root {
//         if node.value > value {
//             recursive_insert(&mut node.left, value);
//         } else if node.value < value {
//             recursive_insert(&mut node.right, value);
//         }
//     }
// }

fn main() {
    let mut tree = Tree::new();

    // recursive_insert(&mut tree.root, 5);
    // recursive_insert(&mut tree.root, 2);
    // recursive_insert(&mut tree.root, 7);
    // recursive_insert(&mut tree.root, 10);
    // recursive_insert(&mut tree.root, 3);
    tree.insert(5);
    tree.insert(2);
    tree.insert(7);
    tree.insert(10);
    tree.insert(3);

    tree.delete(3);
    let search = tree.search(3);
    println!("{:?}", search);
    println!("{:?}", tree.root);
}
