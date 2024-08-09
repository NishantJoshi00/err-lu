use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use err_cat::prelude::{get_connector_list, get_connector_mapping};
use err_cat::wasm::lookup;

fn criterion_benchmark(c: &mut Criterion) {
    let keys = get_connector_list();

    let errors = keys
        .iter()
        .flat_map(|k| get_connector_mapping(k))
        .flat_map(|x| x.keys().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    c.bench_function(&format!("lookup-{}-{}", keys.len(), errors.len()), |b| {
        b.iter(|| {
            for i in keys.iter() {
                for j in &errors {
                    black_box(lookup(i, j));
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
