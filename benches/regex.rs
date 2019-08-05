#[macro_use]
extern crate bencher;
extern crate route_recognizer as recognizer;

use bencher::Bencher;

use recognizer::Router;

use bravery_router::{add, create_root_node, find, optimize};

fn recognizer(bench: &mut Bencher) {
    let mut router = Router::new();

    router.add("/posts/:post_id/comments/:id", "comment1".to_string());
    router.add("/posts/:post_id/comments", "comments2".to_string());

    bench.iter(|| {
        router.recognize("/posts/12/comments").unwrap();
    })
}

fn router(bench: &mut Bencher) {
    let root = create_root_node();
    let root = add(root, "/posts/:post_id/comments/:id", "comments1");
    let root = add(root, "/posts/:post_id/comments", "comments2");

    let optimized = optimize(root);

    bench.iter(|| {
        find(&optimized, "/posts/12/comments").value.unwrap();
    })
}

benchmark_group!(benches, recognizer, router);
benchmark_main!(benches);
