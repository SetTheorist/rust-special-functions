use criterion::{BenchmarkId, black_box, criterion_group, criterion_main, Criterion};

use sf::airy::*;
use sf::exp::*;

fn benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("Airy");
  for x in [0.1, 1.0, 5.0, 10.0, 50.0].iter() {
    group.bench_with_input(BenchmarkId::new("AI", x), |b,&x| b.iter(|| sf_airy_ai(x)));
    group.bench_with_input(BenchmarkId::new("BI", x), |b,&x| b.iter(|| sf_airy_bi(x)));
    group.bench_with_input(BenchmarkId::new("AIBI", x), |b,&x| b.iter(|| sf_airy_aibi(x)));
  }

  let mut group = c.benchmark_group("exp");
  for x in [0.1, 0.5, 1.0, 5.0, 10.0].iter() {
    group.bench_with_input(BenchmarkId::from_parameter(x), x, |b,&x|b.iter(||sf_exp(x)));
  }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
