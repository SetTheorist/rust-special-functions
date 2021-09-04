#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(confusable_idents)]
#![allow(mixed_script_confusables)]


extern crate num;
extern crate num_traits;

mod exp;
mod quad;
mod trig;
mod util;
mod value;

use num::complex::{Complex};
use exp::{exp,exp_m1};

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
    //println!("{}", exp::sf_ln(1.0 + 1.0/16.0));
    //println!("{}", exp::sf_ln_p1(1.0/16.0));
  }
}
