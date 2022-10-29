use criterion::{criterion_group, criterion_main, Criterion};
use nukedoctor::config::ConfigBuilder;
use nukedoctor::parser::parse;
use nukedoctor::utils::{filter_nodes, filter_nodes_2, read_file_to_string};

pub fn bench_filter_nodes(c: &mut Criterion) {
    let scene = read_file_to_string("test_data/bench_data.nk").unwrap();
    let source: Vec<char> = scene.chars().collect();
    let nodes = parse(source);
    let config = ConfigBuilder::build("test_data/bench_data.nk".to_string()).finish();

    c.bench_function("clean_up_scene() bench", |b| {
        b.iter(|| filter_nodes(nodes.clone(), &config))
    });
}

pub fn bench_filter_nodes_2(c: &mut Criterion) {
    let scene = read_file_to_string("test_data/bench_data.nk").unwrap();
    let source: Vec<char> = scene.chars().collect();
    let nodes = parse(source);
    let config = ConfigBuilder::build("test_data/bench_data.nk".to_string()).finish();

    c.bench_function("clean_up_scene() bench", |b| {
        b.iter(|| filter_nodes_2(nodes.clone(), &config))
    });
}

criterion_group!(benches, bench_filter_nodes, bench_filter_nodes_2);
criterion_main!(benches);
