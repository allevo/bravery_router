#[macro_use]
extern crate bencher;
extern crate route_recognizer as recognizer;

use bencher::Bencher;

use recognizer::Router;

use bravery_router::{add, create_root_node, find, optimize};

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
    add(&mut root, "/thomas", "Thomas");
    add(&mut root, "/tom", "Tom");
    add(&mut root, "/wycats", "Yehuda");

    let optimized = optimize(root);

    bench.iter(|| {
        find(&optimized, "/thomas").value.unwrap();
    })
}

fn router_plus_vec(bench: &mut Bencher) {
    let mut root = create_root_node();
    add(&mut root, "/thomas", 0);
    add(&mut root, "/tom", 1);
    add(&mut root, "/wycats", 2);

    let mut handler = Vec::new();
    handler.push("Thomas");
    handler.push("Tom");
    handler.push("Yehuda");

    let optimized = optimize(root);

    bench.iter(|| {
        let index = find(&optimized, "/thomas").value.unwrap();
        handler.get(*index).unwrap();
    })
}

benchmark_group!(benches, recognizer, router, router_plus_vec);
benchmark_main!(benches);
