#[macro_use]
extern crate bencher;
extern crate route_recognizer as recognizer;

use bencher::Bencher;

use recognizer::Router;

use bravery_router::{NodeType, Node, find};

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
        node_type: NodeType::Static(b"/".to_vec()),
        value: None,
        static_children: vec![
            Node {
                node_type: NodeType::Static(b"t".to_vec()),
                value: None,
                static_children: vec![
                    Node {
                        node_type: NodeType::Static(b"homas".to_vec()),
                        value: Some(thomas),
                        static_children: vec![],
                        regex_children: vec![],
                    },
                    Node {
                        node_type: NodeType::Static(b"om".to_vec()),
                        value: Some(tom),
                        static_children: vec![],
                        regex_children: vec![],
                    },
                ],
                regex_children: vec![],
            },
            Node {
                node_type: NodeType::Static(b"wycats".to_vec()),
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
