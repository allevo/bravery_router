
use crate::node::{Node, NodeType};

fn add_inner<T: PartialEq> (root: &mut Node<T>, path: &[u8], value: &'static T, index: usize) {
    if index == path.len() {
        if root.value.is_some() {
            panic!("Value already present!")
        }
        root.value = Some(value);
        return;
    }

    if path[index] == b':' {
        unimplemented!()
    }

    let child_pos = root.static_children.iter()
        .position(|sc| sc.node_type.r#static()[0] == path[index]);
    let mut child: Node<T> = match child_pos {
        Some(p) => root.static_children.remove(p),
        None => Node {
            node_type: NodeType::Static(vec![path[index]]),
            value: None,
            static_children: vec![],
            regex_children: vec![],
        }
    };

    add_inner(&mut child, path, value, index + 1);

    root.static_children.push(child);
}

pub fn add<T: PartialEq> (root: &mut Node<T>, path: &str, value: &'static T) {
    let path = path.as_bytes();
    let path = if path[0] == b'/' { &path[1..] } else { path };
    add_inner(root, path, value, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    // use regex::Regex;

    #[test]
    fn add_one() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&0),
            static_children: vec![],
            regex_children: vec![],
        };

        add(&mut root, "/foo", &1);
        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: Some(&0),
            static_children: vec![
                Node {
                    node_type: NodeType::Static(b"f".to_vec()),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(b"o".to_vec()),
                            value: None,
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(b"o".to_vec()),
                                    value: Some(&1),
                                    static_children: vec![],
                                    regex_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                }
            ],
            regex_children: vec![],
        });
    }

    #[test]
    fn add_more_than_one() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&0),
            static_children: vec![],
            regex_children: vec![],
        };

        add(&mut root, "/foo", &1);
        add(&mut root, "/foobar", &2);
        add(&mut root, "/fo", &3);
        add(&mut root, "/bar", &4);
        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: Some(&0),
            static_children: vec![
                Node {
                    node_type: NodeType::Static(b"f".to_vec()),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(b"o".to_vec()),
                            value: Some(&3),
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(b"o".to_vec()),
                                    value: Some(&1),
                                    static_children: vec![
                                        Node {
                                            node_type: NodeType::Static(b"b".to_vec()),
                                            value: None,
                                            static_children: vec![
                                                Node {
                                                    node_type: NodeType::Static(b"a".to_vec()),
                                                    value: None,
                                                    static_children: vec![
                                                        Node {
                                                            node_type: NodeType::Static(b"r".to_vec()),
                                                            value: Some(&2),
                                                            static_children: vec![],
                                                            regex_children: vec![],
                                                        }
                                                    ],
                                                    regex_children: vec![],
                                                }
                                            ],
                                            regex_children: vec![],
                                        }
                                    ],
                                    regex_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                },
                Node {
                    node_type: NodeType::Static(b"b".to_vec()),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(b"a".to_vec()),
                            value: None,
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(b"r".to_vec()),
                                    value: Some(&4),
                                    static_children: vec![],
                                    regex_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                }
            ],
            regex_children: vec![],
        });
    }
}
