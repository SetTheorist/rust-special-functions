#![allow(confusable_idents)]
#![allow(dead_code)]
#![allow(mixed_script_confusables)]
#![allow(non_snake_case)]
#![allow(unused_parens)]

extern crate num;
extern crate num_traits;

mod embed;
mod exp;
mod kahan;
mod numbers;
mod quad;
mod traits;
mod trig;
mod util;
mod value;

use std::time::{Instant};

use crate::num::complex::{Complex};
use crate::exp::{exp,exp_m1,ln,ln_1p,exp__powser,exp__powser2,exp__powserk};
use crate::util::{power_i};
use crate::numbers::{factorial};

fn rel(ex:f64, ap:f64) -> f64 {
  ((ex-ap).abs()/ex.abs()).ln()/10.0_f64.ln()
}

fn main() {
  // quad
  if true {
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

  if true {
    println!("-----");
    println!("{}", exp(0.25));
    println!("{}", (0.25_f64).exp());
    println!("{}", exp(1.00));
    println!("{}", (1.00_f64).exp());
    println!("{}", exp(5.00));
    println!("{}", (5.00_f64).exp());
    println!("-----");
    println!("{:e}", exp(Complex::new(0.00,3.1415926535897932384626/2.0)));
    println!("{}", exp(Complex::new(0.25,0.25)));
    println!("{}", (Complex::new(0.25,0.25)).exp());
    println!("{}", exp(Complex::new(-2.5,2.5)));
    println!("{}", (Complex::new(-2.5,2.5)).exp());
    println!("{}", exp(Complex::new(-22.5,12.5)));
    println!("{}", (Complex::new(-22.5,12.5)).exp());
    println!("-----");
    println!("{}", exp(1.0/256.0)-1.0);
    println!("{}", exp_m1(1.0/256.0));
    println!("{}", (1.0/256.0_f64).exp_m1());
    println!("-----");
    println!("{}", ln(1.0 + 1.0/16.0));
    println!("{}", ln_1p(1.0/16.0));
    println!("{}", (1.0_f64 + 1.0/16.0).ln());
    println!("-----");
    for n in -5..5 {
      println!("  {}", power_i(3.0,n));
    }
  }

  if true {
    println!("-----");
    let x = 2.0_f64;
    println!("{:e}", f64::EPSILON);
    println!("{:.16e}", x.exp());
    println!("{:.16e}  {}", exp(x), rel(x.exp(),exp(x)));
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
    for n in 0..1000000 { t += exp((x + ((n%13) as f64)/26.0)); }
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

  if true {
    for i in 0..10 {
      println!("{} {}", 3*i, factorial(3*i));
    }
  }
}
