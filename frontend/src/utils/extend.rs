pub struct MTableView<A, B> {
    table_view: TableView<A, B>,
    // ... other fields ...
}

impl<A, B> MTableView<A, B> {
    pub fn table(&self) -> &TableView<A, B> {
        &self.table_view
    }

    pub fn table_mut(&mut self) -> &mut TableView<A, B> {
        &mut self.table_view
    }
}

// Imagine that this is an external crate.
mod external {
    pub struct TreeView;
    impl TreeView {
        pub fn base_method(&self) {
            println!("Base method called");
        }
    }
}

use external::TreeView;
use std::ops::Deref;

struct ExtendedTreeView {
    base: TreeView,
    extension: i32,
}

impl Deref for ExtendedTreeView {
    type Target = TreeView;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

fn test() {
    let view = ExtendedTreeView {
        base: TreeView,
        extension: 0,
    };
    view.base_method();
    println!("New field: {}", view.extension);
}
