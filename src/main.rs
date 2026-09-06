use crate::{avl::run_avl, bst::run_bst};

mod avl;
mod bst;

fn main() {
    {
        println!("Running BST...");
        run_bst();
    }
    {
        println!("Running AVL...");
        run_avl();
    }
}
