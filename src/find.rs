use crate::node::MAX_NEASTING_LEVEL_COUNT;
pub use crate::node::{Node, NodeType};
use regex::Regex;

/// Struct for containing the result of the `find` function.
#[derive(Debug)]
pub struct FindResult<'req, T: Clone> {
    pub value: Option<&'req T>,
    pub params: Vec<&'req str>,
}

#[derive(Debug)]
struct FindState<'req> {
    index: usize,
    steps: [usize; MAX_NEASTING_LEVEL_COUNT],
    step_number: usize,
    params: [&'req str; MAX_NEASTING_LEVEL_COUNT],
    param_number: usize,
}

impl<'req> FindState<'req> {
    fn inc(&mut self, n: usize) {
        self.index += n;
        self.steps[self.step_number] = n;
        self.step_number += 1;

        self.params[self.param_number] = "";
        self.param_number += 1;
    }

    fn inc_with_value(&mut self, n: usize, p: &'req str) {
        self.inc(n);
        self.params[self.param_number - 1] = p;
    }

    fn pop(&mut self) {
        self.step_number -= 1;
        self.index -= self.steps[self.step_number];

        self.param_number -= 1;
    }
}

lazy_static! {
    static ref WILD_CARD_REGEX: Regex = Regex::new("^(.+)$").unwrap();
}

fn get_value_pointer_from_node<'req, T>(node: &'req Node<T>) -> Option<&T> {
    node.value.as_ref()
}

fn find_inner<'req, T>(
    node: &'req Node<T>,
    path: &'req str,
    path_bytes: &'req [u8],
    state: &mut FindState<'req>,
) -> Option<&'req T> {
    match &node.node_type {
        NodeType::Static(p) => {
            if *p == &path_bytes[state.index..] {
                if node.value.is_some() {
                    trace!(
                        "Exit with static! {} {}",
                        std::str::from_utf8(&path_bytes[state.index..]).unwrap(),
                        std::str::from_utf8(&*p).unwrap()
                    );
                    return get_value_pointer_from_node(node);
                }
                state.inc(p.len());
                return None;
            } else {
                state.inc(p.len());
            }
        }
        NodeType::Regex(regex) => {
            let res = regex.captures(&path[state.index..]).unwrap();
            let res = res.get(0).unwrap();
            let res = res.as_str();

            trace!("Matched {} {}", res, res.len());

            let len = res.len();

            state.inc_with_value(len, res);
            if state.index == path.len() {
                if node.value.is_some() {
                    trace!("Exit with regex");
                    return get_value_pointer_from_node(node);
                }
                return None;
            }
        }
        NodeType::Wildcard() => {
            let res = WILD_CARD_REGEX.captures(&path[state.index..]).unwrap();
            let res = res.get(0).unwrap();
            let res = res.as_str();

            trace!("Matched {} {}", res, res.len());

            let len = res.len();

            state.inc_with_value(len, res);

            trace!("Exit with wildcard");
            return get_value_pointer_from_node(node);
        }
    }

    trace!("rest: {}, index: {}", &path[state.index..], state.index);

    trace!("Start found children");

    let child = node.static_children.iter().find(|sc| match &sc.node_type {
        NodeType::Static(sp) => {
            for i in 0..sp.len() {
                if path_bytes.len() <= state.index + i || sp[i] != path_bytes[state.index + i] {
                    return false;
                }
            }

            true
        }
        _ => unimplemented!(),
    });

    if child.is_some() {
        trace!("Child static found");
        let r = find_inner(child.unwrap(), path, path_bytes, state);
        if r.is_some() {
            return r;
        }
        trace!("Child static poped!");
        state.pop();
    }

    trace!("Trying regex index: {}", state.index);

    let child = node.regex_children.iter().find(|sc| match &sc.node_type {
        NodeType::Regex(regex) => {
            trace!("checking {:?}... {}", regex, &path[state.index..]);
            regex.is_match(&path[state.index..])
        }
        _ => unimplemented!(),
    });

    if child.is_some() {
        trace!("Child regex found");
        let r = find_inner(child.unwrap(), path, path_bytes, state);
        if r.is_some() {
            return r;
        }
        trace!("Child regex poped!");
        state.pop();
    }

    trace!("Trying wildcard index: {}", state.index);

    let child = node.wildcard_children.get(0);
    if child.is_some() {
        trace!("Child wildcard found");
        let r = find_inner(child.unwrap(), path, path_bytes, state);
        if r.is_some() {
            return r;
        }
        trace!("Child wildcard poped!");
        state.pop();
    }

    trace!("No found in the branch!");

    None
}

/// Find in the radix tree the path and return a `FindResult`.
///
/// There're some precendence:
/// 1. every Static nodes are compared
/// 1. then every Regex nodes are compared
/// 1. then the Wildcard is applied
///
/// # Examples
///
/// ```
/// use bravery_router::{create_root_node, add, find};
/// let mut node = create_root_node();
/// add(&mut node, "/foo", 1);
///
/// let find_result = find(&node, "/foo").value.unwrap();
/// ```
pub fn find<'req, T: Clone>(node: &'req Node<T>, path: &'req str) -> FindResult<'req, T> {
    let mut find_state = FindState {
        index: 0,
        steps: [0; MAX_NEASTING_LEVEL_COUNT],
        step_number: 0,
        params: [""; MAX_NEASTING_LEVEL_COUNT],
        param_number: 0,
    };
    let value = find_inner(node, path, path.as_bytes(), &mut find_state);
    if value.is_none() {
        return FindResult {
            value: None,
            params: vec![],
        };
    }

    let mut params = find_state.params[0..find_state.param_number].to_vec();
    params.retain(|x| !x.is_empty());

    FindResult { value, params }
}

