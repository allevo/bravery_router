#[macro_use]
extern crate bencher;
extern crate route_recognizer as recognizer;

use bencher::Bencher;

use recognizer::Router;

use bravery_router::{add, optimize, create_root_node, find};

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
    let mut root = create_root_node();
    add(&mut root, "/posts/:post_id/comments/:id", comments);
    add(&mut root, "/posts/:post_id/comments", comments);

    let optimized = optimize(root);

    bench.iter(|| {
        find(&optimized, "/posts/12/comments").value.unwrap();
    })
}

benchmark_group!(benches, recognizer, router);
benchmark_main!(benches);
