mod prelude;

use std::hint::black_box;

use prelude::*;

use bc_orders_collectors::clear::CLEAR;

static COLLECTOR: LazyLock<CLEAR> = LazyLock::new(|| CLEAR);

fn clear_1(c: &mut Criterion) {
    let trade_cell = TradeCell::new(100., vec![1.], vec![2.]);
    c.bench_function("clear_1", |b| {
        b.iter(|| COLLECTOR.collect_orders(black_box(&trade_cell)))
    });
}

criterion_group!(benches, clear_1);
criterion_main!(benches);
