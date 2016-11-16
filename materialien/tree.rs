/// Represents a tree.
///
/// Each internal node can have any number of children. Each node (internal
/// and leaf) carries an `i32`.
#[derive(Debug, Clone)]
enum Tree {
    Leaf(i32),
    Internal {
        children: Vec<Tree>,
        data: i32,
    },
}

impl Tree {
    /// Determines if this tree is just a leaf node.
    fn is_leaf(&self) -> bool {
        match *self {
            Tree::Leaf(_) => true,
            _ => false,
        }
    }

    /// Returns the value attached to this node.
    fn data(&self) -> i32 {
        match *self {
            Tree::Leaf(data) => data,
            Tree::Internal { data, .. } => data,
        }
    }


    fn data_mut(&mut self) -> &mut i32 {
        match *self {
            Tree::Leaf(ref mut data) => data,
            Tree::Internal { ref mut data, .. } => data,
        }
    }

    /// Adds another tree as a child of this node.
    fn add_child(&mut self, child: Tree) {
        if let Tree::Leaf(data) = *self {
            *self = Tree::Internal {
                children: vec![],
                data: data,
            };
        }

        match *self {
            Tree::Internal { ref mut children, .. } => {
                children.push(child);
            }
            _ => unreachable!(),
        }
    }

    /// Adds another leaf node with the given value as child of this node.
    fn add_leaf_child(&mut self, child_data: i32) {
        self.add_child(Tree::Leaf(child_data));
    }

    /// If this node is internal, this function returns a slice of all
    /// children.
    fn children(&self) -> Option<&[Tree]> {
        match *self {
            Tree::Internal { ref children, .. } => Some(children),
            _ => None,
        }
    }

    /// If this node is internal, this function returns a mutable slice of all
    /// children.
    fn children_mut(&mut self) -> Option<&mut [Tree]> {
        match *self {
            Tree::Internal { ref mut children, .. } => Some(children),
            _ => None,
        }
    }

    /// Returns a string that contains the values of all nodes in preorder. The
    /// values are separated by `separator`.
    fn format_preorder(&self, separator: char) -> String {
        match *self {
            Tree::Leaf(data) => data.to_string(),
            Tree::Internal { data, ref children } => {
                let mut out = data.to_string();

                for child in children {
                    out.push(separator);
                    out += &child.format_preorder(separator);
                }

                out
            }
        }
    }
}



fn main() {
    let a = Tree::Leaf(3);
    let b = Tree::Leaf(7);
    let mut root = Tree::Internal {
        children: vec![a, b],
        data: 99,
    };

    for child in root.children_mut().unwrap() {
        child.add_leaf_child(123);
        child.add_leaf_child(234);
    }

    println!("{:#?}", root);

    println!("{}", root.format_preorder(','));
}
