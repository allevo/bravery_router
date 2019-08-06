# bravery_router
[![Build Status](https://travis-ci.org/allevo/bravery_router.svg?branch=master)](https://travis-ci.org/allevo/bravery_router)

Fast HTTP router

## Install

Add the following line into your `Cargo.toml`

```
bravery_router = "*"
```

## Usage

```rust
use bravery_router::{add, optimize, find, create_root_node};

fn main() {
    let mut root = create_root_node();

    add(&mut root, "/foo", 1);
    add(&mut root, "/foobar", 2);
    add(&mut root, "/users/:id", 3);
    add(&mut root, "/all/*", 4);

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
```

## Contributing

Every contribution is welcomed: Open an issue a fire a PR!

## License

See [License file](./LICENSE)
