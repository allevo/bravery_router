use regex::Regex;

pub const MAX_NEASTING_LEVEL_COUNT: usize = 16;

/// Enum for distinguishing the kind of the node type
///
/// Three kind of type are currently implemented:
/// - Static: represent a node that is identified by a chars sequence
/// - Regex: represent a node that is identified by a regular expression
/// - Wildcard: represent tha catch all node
///
#[derive(Clone)]
pub enum NodeType {
    Static(Vec<u8>),
    Regex(Regex),
    Wildcard(),
}

impl NodeType {
    pub fn r#static(&self) -> Vec<u8> {
        match self {
            NodeType::Static(p) => p.clone(),
            _ => panic!("Not static node type!"),
        }
    }
}

/// Struct for representing a node into the RadixTree.
///
/// This struct is used for keeping some information about which
/// kind of type, the value and the children
#[derive(Clone)]
pub struct Node<T> {
    pub node_type: NodeType,
    pub value: Option<T>,
    pub static_children: Vec<Node<T>>,
    pub regex_children: Vec<Node<T>>,
    pub wildcard_children: Vec<Node<T>>,
}

impl PartialEq for NodeType {
    fn eq(&self, other: &NodeType) -> bool {
        match (self, other) {
            (NodeType::Static(s1), NodeType::Static(s2)) => s1 == s2,
            (NodeType::Regex(r1), NodeType::Regex(r2)) => r1.as_str() == r2.as_str(),
            (NodeType::Wildcard(), NodeType::Wildcard()) => true,
            _ => false,
        }
    }
}

impl<'node, T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.node_type == other.node_type
            && self.static_children == other.static_children
            && self.regex_children == other.regex_children
            && self.wildcard_children == other.wildcard_children
    }
}
