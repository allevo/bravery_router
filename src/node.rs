use regex::Regex;

pub const MAX_NEASTING_LEVEL_COUNT: usize = 16;

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

pub struct Node<T: PartialEq> {
    pub node_type: NodeType,
    pub value: Option<T>,
    pub static_children: Vec<Node<T>>,
    pub regex_children: Vec<Node<T>>,
    pub wildcard_children: Vec<Node<T>>,
}

impl PartialEq for NodeType {
    fn eq(&self, other: &NodeType) -> bool {
        match (self, other) {
            (NodeType::Static(s1), NodeType::Static(s2)) => {
                s1 == s2
            },
            (NodeType::Regex(r1), NodeType::Regex(r2)) => {
                r1.as_str() == r2.as_str()
            },
            (NodeType::Wildcard(), NodeType::Wildcard()) => {
                true
            },
            _ => false,
        }
    }
}

impl<'node, T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.static_children == other.static_children
            && self.regex_children == other.regex_children
            && self.wildcard_children == other.wildcard_children
            && self.value == other.value
            && self.node_type == other.node_type
    }
}
