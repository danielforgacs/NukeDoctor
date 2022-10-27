use criterion::{criterion_group, criterion_main, Criterion};
use nukedoctor::utils::{clean_up_scene, read_file_to_string, filter_nodes};
use nukedoctor::parser::parse;
use nukedoctor::config::ConfigBuilder;
use nukedoctor::config::Config;

pub fn bench_clean_up_scene(c: &mut Criterion) {
    let scene = read_file_to_string("test_data/bench_data.nk").unwrap();
    let source: Vec<char> = scene.chars().collect();
    let nodes = parse(source);
    let config = ConfigBuilder::build("test_data/bench_data.nk".to_string()).finish();

    c.bench_function("clean_up_scene() bench", |b| b.iter(||
        filter_nodes(nodes.clone(), &config)
    ));
}

criterion_group!(benches, bench_clean_up_scene);
criterion_main!(benches);
