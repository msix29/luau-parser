use criterion::{black_box, criterion_group, criterion_main, Criterion};
use luau_parser::prelude::LuauParser;

fn parse(parser: &mut LuauParser, code: &str) {
    parser.parse(code, "");
}

fn benchmark(c: &mut Criterion) {
    let mut parser = LuauParser::new();
    let code = include_str!("big_code.lua");
    c.bench_function("env.lua (big_code.lua)", |b| {
        b.iter(|| parse(black_box(&mut parser), black_box(code)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
