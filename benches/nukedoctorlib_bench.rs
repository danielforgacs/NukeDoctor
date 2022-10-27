use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nukedoctor::utils::clean_up_scene;
use nukedoctor::config::Config;

pub fn bench_clean_up_scene(c: &mut Criterion) {
    c.bench_function("clean_up_scene() bench", |b| b.iter(|| clean_up_scene(
        "/home/ford/storage/dev/NukeDoctor/test_data/Cloudtastic.nk".to_string(),
        Config::new("/home/ford/storage/dev/NukeDoctor/test_data/Cloudtastic.nk".to_string())
    )));
}

criterion_group!(benches, bench_clean_up_scene);
criterion_main!(benches);
