// balanced_factor is between -1 and 1

#[derive(Debug, Clone)]
struct Node {
    key: i32,
    height: i8, // 0-based
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
}

impl AvlTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: i32) {
        Self::insert_node(&mut self.root, key);
    }

    fn ll_rotation(node: &mut Option<Box<Node>>) {
        let left = node.as_mut().unwrap().left.take();
        let mut new_root = left.unwrap();
        new_root.right = node.take();

        Self::update_height(new_root.right.as_mut().unwrap());
        Self::update_height(&mut new_root);
        *node = Some(new_root);
    }

    fn lr_rotation(node: &mut Option<Box<Node>>) {
        let right_of_left = node.as_mut().unwrap().left.as_mut().unwrap().right.take();
        let mut new_root = right_of_left.unwrap();

        let left = node.as_mut().unwrap().left.take();
        new_root.left = left;
        Self::update_height(new_root.left.as_mut().unwrap());

        new_root.right = node.take();
        Self::update_height(new_root.right.as_mut().unwrap());
        Self::update_height(&mut new_root);
        *node = Some(new_root);
    }

    fn rr_rotation(node: &mut Option<Box<Node>>) {
        let right = node.as_mut().unwrap().right.take();
        let mut new_root = right.unwrap();
        new_root.left = node.take();

        Self::update_height(new_root.left.as_mut().unwrap());
        Self::update_height(&mut new_root);
        *node = Some(new_root);
    }

    fn rl_rotation(node: &mut Option<Box<Node>>) {
        let left_of_right = node.as_mut().unwrap().right.as_mut().unwrap().left.take();
        let mut new_root = left_of_right.unwrap();

        let right = node.as_mut().unwrap().right.take();
        new_root.right = right;
        Self::update_height(new_root.right.as_mut().unwrap());

        new_root.left = node.take();
        Self::update_height(new_root.left.as_mut().unwrap());
        Self::update_height(&mut new_root);
        *node = Some(new_root);
    }

    fn insert_node(node: &mut Option<Box<Node>>, key: i32) {
        match node {
            None => {
                *node = Some(Box::new(Node::new(key)));
            }

            Some(n) => {
                // skip seen value
                if n.key > key {
                    Self::insert_node(&mut n.left, key);
                } else {
                    Self::insert_node(&mut n.right, key);
                }

                Self::update_height(n);

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
    }

    /// Calculates balanced factor of a node
    ///
    /// bf = height(left subtree) - height(right subtree)
    fn bf(node: &Node) -> i8 {
        Self::height(node.left.as_deref()) - Self::height(node.right.as_deref())
    }

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

    pub fn traversal(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::in_order_traversal(&self.root, &mut result);
        result
    }

    fn in_order_traversal(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
        // LEFT -> NODE(ROOT) -> RIGHT
        if let Some(n) = node {
            Self::in_order_traversal(&n.left, result);
            result.push(n.key);
            Self::in_order_traversal(&n.right, result);
        }
    }
}

// TESTS to assert
// 1. BST ordering is valid
// 2. Every node has |BF| <= 1 (absolute BF is <= 1)
// 3. Stored/computed heights are correct
// 4. In-order traversal is sorted

pub fn run_avl() {
    let mut tree = AvlTree::new();
    // tree.insert(30);
    // tree.insert(20);
    // tree.insert(10);
    // tree.insert(5);

    // tree.insert(10);
    // tree.insert(30);
    // tree.insert(20);
    // tree.insert(5);
    // tree.insert(40);
    // tree.insert(30);
    // tree.insert(50);
    // tree.insert(20);
    // tree.insert(35);
    // tree.insert(45);
    // tree.insert(60);
    // tree.insert(70);
    // tree.insert(41);
    // tree.insert(42);
    // tree.insert(46);

    for i in [45, 40, 30, 41, 35, 46, 60, 50, 20, 42, 70] {
        tree.insert(i);
    }

    let in_order = tree.traversal();
    println!("In-order traversal: {:?}", in_order);
    // [20, 30, 35, 40, 41, 42, 45, 46, 50, 60, 70]
    // [20, 30, 35, 40, 45, 46, 50, 60, 70]
    // let mut root = Node::new(10);
    // root.right = Some(Box::new(Node::new(20)));

    // root.bf = AvlTree::bf(&root);
    // println!("{:#?}", root);

    // println!("{:#?}", tree.root);
}
