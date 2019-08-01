extern crate regex;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate lazy_static;

mod node;
mod find;
mod optimize;
mod add;

use crate::node::{NodeType, Node};

pub use crate::find::{find, FindResult};
pub use crate::optimize::optimize;
pub use crate::add::add;


pub fn create_root_node<T: PartialEq> () -> Node<T> {
    Node {
        node_type: NodeType::Static(vec![b'/']),
        value: None,
        static_children: vec![],
        regex_children: vec![],
        wildcard_children: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl std::fmt::Debug for NodeType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                NodeType::Static(p) => write!(f, "NodeType::Static {:?}", p),
                NodeType::Regex(r) => write!(f, "NodeType::Regex {:?}", r),
                NodeType::Wildcard() => write!(f, "NodeType::Wildcard"),
            }
        }
    }

    impl std::fmt::Debug for Node<u8> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Node {{\n  t: {:?}\n  v: {:?}\n  s: {:?}\n  r: {:?}\n  w: {:?}\n  }}", self.node_type, self.value, self.static_children, self.regex_children, self.wildcard_children)
        }
    }

    impl std::fmt::Debug for Node<&str> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Node {{\n  t: {:?}\n  v: {:?}\n  s: {:?}\n  r: {:?}\n  w: {:?}\n  }}", self.node_type, self.value, self.static_children, self.regex_children, self.wildcard_children)
        }
    }

    #[test]
    fn all() {
        let mut root = create_root_node();

        add(&mut root, "/foo", &1);
        add(&mut root, "/bar", &2);
        add(&mut root, "/foobar", &3);

        add(&mut root, "/:name/b", &4);
        add(&mut root, "/aa/bb/cc/dd/*", &5);

        let optimized = optimize(root);

        let result: FindResult<u8> = find(&optimized, "/foo");
        assert_eq!(1 as u8, *result.value.unwrap());

        let result: FindResult<u8> = find(&optimized, "/bar");
        assert_eq!(2 as u8, *result.value.unwrap());

        let result: FindResult<u8> = find(&optimized, "/foobar");
        assert_eq!(3 as u8, *result.value.unwrap());

        let result: FindResult<u8> = find(&optimized, "/aa/bb/cc/dd/ee");
        assert_eq!(5 as u8, *result.value.unwrap());

        let result: FindResult<u8> = find(&optimized, "/not-found");
        assert_eq!(None, result.value);
    }

    #[test]
    fn bench_regex() {
        let comments = &"comments";
        let mut root = create_root_node();
        add(&mut root, "/posts/:post_id/comments/:id", comments);
        add(&mut root, "/posts/:post_id/comments", comments);

        println!("{:?}", root);

        let optimized = optimize(root);

        println!("{:?}", optimized);

        find(&optimized, "/posts/12/comments").value.unwrap();
    }
}