#[macro_use]
extern crate bencher;
extern crate route_recognizer as recognizer;

use bencher::Bencher;

use recognizer::Router;

use bravery_router::{NodeType, Node, find};
use regex::Regex;

fn recognizer(bench: &mut Bencher) {
    let mut router = Router::new();

    router.add("/posts/:post_id/comments/:id", "comment".to_string());
    router.add("/posts/:post_id/comments", "comments".to_string());

    bench.iter(|| {
        router.recognize("/posts/12/comments").unwrap();
    })
}

fn router(bench: &mut Bencher) {
    let comments = &"comments";
    let root = Node {
        node_type: NodeType::Static(b"/posts/".to_vec()),
        value: None,
        static_children: vec![],
        regex_children: vec![
            Node {
                node_type: NodeType::Regex(Regex::new(r"^(\d+)").unwrap()),
                value: None,
                static_children: vec![
                    Node {
                        node_type: NodeType::Static(b"/comments".to_vec()),
                        value: Some(comments),
                        static_children: vec![],
                        regex_children: vec![],
                    },
                ],
                regex_children: vec![],
            },
        ],
    };

    bench.iter(|| {
        find(&root, "/posts/12/comments").value.unwrap();
    })
}

benchmark_group!(benches, recognizer, router);
benchmark_main!(benches);
