
use crate::node::{Node, NodeType};
use regex::Regex;

fn add_inner<T: PartialEq> (root: &mut Node<T>, path: &[u8], value: &'static T, index: usize) {
    if index == path.len() {
        if root.value.is_some() {
            panic!("Value already present!")
        }
        root.value = Some(value);
        return;
    }

    if path[index] == b':' {
        if root.regex_children.is_empty() {
            let next_path = path.iter()
                .skip(index)
                .position(|&x| x == b'/')
                .unwrap_or_else(|| path.len());

            let mut child = Node {
                node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                value: if next_path == path.len() { Some(value) } else { None },
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            };

            if next_path != path.len() {
                add_inner(&mut child, path, value, next_path + index);
            }

            root.regex_children.push(child);
            return
        } else {
            let mut child: Node<T> = root.regex_children.remove(0);
            
            let next_path = path.iter()
                .skip(index)
                .position(|&x| x == b'/')
                .unwrap_or_else(|| path.len());
            
            if child.value.is_some() && next_path == path.len() {
                panic!("Value already present!")
            }

            if next_path == path.len() {
                child.value = Some(value);
            }

            if next_path != path.len() {
                add_inner(&mut child, path, value, next_path + index);
            }

            root.regex_children.push(child);

            return
        }
    }
    if path[index] == b'*' {
        let child = Node {
            node_type: NodeType::Wildcard(),
            value: Some(value),
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };
        root.wildcard_children.push(child);

        if index != path.len() - 1 {
            unimplemented!("Wildcard should be at the end of the path");
        }
        return
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
            wildcard_children: vec![],
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
    use regex::Regex;

    #[test]
    fn add_one() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&0),
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
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
                                    wildcard_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }
            ],
            regex_children: vec![],
            wildcard_children: vec![],
        });
    }

    #[test]
    fn add_more_than_one() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&0),
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
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
                                                            wildcard_children: vec![],
                                                        }
                                                    ],
                                                    regex_children: vec![],
                                                    wildcard_children: vec![],
                                                }
                                            ],
                                            regex_children: vec![],
                                            wildcard_children: vec![],
                                        }
                                    ],
                                    regex_children: vec![],
                                    wildcard_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
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
                                    wildcard_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }
            ],
            regex_children: vec![],
            wildcard_children: vec![],
        });
    }

    #[test]
    fn add_regex_named_on_root() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&0),
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/:name", &1);

        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: Some(&0),
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                    value: Some(&1),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                },
            ],
            wildcard_children: vec![],
        });
    }

    #[test]
    fn add_regex_named_neasted() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/foo/:name", &1);

        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: None
            ,
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'f']),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'o']),
                            value: None,
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(vec![b'o']),
                                    value: None,
                                    static_children: vec![
                                        Node {
                                            node_type: NodeType::Static(vec![b'/']),
                                            value: None,
                                            static_children: vec![],
                                            regex_children: vec![
                                                Node {
                                                    node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                                                    value: Some(&1),
                                                    static_children: vec![],
                                                    regex_children: vec![],
                                                    wildcard_children: vec![],
                                                },
                                            ],
                                            wildcard_children: vec![],
                                        }
                                    ],
                                    regex_children: vec![],
                                    wildcard_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }
            ],
            regex_children: vec![],
            wildcard_children: vec![],
        });
    }

    #[test]
    fn add_regex_named_before_static() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/:name/bar", &1);

        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: None,
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'/']),
                            value: None,
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(vec![b'b']),
                                    value: None,
                                    static_children: vec![
                                        Node {
                                            node_type: NodeType::Static(vec![b'a']),
                                            value: None,
                                            static_children: vec![
                                                Node {
                                                    node_type: NodeType::Static(vec![b'r']),
                                                    value: Some(&1),
                                                    static_children: vec![],
                                                    regex_children: vec![],
                                                    wildcard_children: vec![],
                                                }
                                            ],
                                            regex_children: vec![],
                                            wildcard_children: vec![],
                                        }
                                    ],
                                    regex_children: vec![],
                                    wildcard_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
                },
            ],
            wildcard_children: vec![],
        });
    }

    #[test]
    fn add_regex_named_after_static() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/foo/:name", &1);

        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: None,
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'f']),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'o']),
                            value: None,
                            static_children: vec![
                                Node {
                                    node_type: NodeType::Static(vec![b'o']),
                                    value: None,
                                    static_children: vec![
                                        Node {
                                            node_type: NodeType::Static(vec![b'/']),
                                            value: None,
                                            static_children: vec![],
                                            regex_children: vec![
                                                Node {
                                                    node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                                                    value: Some(&1),
                                                    static_children: vec![],
                                                    regex_children: vec![],
                                                    wildcard_children: vec![],
                                                },
                                            ],
                                            wildcard_children: vec![],
                                        }
                                    ],
                                    regex_children: vec![],
                                    wildcard_children: vec![],
                                }
                            ],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }
            ],
            regex_children: vec![],
            wildcard_children: vec![],
        });
    }

    #[test]
    fn add_regex_multiple() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/:name/:surname/:age", &1);

        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: None,
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'/']),
                            value: None,
                            static_children: vec![],
                            regex_children: vec![
                                Node {
                                    node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                                    value: None,
                                    static_children: vec![
                                        Node {
                                            node_type: NodeType::Static(vec![b'/']),
                                            value: None,
                                            static_children: vec![],
                                            regex_children: vec![
                                                Node {
                                                    node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                                                    value: Some(&1),
                                                    static_children: vec![],
                                                    regex_children: vec![],
                                                    wildcard_children: vec![],
                                                },
                                            ],
                                            wildcard_children: vec![],
                                        }
                                    ],
                                    regex_children: vec![],
                                    wildcard_children: vec![],
                                },
                            ],
                            wildcard_children: vec![],
                        }
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
                },
            ],
            wildcard_children: vec![],
        });
    }

    #[test]
    fn add_wildcard_on_root() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/*", &1);

        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![
                Node {
                    node_type: NodeType::Wildcard(),
                    value: Some(&1),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![]
                },
            ],
        });
    }

    #[test]
    fn add_wildcard_after_static() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/foo/*", &1);

        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: None,
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
                                    value: None,
                                    static_children: vec![
                                        Node {
                                            node_type: NodeType::Static(b"/".to_vec()),
                                            value: None,
                                            static_children: vec![],
                                            regex_children: vec![],
                                            wildcard_children: vec![
                                                Node {
                                                    node_type: NodeType::Wildcard(),
                                                    value: Some(&1),
                                                    static_children: vec![],
                                                    regex_children: vec![],
                                                    wildcard_children: vec![]
                                                },
                                            ],
                                        },
                                    ],
                                    regex_children: vec![],
                                    wildcard_children: vec![],
                                },
                            ],
                            regex_children: vec![],
                            wildcard_children: vec![],
                        },
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
                },
            ],
            regex_children: vec![],
            wildcard_children: vec![],
        });
    }

    #[test]
    fn add_wildcard_after_regex() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/:name/*", &1);

        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: None,
            static_children: vec![],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(b"/".to_vec()),
                            value: None,
                            static_children: vec![],
                            regex_children: vec![],
                            wildcard_children: vec![
                                Node {
                                    node_type: NodeType::Wildcard(),
                                    value: Some(&1),
                                    static_children: vec![],
                                    regex_children: vec![],
                                    wildcard_children: vec![],
                                },
                            ],
                        },
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
                },
            ],
            wildcard_children: vec![],
        });
    }

    #[test]
    #[should_panic]
    fn add_wildcard_before_something() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/*/:name", &1);
    }

    #[test]
    fn two_regex() {
        let mut root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![],
            wildcard_children: vec![],
        };

        add(&mut root, "/p/:pid/c/", &1);
        add(&mut root, "/p/:pid/c", &2);

        assert_eq!(root, Node {
            node_type: NodeType::Static(b"/".to_vec()),
            value: None,
            static_children: vec![
                Node {
                    node_type: NodeType::Static(b"p".to_vec()),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(b"/".to_vec()),
                            value: None,
                            static_children: vec![],
                            regex_children: vec![
                                Node {
                                    node_type: NodeType::Regex(Regex::new("^([^/]+|$)").unwrap()),
                                    value: None,
                                    static_children: vec![
                                        Node {
                                            node_type: NodeType::Static(b"/".to_vec()),
                                            value: None,
                                            static_children: vec![
                                                Node {
                                                    node_type: NodeType::Static(b"c".to_vec()),
                                                    value: Some(&2),
                                                    static_children: vec![
                                                        Node {
                                                            node_type: NodeType::Static(b"/".to_vec()),
                                                            value: Some(&1),
                                                            static_children: vec![],
                                                            regex_children: vec![],
                                                            wildcard_children: vec![],
                                                        },
                                                    ],
                                                    regex_children: vec![],
                                                    wildcard_children: vec![],
                                                },
                                            ],
                                            regex_children: vec![],
                                            wildcard_children: vec![],
                                        },
                                    ],
                                    regex_children: vec![],
                                    wildcard_children: vec![],
                                },
                            ],
                            wildcard_children: vec![],
                        },
                    ],
                    regex_children: vec![],
                    wildcard_children: vec![],
                },
            ],
            regex_children: vec![],
            wildcard_children: vec![],
        });
    }
}
