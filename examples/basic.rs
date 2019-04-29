use bravery_router::{NodeType, Node, add, optimize, find};

fn main() {
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

    let root = optimize(root);

    assert_eq!(find(&root, "/foo").value, Some(&1));
    assert_eq!(find(&root, "/foobar").value, Some(&2));
    assert_eq!(find(&root, "/fo").value, Some(&3));
    assert_eq!(find(&root, "/bar").value, Some(&4));
    assert_eq!(find(&root, "/unknown").value, None);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main()
    }
}
