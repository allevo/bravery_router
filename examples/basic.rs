use bravery_router::{add, optimize, find, create_root_node};

fn main() {
    let root = create_root_node();

    let root = add(root, "/foo", 1);
    let root = add(root, "/foobar", 2);
    let root = add(root, "/users/:id", 3);
    let root = add(root, "/all/*", 4);

    let root = optimize(root);

    let empty_vec: Vec<&str> = vec![];

    let ret = find(&root, "/foo");
    assert_eq!(ret.value, Some(&1));
    assert_eq!(ret.params, empty_vec);
    println!("/foo, {:?}, {:?}", ret.value, ret.params);

    let ret = find(&root, "/foobar");
    assert_eq!(ret.value, Some(&2));
    assert_eq!(ret.params, empty_vec);
    println!("/foobar, {:?}, {:?}", ret.value, ret.params);

    let ret = find(&root, "/users/42");
    assert_eq!(ret.value, Some(&3));
    assert_eq!(ret.params, vec!["42"]);
    println!("/users/42, {:?}, {:?}", ret.value, ret.params);

    let ret = find(&root, "/all/foo/bar");
    assert_eq!(ret.value, Some(&4));
    assert_eq!(ret.params, vec!["foo/bar"]);
    println!("/all/foo/bar, {:?}, {:?}", ret.value, ret.params);

    let ret = find(&root, "/unknwon");
    assert_eq!(ret.value, None);
    assert_eq!(ret.params, empty_vec);
    println!("/unknwon, {:?}, {:?}", ret.value, ret.params);

    println!("Done!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main()
    }
}
