use criterion::{black_box, criterion_group, criterion_main, Criterion};
use proton_pass_common::creditcard::CreditCardDetector;

fn card_detector(c: &mut Criterion) {
    let detector = CreditCardDetector::default();
    c.bench_function("detect unknown card", |b| b.iter(|| detector.detect(black_box("1"))));
    c.bench_function("detect visa card", |b| {
        b.iter(|| detector.detect(black_box("4111111111111111")))
    });
    c.bench_function("detect maestro card", |b| {
        b.iter(|| detector.detect(black_box("6703 4444 4444 4449")))
    });
}

criterion_group!(benches, card_detector);
criterion_main!(benches);
