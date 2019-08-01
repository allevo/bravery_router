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

    add(&mut root, "/foo", &1);
    add(&mut root, "/foobar", &2);
    add(&mut root, "/fo", &3);
    add(&mut root, "/bar", &4);
    add(&mut root, "/users/:id", &5);
    add(&mut root, "/all/*", &6);

    let root = optimize(root);

    assert_eq!(find(&root, "/foo").value, Some(&1));
    assert_eq!(find(&root, "/foobar").value, Some(&2));
    assert_eq!(find(&root, "/fo").value, Some(&3));
    assert_eq!(find(&root, "/bar").value, Some(&4));
    assert_eq!(find(&root, "/users/42").value, Some(&5));
    assert_eq!(find(&root, "/all/foo/bar").value, Some(&6));
    assert_eq!(find(&root, "/unknown").value, None);
}
```

## License

See [License file](./LICENSE)
