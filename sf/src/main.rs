#![feature(type_ascription)]
#![allow(confusable_idents)]
#![allow(dead_code)]
#![allow(mixed_script_confusables)]
#![allow(non_snake_case)]
#![allow(unused_parens)]

extern crate num;
extern crate num_traits;
extern crate once_cell;

mod dawson;
mod embed;
mod erf;
mod exp;
mod gamma;
mod kahan;
mod numbers;
mod quad;
mod traits;
mod trig;
mod util;
mod value;

use std::time::{Instant};

use crate::num::complex::{Complex};
use crate::exp::{sf_exp,sf_exp_m1,sf_ln,sf_ln_1p,exp__powser,exp__powser2,exp__powserk};
use crate::util::{power_i};
use crate::numbers::{*};
use crate::dawson::{*};
use crate::erf::{*};
use crate::embed::{*};
use crate::gamma::{*};

use crate::kahan::{*};

mod real;
use crate::real::{*};

fn rel(ex:f64, ap:f64) -> f64 {
  ((ex-ap).abs()/ex.abs()).ln()/10.0_f64.ln()
}

pub fn emb<A,B:From<A>>(a:A) -> B { B::from(a) }

fn main() {
  // quad
  if false {
    let q_pi = quad::stoq("3.14159265358979323846264338327950288419716939937510");
    println!("{:?}", q_pi);
    println!("{:?}", quad::qtos(q_pi));
    let q_eulergamma = quad::stoq("0.57721566490153286060651209008240243104215933593992");
    println!("{:?}", q_eulergamma);
    let q_ln2 = quad::stoq("0.69314718055994530941723212145817656807");
    println!("{:?}", q_ln2);
    println!("-----");
    let x = quad::Quad::new(1.0,0.0); 
    let y = quad::Quad::new(0.0,0.1); 
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", quad::Quad::new(1.0,0.1));
    println!("{:?}", (x+y)*(x+y));
    println!("{:?}", (x*y)+(x*y));
    println!("{:?}", (x+y)*10.0);
    println!("{:?}", quad::Quad::new(1.0,0.0)/10.0);
    println!("{:?}", (quad::Quad::new(1.0,0.0)/10.0)*10.0);
    println!("{:?}", quad::Quad::new(1.0,0.1).scale2(3));
    println!("-----");
    let mut z = quad::Quad::new(1.0,0.0);
    z /= 10.0;
    println!("{:?}", z);
    println!("{:?}", quad::qtos(z));
    println!("{:?}", quad::qtos(quad::Quad::new(0.1,0.0)));
  }

  if false {
    println!("-----");
    println!("{}", sf_exp(0.25));
    println!("{}", (0.25_f64).exp());
    println!("{}", sf_exp(1.00));
    println!("{}", (1.00_f64).exp());
    println!("{}", sf_exp(5.00));
    println!("{}", (5.00_f64).exp());
    println!("-----");
    println!("{:e}", sf_exp(Complex::new(0.00,3.1415926535897932384626/2.0)));
    println!("{}", sf_exp(Complex::new(0.25,0.25)));
    println!("{}", (Complex::new(0.25,0.25)).exp());
    println!("{}", sf_exp(Complex::new(-2.5,2.5)));
    println!("{}", (Complex::new(-2.5,2.5)).exp());
    println!("{}", sf_exp(Complex::new(-22.5,12.5)));
    println!("{}", (Complex::new(-22.5,12.5)).exp());
    println!("-----");
    println!("{}", sf_exp(1.0/256.0)-1.0);
    println!("{}", sf_exp_m1(1.0/256.0));
    println!("{}", (1.0/256.0_f64).exp_m1());
    println!("-----");
    println!("{}", sf_ln(1.0 + 1.0/16.0));
    println!("{}", sf_ln_1p(1.0/16.0));
    println!("{}", (1.0_f64 + 1.0/16.0).ln());
    println!("-----");
    for n in 0..5 {
      println!("  {}", power_i(3.0,n));
    }
  }

  if true {
    println!("-----");
    let x = 2.0_f64;
    println!("{:e}", f64::EPSILON);
    println!("{:.16e}", x.exp());
    println!("{:.16e}  {}", sf_exp(x), rel(x.exp(),sf_exp(x)));
    println!("{:.16e}  {}", exp__powser(x,1.0), rel(x.exp(),exp__powser(x,1.0)));
    println!("{:.16e}  {}", exp__powser2(x,1.0), rel(x.exp(),exp__powser2(x,1.0)));
    println!("{:.16e}  {}", exp__powserk(x,1.0), rel(x.exp(),exp__powserk(x,1.0)));

    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += (x + ((n%13) as f64)/26.0).exp(); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += sf_exp((x + ((n%13) as f64)/26.0)); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += exp__powser((x + ((n%13) as f64)/26.0),1.0); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += exp__powser2((x + ((n%13) as f64)/26.0),1.0); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += exp__powserk((x + ((n%13) as f64)/26.0),1.0); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
  }
  if false {
    for i in 0..=10 {
      println!("{} {}", 3*i, sf_factorial_exact(3*i));
    }
    for i in 0..=10 {
      println!("{} {}", 10*i, sf_fibonacci_number_exact(10*i));
    }
    println!("{} {}", 1000, sf_fibonacci_number_exact(1000));

    for n in 0..=5 {
      print!("{} : ", n);
      for k in 0..=n { print!("{}  ", sf_binomial_exact(n,k)); }
      println!();
    }

    println!("=====");
    for i in 0..=30 { print!("{}  ", sf_bernoulli_number_exact(i)); }
    println!();
    println!("=====");
    for i in 0..=10 { print!("{}  ", sf_harmonic_number_exact(i)); }
    println!();
    for i in 0..=10 { print!("{}  ", sf_tangent_number_exact(i)); }
    println!();
    for i in 0..=10 { print!("{}  ", sf_genocchi_number_exact(i)); }
    println!();
  }

  if true {
    println!("=====");
    for i in 0..=10 { print!("{}  ", sf_bernoulli_number_exact(i)); }
    println!();
    for i in 0..=10 { print!("{}  ", sf_bernoulli_number_approx(i)); }
    println!();
  }

  if false {
    for x in &vec![-9.0, -1.0, 0.0, 1.0, 5.0, 13.0] {
      let x = *x;
      println!("{}  {:.16e}  {:.16e}  {:.16e}  {:.16e}",
        x, dawson_contfrac(x), dawson_contfrac2(x), dawson_seres(x), dawson_rybicki(x));
    }
    for z in &vec![Complex::new(0.0,1.0), Complex::new(1.0,1.0), Complex::new(5.0,5.0)] {
      let z = *z;
      println!("{}  {:.16e}  {:.16e}  {:.16e}  {:.16e}",
        z, dawson_contfrac(z), dawson_contfrac2(z), dawson_seres(z), dawson_rybicki(z));
    }
  }

  if false {
    for x in &vec![-2.0, -1.0, 0.0, 0.5, 1.0, 3.0] {
      let x = *x;
      println!("{}  {:.16e}", x, erf_series(x));
    }
    for z in &vec![Complex::new(0.0,1.0), Complex::new(1.0,1.0), Complex::new(5.0,5.0)] {
      let z = *z;
      println!("{}  {:.16e}", z, erf_series(z));
    }
  }

  if true {
    let terms = (1..10).scan(1.0_f64,|s,n|{*s*=2.0/(ι(n):f64);Some(*s)});
    for t in terms { print!("  {}", t); } println!();
    println!("sum:  {:.16e}", 
      ((1..10).scan(1.0_f64,|s,n|{*s*=2.0/(ι(n):f64);Some(*s)})).sum():f64);
    println!("ksum: {:.16e}", 
      ((1..10).scan(1.0_f64,|s,n|{*s*=2.0/(ι(n):f64);Some(*s)})).ksum():f64);
    println!("sum:  {:.16e}", [1.0_f64,1e-12,-1.0,1e-22].iter().sum():f64);
    //println!("ksum: {:.16e}", [1.0_f64,1e-12,-1.0,1e-22].iter().ksum():f64);
    println!("ksum: {:.16e}", [1.0_f64,1e-12,-1.0,1e-22].iter().cloned().ksum():f64);
    let mut k = Kahan::default();
    for &x in &[1.0_f64,1e-12,-1.0,1e-22] {
      k += x;
    }
    println!("{:.16e}", k.0);
  }

  if true {
    //let x = 2 + 3*r(1.0) + 0.5 + 3 + (3.5 % 1.0);
    let x = 2 - 2*(r64(1.0) + 0.5)*3;
    println!("{:?}", x);
    println!("{:?}", eps(r64(1.0)));
    println!("{:?}", eps2(r64(1.0)));
    println!("{:?}", dss(r64(1.0)));
  }
  if true {
    println!("=====");
    println!("{}", sf_factorial_approx(4));
    println!("{} {:.16e} {}", 3.0, gamma_asympt(3.0), sf_factorial_exact(2));
    println!("{} {:.16e} {}", 13.0, gamma_asympt(13.0), sf_factorial_exact(12));
    println!("{} {:.16e} {}", 20.0, gamma_asympt(20.0), sf_factorial_exact(19));
    println!("{} {:.16e} {}", 21.0, gamma_asympt(21.0), sf_factorial_exact(20));
    println!("{} {:.16e} {}", 51.0, gamma_asympt(51.0), sf_factorial_exact(50));
    println!("-----");
    println!("{} {:.16e} {}", 3.0, gamma_spouge(11,3.0), sf_factorial_exact(2));
    println!("{} {:.16e} {}", 13.0, gamma_spouge(11,13.0), sf_factorial_exact(12));
    println!("{} {:.16e} {}", 40.0, gamma_spouge(11,40.0), sf_factorial_exact(39));
    println!("{} {:.16e} {}", 100.0, gamma_spouge(11,100.0), sf_factorial_exact(99));
    println!("{} {:.16e} {}", 40.0, gamma_spouge(11,40.0), sf_factorial_exact(39));
  }
  //println!("{:e}", {let x:f64 = 2.5_f64 + ι(3);x});
  if true {
    println!("{:?}", eps2(r64(1.0)));
    println!("{:?}", erf_series(1.0));
    println!("{:?}", erf_ss(r64(1.0)));
  }
  if true {
    println!("-----");
    let x = 30.0;
    println!("{:.16e} {:.16e}", (1.0_f64+x).ln(), ln_1p_cf(r64(x)).0);
    let x = 8.0;
    println!("{:.16e} {:.16e}", (1.0_f64+x).ln(), ln_1p_cf(r64(x)).0);
    let x = 0.5;
    println!("{:.16e} {:.16e}", (1.0_f64+x).ln(), ln_1p_cf(r64(x)).0);
    let x = 0.01;
    println!("{:.16e} {:.16e}", (1.0_f64+x).ln(), ln_1p_cf(r64(x)).0);
    let x = 1e-8;
    println!("{:.16e} {:.16e}", (1.0_f64+x).ln(), ln_1p_cf(r64(x)).0);
    println!("--");
    for &x in &[-1.0, 0.1, 1.0, 9.0_f64] {
      println!("{} {:.16e} {:.16e} {:.16e}",
        x, (x).exp(), exp_cf(r64(x)).0, exp_cf2(r64(x)).0);
    }
    println!("--");
    println!("{:.16e} {:.16e}",
      (1.0+5.0_f64.sqrt())*0.5,
      contfrac((0..).map(|_|(r64(1.0),r64(1.0))),1e-12).0);
  }
}
