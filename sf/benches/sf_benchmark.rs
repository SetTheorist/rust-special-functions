use criterion::{BenchmarkId, black_box, criterion_group, criterion_main, Criterion};

use sf::airy::*;
use sf::exp::*;

fn benchmark(c: &mut Criterion) {
  {
    let mut group = c.benchmark_group("Airy");
    for x in [0.1, 1.0, 5.0, 10.0, 50.0].iter() {
      group.bench_with_input(BenchmarkId::new("AI", x), x, |b,&x| b.iter(|| sf_airy_ai(x)));
      group.bench_with_input(BenchmarkId::new("BI", x), x, |b,&x| b.iter(|| sf_airy_bi(x)));
      group.bench_with_input(BenchmarkId::new("AIBI", x), x, |b,&x| b.iter(|| sf_airy_aibi(x)));
    }
  }

  {
    let mut group = c.benchmark_group("exp");
    for x in [0.1, 0.5, 1.0, 5.0, 10.0].iter() {
      group.bench_with_input(BenchmarkId::new("exp", x), x, |b,&x|b.iter(||sf_exp(x)));
      group.bench_with_input(BenchmarkId::new("exp_m1", x), x, |b,&x|b.iter(||sf_exp_m1(x)));
      group.bench_with_input(BenchmarkId::new("exp_m1vx", x), x, |b,&x|b.iter(||sf_exp_m1vx(x)));
      //group.bench_with_input(BenchmarkId::new("expn(5)", x), x, |b,&x|b.iter(||sf_expn(5, x)));
      //group.bench_with_input(BenchmarkId::new("exp_men(5)", x), x, |b,&x|b.iter(||sf_exp_men(5, x)));
      //group.bench_with_input(BenchmarkId::new("exp_menx(5)", x), x, |b,&x|b.iter(||sf_exp_menx(5, x)));
    }
  }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