#[cfg(test)]
mod tests {
    use super::*;

    use regex::Regex;

    #[test]
    fn get_root() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(0),
            static_children: Vec::new(),
            regex_children: Vec::new(),
            wildcard_children: vec![],
        };

        let output = find(&root, "/");
        assert_eq!(
            FindResult {
                value: Some(&0),
                params: vec![]
            },
            output
        );

        let output = find(&root, "b");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "/b");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );
    }

    #[test]
    fn get_static_child() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'a']),
                    value: Some(1),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                },
                Node {
                    node_type: NodeType::Static(vec![b'b']),
                    value: Some(2),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                },
                Node {
                    node_type: NodeType::Static(vec![b'c']),
                    value: Some(3),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                },
            ],
            regex_children: Vec::new(),
            wildcard_children: vec![],
        };

        let output = find(&root, "/a");
        assert_eq!(
            FindResult {
                value: Some(&1),
                params: vec![]
            },
            output
        );

        let output = find(&root, "/b");
        assert_eq!(
            FindResult {
                value: Some(&2),
                params: vec![]
            },
            output
        );

        let output = find(&root, "/c");
        assert_eq!(
            FindResult {
                value: Some(&3),
                params: vec![]
            },
            output
        );

        let output = find(&root, "/aa");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "/z");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "/");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );
    }

    #[test]
    fn get_regex_child() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: Vec::new(),
            regex_children: vec![Node {
                node_type: NodeType::Regex(Regex::new("^(\\d+)").unwrap()),
                value: Some(1),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            wildcard_children: vec![],
        };

        let output = find(&root, "/1");
        assert_eq!(
            FindResult {
                value: Some(&1),
                params: vec!["1"]
            },
            output
        );

        let output = find(&root, "/b");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "/aa");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "b");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "/");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );
    }

    #[test]
    fn get_static_fallback() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![Node {
                node_type: NodeType::Static(vec![b'1']),
                value: None,
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'a']),
                    value: Some(11),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            regex_children: vec![Node {
                node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                value: Some(1),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            wildcard_children: vec![],
        };

        let output = find(&root, "/1a");
        assert_eq!(
            FindResult {
                value: Some(&11),
                params: vec![]
            },
            output
        );

        let output = find(&root, "/1");
        assert_eq!(
            FindResult {
                value: Some(&1),
                params: vec!["1"]
            },
            output
        );

        let output = find(&root, "/11");
        assert_eq!(
            FindResult {
                value: Some(&1),
                params: vec!["11"]
            },
            output
        );

        let output = find(&root, "/");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "/z");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );
    }

    #[test]
    fn get_regex_fallback() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![Node {
                node_type: NodeType::Static(vec![b'1']),
                value: None,
                static_children: vec![],
                regex_children: vec![Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: None, // unuseful node!
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                wildcard_children: vec![],
            }],
            regex_children: vec![Node {
                node_type: NodeType::Regex(Regex::new(r"^(\d)").unwrap()),
                value: None,
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'1']),
                    value: Some(1),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            wildcard_children: vec![],
        };

        let output = find(&root, "/11");
        assert_eq!(
            FindResult {
                value: Some(&1),
                params: vec!["1"]
            },
            output
        );

        let output = find(&root, "/1a");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "/1");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "/");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );

        let output = find(&root, "/z");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );
    }

    #[test]
    fn get_wildcard() {
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

        let output = find(&root, "/11");
        assert_eq!(
            FindResult {
                value: Some(&1),
                params: vec!["11"]
            },
            output
        );

        let output = find(&root, "/1a");
        assert_eq!(
            FindResult {
                value: Some(&1),
                params: vec!["1a"]
            },
            output
        );

        let output = find(&root, "/1a/bar/foo");
        assert_eq!(
            FindResult {
                value: Some(&1),
                params: vec!["1a/bar/foo"]
            },
            output
        );
    }

    #[test]
    fn get_regex_and_wildcard() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![Node {
                node_type: NodeType::Regex(Regex::new("^([^/]+)").unwrap()),
                value: Some(1),
                static_children: vec![],
                regex_children: vec![],
                wildcard_children: vec![Node {
                    node_type: NodeType::Wildcard(),
                    value: Some(2),
                    static_children: vec![],
                    regex_children: vec![],
                    wildcard_children: vec![],
                }],
            }],
            wildcard_children: vec![],
        };

        let output = find(&root, "/1a/bar/foo");
        assert_eq!(
            FindResult {
                value: Some(&2),
                params: vec!["1a", "/bar/foo"]
            },
            output
        );
    }

    #[test]
    fn get_double_regex() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/', b'p', b'/']),
            value: None,
            static_children: vec![],
            regex_children: vec![Node {
                node_type: NodeType::Regex(Regex::new("^([^/]+)").unwrap()),
                value: None,
                static_children: vec![Node {
                    node_type: NodeType::Static(vec![b'/', b'c', b'/']),
                    value: None,
                    static_children: vec![],
                    regex_children: vec![Node {
                        node_type: NodeType::Regex(Regex::new("^([^/]+)").unwrap()),
                        value: Some(1),
                        static_children: vec![],
                        regex_children: vec![],
                        wildcard_children: vec![],
                    }],
                    wildcard_children: vec![],
                }],
                regex_children: vec![],
                wildcard_children: vec![],
            }],
            wildcard_children: vec![],
        };

        let output = find(&root, "/p/bar/c/foo");
        assert_eq!(
            FindResult {
                value: Some(&1),
                params: vec!["bar", "foo"]
            },
            output
        );

        let output = find(&root, "/p/bar/c");
        assert_eq!(
            FindResult {
                value: None,
                params: vec![]
            },
            output
        );
    }
}
