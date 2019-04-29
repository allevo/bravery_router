use crate::node::{MAX_NEASTING_LEVEL_COUNT};
pub use crate::node::{NodeType, Node};

#[derive(Debug, PartialEq)]
pub struct FindResult<'a, T: PartialEq + 'static> {
    pub value: Option<&'static T>,
    pub params: Vec<&'a str>,
}

#[derive(Debug)]
struct FindState<'a, T: PartialEq + 'static> {
    index: usize,
    steps: [usize; MAX_NEASTING_LEVEL_COUNT],
    step_number: usize,
    value: Option<&'static T>,
    params: [&'a str; MAX_NEASTING_LEVEL_COUNT],
    param_number: usize,
}

impl<'a, T: PartialEq + 'static> FindState<'a, T> {
    fn inc (&mut self, n: usize) {
        self.index += n;
        self.steps[self.step_number] = n;
        self.step_number += 1;

        self.params[self.param_number] = "";
        self.param_number += 1;
    }

    fn inc_with_value (&mut self, n: usize, p: &'a str) {
        self.inc(n);
        self.params[self.param_number - 1] = p;
    }

    fn pop (&mut self) {
        self.step_number -= 1;
        self.index -= self.steps[self.step_number];

        self.param_number -= 1;
    }
}

fn find_inner<'a, T: PartialEq> (node: &Node<T>, path: &'a str, path_bytes: &'a [u8], mut state: &mut FindState<'a, T>) -> bool {
    match &node.node_type {
        NodeType::Static(p) => {
            if *p == &path_bytes[state.index..] {
                if node.value.is_some() {
                    state.value = node.value;
                    trace!("Exit with static! {} {}", std::str::from_utf8(&path_bytes[state.index..]).unwrap(), std::str::from_utf8(&*p).unwrap());
                    return true;
                }
                state.inc(p.len());
                return false;
            } else {
                state.inc(p.len());
            }
        },
        NodeType::Regex(regex) => {
            let res = regex.captures(&path[state.index..]).unwrap();
            let res = res.get(0).unwrap();
            let res = res.as_str();

            trace!("Matched {} {}", res, res.len());

            let len = res.len();

            state.inc_with_value(len, res);
            if state.index == path.len() {
                if node.value.is_some() {
                    state.value = node.value;
                    trace!("Exit with regex");
                    return true;
                }
                return false;
            }
        },
    }

    trace!("before child index: {}", state.index);

    let child = node.static_children.iter().find(|sc| {
        match &sc.node_type {
            NodeType::Static(sp) => {
                for i in 0..sp.len() {
                    if sp[i] != path_bytes[state.index + i] {
                        return false;
                    }
                }

                true
            },
            _ => unimplemented!(),
        }
    });

    if child.is_some() {
        trace!("Child static found");
        let r = find_inner(child.unwrap(), path, path_bytes, state);
        if r {
            return true;
        }
        trace!("Child static poped!");
        state.pop();
    }

    trace!("before regex index: {}", state.index);

    let child = node.regex_children.iter().find(|sc| {
        match &sc.node_type {
            NodeType::Regex(regex) => {
                trace!("checking... {}", &path[state.index..]);
                regex.is_match(&path[state.index..])
            },
            _ => unimplemented!(),
        }
    });

    if child.is_some() {
        trace!("Child regex found");
        let r = find_inner(child.unwrap(), path, path_bytes, state);
        if r {
            return true;
        }
        trace!("Child regex poped!");
        state.pop();
    }

    trace!("No found in the branch!");

    false
}

