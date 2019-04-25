use std::fmt::Debug;
use regex::Regex;

pub const MAX_NEASTING_LEVEL_COUNT: usize = 16;

pub enum NodeType {
    Static(&'static [u8]),
    Regex(Regex),
}

pub struct Node<T: Debug + 'static> {
    pub node_type: NodeType,
    pub value: Option<&'static T>,
    pub static_children: Vec<Node<T>>,
    pub regex_children: Vec<Node<T>>,
}
