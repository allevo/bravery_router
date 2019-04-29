use regex::Regex;

pub const MAX_NEASTING_LEVEL_COUNT: usize = 16;

#[derive(Clone)]
pub enum NodeType {
    Static(Vec<u8>),
    Regex(Regex),
}

pub struct Node<T: PartialEq + 'static> {
    pub node_type: NodeType,
    pub value: Option<&'static T>,
    pub static_children: Vec<Node<T>>,
    pub regex_children: Vec<Node<T>>,
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
            _ => false,
        }
    }
}

impl<T: PartialEq + 'static> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.static_children == other.static_children
            && self.regex_children == other.regex_children
            && self.value == other.value
            && self.node_type == other.node_type
    }
}
