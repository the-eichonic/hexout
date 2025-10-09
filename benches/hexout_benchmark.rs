use criterion::{criterion_group, criterion_main, Criterion};
use hexout::HexOutSettings;

fn criterion_benchmark(c: &mut Criterion) {
    let size = 1024 * 1024;
    let mut data = Vec::with_capacity(size);
    let mut val: u8 = 3;
    for i in 0..size {
        data.push(val);
        val = val.wrapping_add(i as u8) ^ 0x5A;
    }
    let settings: HexOutSettings = HexOutSettings::default();
    c.bench_function("hex_out 1M bytes", |b| b.iter(|| hexout::hex_out(&data, &settings, 0, 0, 0)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
