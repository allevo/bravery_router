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


pub fn create_root_node<'node, T: PartialEq> () -> Node<T> {
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

    impl<'node> std::fmt::Debug for Node<u8> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Node {{\n  t: {:?}\n  v: {:?}\n  s: {:?}\n  r: {:?}\n  w: {:?}\n  }}", self.node_type, self.value, self.static_children, self.regex_children, self.wildcard_children)
        }
    }

    impl<'node> std::fmt::Debug for Node<String> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Node {{\n  t: {:?}\n  v: {:?}\n  s: {:?}\n  r: {:?}\n  w: {:?}\n  }}", self.node_type, self.value, self.static_children, self.regex_children, self.wildcard_children)
        }
    }

    #[test]
    fn all() {
        let root = create_root_node();

        let root = add(root, "/foo", 1);
        let root = add(root, "/bar", 2);
        let root = add(root, "/foobar", 3);
        let root = add(root, "/:name/b", 4);
        let root = add(root, "/aa/bb/cc/dd/*", 5);

        let optimized = optimize(root);

        let result: FindResult<u8> = find(&optimized, "/foo");
        assert_eq!(&1, result.value.unwrap());

        let result: FindResult<u8> = find(&optimized, "/bar");
        assert_eq!(&2, result.value.unwrap());

        let result: FindResult<u8> = find(&optimized, "/foobar");
        assert_eq!(&3, result.value.unwrap());

        let result: FindResult<u8> = find(&optimized, "/tom/b");
        assert_eq!(&4, result.value.unwrap());

        let result: FindResult<u8> = find(&optimized, "/aa/bb/cc/dd/ee");
        assert_eq!(&5, result.value.unwrap());

        let result: FindResult<u8> = find(&optimized, "/not-found");
        assert_eq!(true, result.value.is_none());
    }

    #[test]
    fn bench_regex() {
        let root = create_root_node();
        let root = add(root, "/posts/:post_id/comments/:id", "comments".to_owned());
        let root = add(root, "/posts/:post_id/comments", "comments".to_owned());

        println!("{:?}", root);

        let optimized = optimize(root);

        println!("{:?}", optimized);

        find(&optimized, "/posts/12/comments").value.unwrap();
    }

    pub trait Handler<T: Clone> {
        fn invoke(&self) -> Result<u8, u8>;
    }

    #[derive(Clone)]
    struct HandlerFor404 {}

    impl<'node> std::fmt::Debug for Node<HandlerFor404> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Node {{ HandlerFor404 }}")
        }
    }

    impl PartialEq<HandlerFor404> for HandlerFor404 {
        fn eq(&self, _other: &HandlerFor404) -> bool {
            true
        }
    }

    #[test]
    fn with_struct() {
        let handler = HandlerFor404 { };
        let root = create_root_node();
        let root = add(root, "/posts/:post_id/comments/:id", handler);

        let optimized = optimize(root);

        assert_eq!(true, find(&optimized, "/posts/12/comments").value.is_none());
        assert_eq!(true, find(&optimized, "/posts/12/comments/foo").value.is_some());
    }
}