#[macro_use]
extern crate bencher;
extern crate route_recognizer as recognizer;

use bencher::Bencher;

use recognizer::Router;

use bravery_router::{add, optimize, create_root_node, find};

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
    let mut root = create_root_node();
    add(&mut root,"/thomas", &"Thomas");
    add(&mut root,"/tom", &"Tom");
    add(&mut root,"/wycats", &"Yehuda");

    let optimized = optimize(root);

    bench.iter(|| {
        find(&optimized, "/thomas").value.unwrap();
    })
}

benchmark_group!(benches, recognizer, router);
benchmark_main!(benches);