pub fn find<'a, T: PartialEq> (node: &Node<T>, path: &'a str) -> FindResult<'a, T> {
    let mut find_state = FindState {
        index: 0,
        steps: [0; MAX_NEASTING_LEVEL_COUNT],
        step_number: 0,
        value: None,
        params: [""; MAX_NEASTING_LEVEL_COUNT],
        param_number: 0
    };
    find_inner(node, path, path.as_bytes(), &mut find_state);

    let mut params = find_state.params[0..find_state.param_number].to_vec();
    params.retain(|x| !x.is_empty());
    FindResult {
        value: find_state.value,
        params,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use regex::Regex;

    #[test]
    fn get_root() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: Some(&0),
            static_children: Vec::new(),
            regex_children: Vec::new(),
        };

        let output = find(&root, "/");
        assert_eq!(FindResult { value: Some(&0), params: vec![] }, output);

        let output = find(&root, "b");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "/b");
        assert_eq!(FindResult { value: None, params: vec![] }, output);
    }

    #[test]
    fn get_static_child() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'a']),
                    value: Some(&1),
                    static_children: vec![],
                    regex_children: vec![],
                },
                Node {
                    node_type: NodeType::Static(vec![b'b']),
                    value: Some(&2),
                    static_children: vec![],
                    regex_children: vec![],
                },
                Node {
                    node_type: NodeType::Static(vec![b'c']),
                    value: Some(&3),
                    static_children: vec![],
                    regex_children: vec![],
                },
            ],
            regex_children: Vec::new(),
        };

        let output = find(&root, "/a");
        assert_eq!(FindResult { value: Some(&1), params: vec![] }, output);

        let output = find(&root, "/b");
        assert_eq!(FindResult { value: Some(&2), params: vec![] }, output);

        let output = find(&root, "/c");
        assert_eq!(FindResult { value: Some(&3), params: vec![] }, output);

        let output = find(&root, "/aa");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "/z");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "/");
        assert_eq!(FindResult { value: None, params: vec![] }, output);
    }

    #[test]
    fn get_regex_child() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: Vec::new(),
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new("^(\\d+)").unwrap()),
                    value: Some(&1),
                    static_children: vec![],
                    regex_children: vec![],
                },
            ],
        };

        let output = find(&root, "/1");
        assert_eq!(FindResult { value: Some(&1), params: vec!["1"] }, output);

        let output = find(&root, "/b");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "/aa");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "b");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "/");
        assert_eq!(FindResult { value: None, params: vec![] }, output);
    }

    #[test]
    fn get_static_fallback() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'1']),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'a']),
                            value: Some(&11),
                            static_children: vec![],
                            regex_children: vec![],
                        },
                    ],
                    regex_children: vec![],
                },
            ],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                    value: Some(&1),
                    static_children: vec![],
                    regex_children: vec![],
                },
            ],
        };

        let output = find(&root, "/1a");
        assert_eq!(FindResult { value: Some(&11), params: vec![] }, output);

        let output = find(&root, "/1");
        assert_eq!(FindResult { value: Some(&1), params: vec!["1"] }, output);

        let output = find(&root, "/11");
        assert_eq!(FindResult { value: Some(&1), params: vec!["11"] }, output);

        let output = find(&root, "/");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "/z");
        assert_eq!(FindResult { value: None, params: vec![] }, output);
    }

    #[test]
    fn get_regex_fallback() {
        let root = Node {
            node_type: NodeType::Static(vec![b'/']),
            value: None,
            static_children: vec![
                Node {
                    node_type: NodeType::Static(vec![b'1']),
                    value: None,
                    static_children: vec![],
                    regex_children: vec![
                        Node {
                            node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                            value: None, // unuseful node!
                            static_children: vec![],
                            regex_children: vec![],
                        },
                    ],
                },
            ],
            regex_children: vec![
                Node {
                    node_type: NodeType::Regex(Regex::new(r"^(\d)").unwrap()),
                    value: None,
                    static_children: vec![
                        Node {
                            node_type: NodeType::Static(vec![b'1']),
                            value: Some(&1),
                            static_children: vec![],
                            regex_children: vec![],
                        },
                    ],
                    regex_children: vec![],
                },
            ],
        };

        let output = find(&root, "/11");
        assert_eq!(FindResult { value: Some(&1), params: vec!["1"] }, output);

        let output = find(&root, "/1a");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "/1");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "/");
        assert_eq!(FindResult { value: None, params: vec![] }, output);

        let output = find(&root, "/z");
        assert_eq!(FindResult { value: None, params: vec![] }, output);
    }
}
