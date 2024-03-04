use super::Node;
use std::fmt::Debug;

/// Describe the path traversal of a Node starting from the root node
///
/// The figure below shows `node_idx` in a depth first traversal.
///
/// ```text
///            .─.
///           ( 0 )
///            `-'
///           /   \
///          /     \
///         /       \
///        ▼         ▼
///       .─.         .─.
///      ( 1 )       ( 4 )
///       `-'         `-'
///      /  \          | \ '.
///     /    \         |  \  '.
///    ▼      ▼        |   \   '.
///  .─.      .─.      ▼    ▼     ▼
/// ( 2 )    ( 3 )    .─.   .─.   .─.
///  `─'      `─'    ( 5 ) ( 6 ) ( 7 )
///                   `─'   `─'   `─'
/// ```
///
/// The figure below shows the index of each child node relative to their parent node
///
/// ```text
///             .─.
///            ( 0 )
///             `-'
///            /   \
///           /     \
///          /       \
///         ▼         ▼
///        .─.         .─.
///       ( 0 )       ( 1 )
///        `-'         `-'
///       /  \          | \ '.
///      /    \         |  \  '.
///     ▼      ▼        |   \   '.
///   .─.      .─.      ▼    ▼     ▼
///  ( 0 )    ( 1 )    .─.   .─.   .─.
///   `─'      `─'    ( 0 ) ( 1 ) ( 2 )
///                    `─'   `─'   `─'
/// ```
/// The equivalent idx and path are as follows:
/// ```text
///    0 = []
///    1 = [0]
///    2 = [0,0]
///    3 = [0,1]
///    4 = [1]
///    5 = [1,0]
///    6 = [1,1]
///    7 = [1,2]
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TreePath {
    /// An array of child index at each level of the dom tree.
    /// The children of the nodes at each child index is traverse
    /// at each traversal the first element of path is removed until
    /// the path becomes empty.
    /// If the path has become empty the node is said to be found.
    ///
    /// Empty path means root node
    pub path: Vec<usize>,
}

impl TreePath {
    /// create a TreePath with node index `node_idx` and traversal path `path`
    pub fn new(path: impl IntoIterator<Item = usize>) -> Self {
        Self {
            path: path.into_iter().collect(),
        }
    }

    /// create a TreePath which starts at empty vec which is the root node of a DOM tree
    pub fn root() -> Self {
        Self { path: vec![] }
    }

    /// add a path node idx
    pub fn push(&mut self, node_idx: usize) {
        self.path.push(node_idx)
    }

    /// create a new TreePath with an added node_index
    /// This is used for traversing into child elements
    pub fn traverse(&self, node_idx: usize) -> Self {
        let mut new_path = self.clone();
        new_path.push(node_idx);
        new_path
    }

    /// backtrack to the parent node path
    pub fn backtrack(&self) -> Self {
        let mut new_path = self.clone();
        new_path.path.pop();
        new_path
    }

    /// remove first node index of this treepath
    /// Everytime a node is traversed, the first element should be removed
    /// until no more index is in this path
    pub fn remove_first(&mut self) -> usize {
        self.path.remove(0)
    }

    /// pluck the next in line node index in this treepath
    pub fn pluck(&mut self) -> usize {
        self.remove_first()
    }

    /// returns tree if the path is empty
    /// This is used for checking if the path has been traversed
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    /// find the node using the path of this tree path
    pub fn find_node_by_path<'a, MSG>(&self, node: &'a Node<MSG>) -> Option<&'a Node<MSG>> {
        let mut path = self.clone();
        traverse_node_by_path(node, &mut path)
    }
}

impl<const N: usize> From<[usize; N]> for TreePath {
    fn from(array: [usize; N]) -> Self {
        Self {
            path: array.to_vec(),
        }
    }
}

impl From<Vec<usize>> for TreePath {
    fn from(vec: Vec<usize>) -> Self {
        Self { path: vec }
    }
}

