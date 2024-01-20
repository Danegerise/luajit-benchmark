use criterion::{criterion_group, criterion_main, Criterion};
use luajit::exec;

fn local(c: &mut Criterion) {
    c.bench_function("local", |b| {
        b.iter(|| {
            exec!("../scripts/local.lua");
        });
    });
}

fn table(c: &mut Criterion) {
    c.bench_function("table", |b| {
        b.iter(|| {
            exec!("../scripts/table.lua");
        })
    });
}

criterion_group!(benches, local, table);

criterion_main!(benches);
