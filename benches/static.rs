#[macro_use]
extern crate bencher;
extern crate route_recognizer as recognizer;

use bencher::Bencher;

use recognizer::Router;

use router::{NodeType, Node, find};

fn recognizer(bench: &mut Bencher) {
    let mut router = Router::new();

    router.add("/thomas", "Thomas".to_string());
    router.add("/tom", "Tom".to_string());
    router.add("/wycats", "Yehuda".to_string());

    bench.iter(|| {
        router.recognize("/thomas").unwrap();
    })
}

fn router(bench: &mut Bencher) {
    let thomas = &"Thomas";
    let tom = &"Tom";
    let yehuda = &"Yehuda";
    let root = Node {
        node_type: NodeType::Static(b"/"),
        value: None,
        static_children: vec![
            Node {
                node_type: NodeType::Static(b"t"),
                value: None,
                static_children: vec![
                    Node {
                        node_type: NodeType::Static(b"homas"),
                        value: Some(thomas),
                        static_children: vec![],
                        regex_children: vec![],
                    },
                    Node {
                        node_type: NodeType::Static(b"om"),
                        value: Some(tom),
                        static_children: vec![],
                        regex_children: vec![],
                    },
                ],
                regex_children: vec![],
            },
            Node {
                node_type: NodeType::Static(b"wycats"),
                value: Some(yehuda),
                static_children: vec![],
                regex_children: vec![],
            },
        ],
        regex_children: Vec::new(),
    };

    bench.iter(|| {
        find(&root, "/thomas").value.unwrap();
    })
}

benchmark_group!(benches, recognizer, router);
benchmark_main!(benches);