fn traverse_node_by_path<'a, MSG>(
    node: &'a Node<MSG>,
    path: &mut TreePath,
) -> Option<&'a Node<MSG>> {
    if path.path.is_empty() {
        Some(node)
    } else {
        let idx = path.path.remove(0);
        if let Some(child) = node.children().get(idx) {
            traverse_node_by_path(child, path)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use alloc::format;
    use alloc::string::String;
    use alloc::string::ToString;

    #[test]
    fn test_traverse() {
        let path = TreePath::from([0]);

        assert_eq!(path.traverse(1), TreePath::from([0, 1]));
    }

    fn sample_node() -> Node {
        let node: Node = element(
            "div",
            vec![attr("class", "[]"), attr("id", "0")],
            vec![
                element(
                    "div",
                    vec![attr("class", "[0]"), attr("id", "1")],
                    vec![
                        element(
                            "div",
                            vec![attr("class", "[0,0]"), attr("id", "2")],
                            vec![],
                        ),
                        element(
                            "div",
                            vec![attr("class", "[0,1]"), attr("id", "3")],
                            vec![],
                        ),
                    ],
                ),
                element(
                    "div",
                    vec![attr("class", "[1]"), attr("id", "4")],
                    vec![
                        element(
                            "div",
                            vec![attr("class", "[1,0]"), attr("id", "5")],
                            vec![],
                        ),
                        element(
                            "div",
                            vec![attr("class", "[1,1]"), attr("id", "6")],
                            vec![],
                        ),
                        element(
                            "div",
                            vec![attr("class", "[1,2]"), attr("id", "7")],
                            vec![],
                        ),
                    ],
                ),
            ],
        );
        node
    }

    // index is the index of this code with respect to it's sibling
    fn assert_traverse_match(
        node: &Node,
        node_idx: &mut usize,
        path: Vec<usize>,
    ) {
        let id = node.attribute_value(&"id").unwrap()[0];
        let class = node.attribute_value(&"class").unwrap()[0];
        assert_eq!(id.to_string(), node_idx.to_string());
        assert_eq!(class.to_string(), format_vec(&path));
        for (i, child) in node.children().iter().enumerate() {
            *node_idx += 1;
            let mut child_path = path.clone();
            child_path.push(i);
            assert_traverse_match(child, node_idx, child_path);
        }
    }

    fn traverse_tree_path(node: &Node, path: &TreePath, node_idx: &mut usize) {
        let id = node.attribute_value(&"id").unwrap()[0];
        let class = node.attribute_value(&"class").unwrap()[0];
        assert_eq!(id.to_string(), node_idx.to_string());
        assert_eq!(class.to_string(), format_vec(&path.path));
        for (i, child) in node.children().iter().enumerate() {
            *node_idx += 1;
            let mut child_path = path.clone();
            child_path.path.push(i);
            traverse_tree_path(child, &child_path, node_idx);
        }
    }

    fn format_vec(v: &[usize]) -> String {
        format!(
            "[{}]",
            v.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    #[test]
    fn should_match_paths() {
        let node = sample_node();
        assert_traverse_match(&node, &mut 0, vec![]);
        traverse_tree_path(&node, &TreePath::new(vec![]), &mut 0);
    }

    #[test]
    fn should_find_root_node() {
        let node = sample_node();
        let path = TreePath::new(vec![]);
        let root = path.find_node_by_path(&node);
        assert_eq!(Some(&node), root);
    }

    #[test]
    fn should_find_node1() {
        let node = sample_node();
        let path = TreePath::new(vec![0]);
        let found = path.find_node_by_path(&node);
        let expected = element(
            "div",
            vec![attr("class", "[0]"), attr("id", "1")],
            vec![
                element(
                    "div",
                    vec![attr("class", "[0,0]"), attr("id", "2")],
                    vec![],
                ),
                element(
                    "div",
                    vec![attr("class", "[0,1]"), attr("id", "3")],
                    vec![],
                ),
            ],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node2() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 0]);
        let found = path.find_node_by_path(&node);
        let expected = element(
            "div",
            vec![attr("class", "[0,0]"), attr("id", "2")],
            vec![],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node3() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 1]);
        let found = path.find_node_by_path(&node);
        let expected = element(
            "div",
            vec![attr("class", "[0,1]"), attr("id", "3")],
            vec![],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node4() {
        let node = sample_node();
        let path = TreePath::new(vec![1]);
        let found = path.find_node_by_path(&node);
        let expected = element(
            "div",
            vec![attr("class", "[1]"), attr("id", "4")],
            vec![
                element(
                    "div",
                    vec![attr("class", "[1,0]"), attr("id", "5")],
                    vec![],
                ),
                element(
                    "div",
                    vec![attr("class", "[1,1]"), attr("id", "6")],
                    vec![],
                ),
                element(
                    "div",
                    vec![attr("class", "[1,2]"), attr("id", "7")],
                    vec![],
                ),
            ],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node5() {
        let node = sample_node();
        let path = TreePath::new(vec![1, 0]);
        let found = path.find_node_by_path(&node);
        let expected = element(
            "div",
            vec![attr("class", "[1,0]"), attr("id", "5")],
            vec![],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node6() {
        let node = sample_node();
        let path = TreePath::new(vec![1, 1]);
        let found = path.find_node_by_path(&node);
        let expected = element(
            "div",
            vec![attr("class", "[1,1]"), attr("id", "6")],
            vec![],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_none_in_013() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 1, 3]);
        let found = path.find_node_by_path(&node);
        assert_eq!(None, found);
    }

    #[test]
    fn should_find_none_in_00000() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 0, 0, 0]);
        let found = path.find_node_by_path(&node);
        assert_eq!(None, found);
    }

    #[test]
    fn should_find_none_in_007() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 0, 7]);
        let bond = path.find_node_by_path(&node);
        assert_eq!(None, bond);
    }
}
