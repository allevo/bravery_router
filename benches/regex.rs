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
    let mut root = create_root_node();
    add(&mut root, "/posts/:post_id/comments/:id", "comments1");
    add(&mut root, "/posts/:post_id/comments", "comments2");

    let optimized = optimize(root);

    bench.iter(|| {
        find(&optimized, "/posts/12/comments").value.unwrap();
    })
}

fn router_plus_vec(bench: &mut Bencher) {
    let mut root = create_root_node();
    add(&mut root, "/posts/:post_id/comments/:id", 0);
    add(&mut root, "/posts/:post_id/comments", 1);

    let mut handler = Vec::new();
    handler.push("comments1");
    handler.push("comments2");

    let optimized = optimize(root);

    bench.iter(|| {
        let index: &usize = find(&optimized, "/posts/12/comments").value.unwrap();
        handler.get(*index).unwrap();
    })
}

benchmark_group!(benches, recognizer, router, router_plus_vec);
benchmark_main!(benches);
