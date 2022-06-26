use criterion::{BenchmarkId, black_box, criterion_group, criterion_main, Criterion};

use sf::airy::*;
use sf::erf::*;
use sf::exp::*;
use sf::gamma::*;

fn benchmark(c: &mut Criterion) {
  {
    let mut group = c.benchmark_group("Airy");
    for x in [0.1, 1.0, 5.0, 10.0, 50.0].iter() {
      group.bench_with_input(BenchmarkId::new("AI", x), x, |b,&x| b.iter(|| sf_airy_ai(black_box(x))));
      group.bench_with_input(BenchmarkId::new("BI", x), x, |b,&x| b.iter(|| sf_airy_bi(black_box(x))));
      group.bench_with_input(BenchmarkId::new("AIBI", x), x, |b,&x| b.iter(|| sf_airy_aibi(black_box(x))));
    }
  }

  {
    let mut group = c.benchmark_group("Erf");
    for x in [0.1, 0.5, 1.0, 5.0, 10.0].iter() {
      group.bench_with_input(BenchmarkId::new("erf", x), x, |b,&x|b.iter(||sf_erf(black_box(x))));
      group.bench_with_input(BenchmarkId::new("erfc", x), x, |b,&x|b.iter(||sf_erfc(black_box(x))));
    }
  }

  {
    let mut group = c.benchmark_group("ErfInv");
    for x in [0.0, 0.1, 0.5, 0.9, 0.99].iter() {
      group.bench_with_input(BenchmarkId::new("erf_inv", x), x, |b,&x|b.iter(||sf_erf_inv(black_box(x))));
    }
  }

  {
    let mut group = c.benchmark_group("Exp");
    for x in [0.1, 0.5, 1.0, 5.0, 10.0].iter() {
      group.bench_with_input(BenchmarkId::new("exp", x), x, |b,&x|b.iter(||sf_exp(black_box(x))));
      group.bench_with_input(BenchmarkId::new("exp_m1", x), x, |b,&x|b.iter(||sf_exp_m1(black_box(x))));
      group.bench_with_input(BenchmarkId::new("exp_m1vx", x), x, |b,&x|b.iter(||sf_exp_m1vx(black_box(x))));
      //group.bench_with_input(BenchmarkId::new("expn(5)", x), x, |b,&x|b.iter(||sf_expn(5, black_box(x))));
      group.bench_with_input(BenchmarkId::new("exp_men(5)", x), x, |b,&x|b.iter(||sf_exp_men(5, black_box(x))));
      //group.bench_with_input(BenchmarkId::new("exp_menx(5)", x), x, |b,&x|b.iter(||sf_exp_menx(5, black_box(x))));
    }
  }

  {
    let mut group = c.benchmark_group("Gamma");
    for x in [0.1, 1.0, 5.0, 10.0, 20.0].iter() {
      group.bench_with_input(BenchmarkId::new("gamma", x), x, |b,&x| b.iter(|| sf_gamma(black_box(x))));
      group.bench_with_input(BenchmarkId::new("lngamma", x), x, |b,&x| b.iter(|| sf_lngamma(black_box(x))));
      group.bench_with_input(BenchmarkId::new("digamma", x), x, |b,&x| b.iter(|| sf_digamma(black_box(x))));
      group.bench_with_input(BenchmarkId::new("beta(5)", x), x, |b,&x| b.iter(|| sf_beta(black_box(x), 5.0)));
    }
  }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
