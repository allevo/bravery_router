#[macro_use]
extern crate bencher;
extern crate route_recognizer as recognizer;

use bencher::Bencher;

use recognizer::Router;

use bravery_router::{add, create_root_node, find, optimize};

fn recognizer(bench: &mut Bencher) {
    let mut router = Router::new();

    router.add("/posts/*", "comment".to_string());

    bench.iter(|| {
        router.recognize("/posts/12/comments").unwrap();
    })
}

fn router(bench: &mut Bencher) {
    let comments = "comments";
    let root = create_root_node();
    let root = add(root, "/posts/*", comments);

    let optimized = optimize(root);

    bench.iter(|| {
        find(&optimized, "/posts/12/comments").value.unwrap();
    })
}

benchmark_group!(benches, recognizer, router);
benchmark_main!(benches);
