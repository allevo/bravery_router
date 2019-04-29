
use crate::node::{Node, NodeType};

pub fn optimize<T: PartialEq> (mut root: Node<T>) -> Node<T> {
    match &root.node_type {
        NodeType::Static(p1) => {
            if root.regex_children.is_empty() && root.value.is_none() && (root.static_children.len() == 1) {
                let child = root.static_children.pop().unwrap();
                match &child.node_type {
                    NodeType::Static(p2) => {
                        let mut n = Vec::new();
                        n.extend(p1);
                        n.extend(p2);

                        optimize(Node {
                            node_type: NodeType::Static(n),
                            value: child.value,
                            static_children: child.static_children,
                            regex_children: Vec::new(),
                        })
                    },
                    _ => panic!(),
                }
            } else {
                root.static_children = root.static_children.into_iter()
                    .map(optimize)
                    .collect();

                root.regex_children = root.regex_children.into_iter()
                    .map(optimize)
                    .collect();

                root
            }
        },
        _ => {
            root.static_children = root.static_children.into_iter()
                .map(optimize)
                .collect();

            root.regex_children = root.regex_children.into_iter()
                .map(optimize)
                .collect();

            root
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    impl std::fmt::Debug for NodeType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                NodeType::Static(p) => write!(f, "NodeType::Static {:?}", p),
                NodeType::Regex(r) => write!(f, "NodeType::Regex {:?}", r),
            }
        }
    }

    impl std::fmt::Debug for Node<u8> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Node {{ t: {:?} v: {:?} s: {:?} r: {:?} }}", self.node_type, self.value, self.static_children, self.regex_children)
        }
    }

    #[test]
    fn trivial_case() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&0),
            static_children: vec![],
            regex_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(optimized, Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&0),
            static_children: vec![],
            regex_children: vec![],
        });
    }

    #[test]
    fn static_one_neasting() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'a']),
                    value: Some(&0),
                    static_children: vec![],
                    regex_children: vec![],
                }
            ],
            regex_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(optimized, Node {
            node_type: NodeType::Static(vec![b'/', b'a']),
            value: Some(&0),
            static_children: vec![],
            regex_children: vec![],
        });
    }

    #[test]
    fn static_cannot_neasting_for_value() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&1),
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'a']),
                    value: Some(&0),
                    static_children: vec![],
                    regex_children: vec![],
                }
            ],
            regex_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(optimized, Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&1),
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'a']),
                    value: Some(&0),
                    static_children: vec![],
                    regex_children: vec![],
                }
            ],
            regex_children: vec![],
        });
    }

    #[test]
    fn static_neasting_but_not_on_root() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&1),
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'a']),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'b']),
                            value: Some(&2),
                            static_children: vec![],
                            regex_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                }
            ],
            regex_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(optimized, Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&1),
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'a', b'b']),
                    value: Some(&2),
                    static_children: vec![],
                    regex_children: vec![],
                }
            ],
            regex_children: vec![],
        });
    }

    #[test]
    fn static_multiple_neasting() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'a']),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'b']),
                            value: None,
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(vec![b'c']),
                                    value: None,
                                    static_children: vec![
                                        Node {
                                            node_type: NodeType::Static(vec![b'd']),
                                            value: None,
                                            static_children: vec![
                                                Node {
                                                    node_type: NodeType::Static(vec![b'e']),
                                                    value: None,
                                                    static_children: vec![
                                                        Node {
                                                            node_type: NodeType::Static(vec![b'f']),
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
                                }
                            ],
                            regex_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                }
            ],
            regex_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(optimized, Node {
            node_type: NodeType::Static(vec![b'/', b'a', b'b', b'c', b'd', b'e', b'f']),
            value: Some(&1),
            static_children: vec![],
            regex_children: vec![],
        });
    }

    #[test]
    fn regex_is_untouched() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(&0),
                    static_children: vec![],
                    regex_children: vec![],
                }
            ],
        };

        let optimized = optimize(root);
        assert_eq!(optimized, Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(&0),
                    static_children: vec![],
                    regex_children: vec![],
                }
            ],
        });
    }

    #[test]
    fn regex_with_static_is_not_untouched() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(&0),
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'/']),
                            value: None,
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(vec![b'b']),
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
        };

        let optimized = optimize(root);
        assert_eq!(optimized, Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(&0),
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'/', b'b']),
                            value: Some(&2),
                            static_children: vec![],
                            regex_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                }
            ],
        });
    }

    #[test]
    fn regex_with_neasted_static_is_not_untouched() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(&0),
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'/']),
                            value: None,
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(vec![b'b']),
                                    value: Some(&2),
                                    static_children: vec![],
                                    regex_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                        }
                    ],
                    regex_children: vec![
                        Node {
                            node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                            value: Some(&0),
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(vec![b'/']),
                                    value: None,
                                    static_children: vec![
                                        Node {
                                            node_type: NodeType::Static(vec![b'c']),
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
                }
            ],
        };

        let optimized = optimize(root);
        assert_eq!(optimized, Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(&0),
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'/', b'b']),
                            value: Some(&2),
                            static_children: vec![],
                            regex_children: vec![],
                        }
                    ],
                    regex_children: vec![
                        Node {
                            node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                            value: Some(&0),
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(vec![b'/', b'c']),
                                    value: Some(&2),
                                    static_children: vec![],
                                    regex_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                        }
                    ],
                }
            ],
        });
    }
}
