use crate::node::{Node, NodeType};

/// Optimize the radix tree returning the new optimized instance of Node.
///
/// # Examples
///
/// ```
/// use bravery_router::{create_root_node, add, optimize};
/// let mut node = create_root_node();
/// add(&mut node, "/foo", 1);
/// let optmized = optimize(node);
/// ```
pub fn optimize<T>(mut root: Node<T>) -> Node<T> {
    match &root.node_type {
        NodeType::Static(p1) => {
            if root.wildcard_children.is_empty()
                && root.regex_children.is_empty()
                && root.value.is_none()
                && (root.static_children.len() == 1)
            {
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
                            regex_children: child.regex_children,
                            wildcard_children: child.wildcard_children,
                        })
                    }
                    _ => panic!(),
                }
            } else {
                root.static_children = root.static_children.into_iter().map(optimize).collect();

                root.regex_children = root.regex_children.into_iter().map(optimize).collect();

                root.wildcard_children = root.wildcard_children.into_iter().map(optimize).collect();

                root
            }
        }
        _ => {
            root.static_children = root.static_children.into_iter().map(optimize).collect();

            root.regex_children = root.regex_children.into_iter().map(optimize).collect();

            root.wildcard_children = root.wildcard_children.into_iter().map(optimize).collect();

            root
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn trivial_case() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(0),
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/']),
                value: Some(0),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            }
        );
    }

    #[test]
    fn static_one_neasting() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![Node {
                node_type: NodeType::Static(vec![b'a']),
                value: Some(0),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/', b'a']),
                value: Some(0),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            }
        );
    }

    #[test]
    fn static_cannot_neasting_for_value() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(1),
            static_children: vec![Node {
                node_type: NodeType::Static(vec![b'a']),
                value: Some(0),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/']),
                value: Some(1),
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'a']),
                    value: Some(0),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                regex_children: vec![],
                wildcard_children: vec![],
            }
        );
    }

    #[test]
    fn static_neasting_but_not_on_root() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(1),
            static_children: vec![Node {
                node_type: NodeType::Static(vec![b'a']),
                value: None,
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'b']),
                    value: Some(2),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/']),
                value: Some(1),
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'a', b'b']),
                    value: Some(2),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                regex_children: vec![],
                wildcard_children: vec![],
            }
        );
    }

    #[test]
    fn static_multiple_neasting() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![Node {
                node_type: NodeType::Static(vec![b'a']),
                value: None,
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'b']),
                    value: None,
                    static_children: vec![Node {
                        node_type: NodeType::Static(vec![b'c']),
                        value: None,
                        static_children: vec![Node {
                            node_type: NodeType::Static(vec![b'd']),
                            value: None,
                            static_children: vec![Node {
                                node_type: NodeType::Static(vec![b'e']),
                                value: None,
                                static_children: vec![Node {
                                    node_type: NodeType::Static(vec![b'f']),
                                    value: Some(1),
                                    static_children: vec![],
                                    regex_children: vec![],
                                    wildcard_children: vec![],
                                }],
                                regex_children: vec![],
                                wildcard_children: vec![],
                            }],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }],
                        regex_children: vec![],
                        wildcard_children: vec![],
                    }],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/', b'a', b'b', b'c', b'd', b'e', b'f']),
                value: Some(1),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            }
        );
    }

    #[test]
    fn regex_is_untouched() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![Node {
                node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                value: Some(0),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            wildcard_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/']),
                value: None,
                static_children: vec![],
                regex_children: vec![Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(0),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                wildcard_children: vec![],
            }
        );
    }

    #[test]
    fn regex_with_static_is_not_untouched() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![Node {
                node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                value: Some(0),
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'/']),
                    value: None,
                    static_children: vec![Node {
                        node_type: NodeType::Static(vec![b'b']),
                        value: Some(2),
                        static_children: vec![],
                        regex_children: vec![],
                        wildcard_children: vec![],
                    }],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            wildcard_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/']),
                value: None,
                static_children: vec![],
                regex_children: vec![Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(0),
                    static_children: vec![Node {
                        node_type: NodeType::Static(vec![b'/', b'b']),
                        value: Some(2),
                        static_children: vec![],
                        regex_children: vec![],
                        wildcard_children: vec![],
                    }],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                wildcard_children: vec![],
            }
        );
    }

    #[test]
    fn regex_with_neasted_static_is_not_untouched() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![Node {
                node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                value: Some(0),
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'/']),
                    value: None,
                    static_children: vec![Node {
                        node_type: NodeType::Static(vec![b'b']),
                        value: Some(2),
                        static_children: vec![],
                        regex_children: vec![],
                        wildcard_children: vec![],
                    }],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                regex_children: vec![Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(0),
                    static_children: vec![Node {
                        node_type: NodeType::Static(vec![b'/']),
                        value: None,
                        static_children: vec![Node {
                            node_type: NodeType::Static(vec![b'c']),
                            value: Some(2),
                            static_children: vec![],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }],
                        regex_children: vec![],
                        wildcard_children: vec![],
                    }],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                wildcard_children: vec![],
            }],
            wildcard_children: vec![],
        };

        let optimized = optimize(root);
        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/']),
                value: None,
                static_children: vec![],
                regex_children: vec![Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(0),
                    static_children: vec![Node {
                        node_type: NodeType::Static(vec![b'/', b'b']),
                        value: Some(2),
                        static_children: vec![],
                        regex_children: vec![],
                        wildcard_children: vec![],
                    }],
                    regex_children: vec![Node {
                        node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                        value: Some(0),
                        static_children: vec![Node {
                            node_type: NodeType::Static(vec![b'/', b'c']),
                            value: Some(2),
                            static_children: vec![],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }],
                        regex_children: vec![],
                        wildcard_children: vec![],
                    }],
                    wildcard_children: vec![],
                }],
                wildcard_children: vec![],
            }
        );
    }

    #[test]
    fn static_with_wildcard() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![Node {
                node_type: NodeType::Wildcard(),
                value: Some(1),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
        };

        let optimized = optimize(root);

        println!("{:?}", optimized);

        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/']),
                value: None,
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![Node {
                    node_type: NodeType::Wildcard(),
                    value: Some(1),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
            }
        );
    }

    #[test]
    fn static_with_wildcard_2() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![Node {
                node_type: NodeType::Static(vec![b'f']),
                value: None,
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'o']),
                    value: None,
                    static_children: vec![Node {
                        node_type: NodeType::Static(vec![b'o']),
                        value: None,
                        static_children: vec![],
                        regex_children: vec![],
                        wildcard_children: vec![Node {
                            node_type: NodeType::Wildcard(),
                            value: Some(1),
                            static_children: vec![],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }],
                    }],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        let optimized = optimize(root);

        assert_eq!(
            optimized,
            Node {
                node_type: NodeType::Static(vec![b'/', b'f', b'o', b'o']),
                value: None,
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![Node {
                    node_type: NodeType::Wildcard(),
                    value: Some(1),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
            }
        );
    }
}
