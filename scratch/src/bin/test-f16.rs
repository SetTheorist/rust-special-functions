#![allow(clippy::comparison_chain)]
#![allow(clippy::eq_op)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::float_cmp)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]
#![allow(clippy::wrong_self_convention)]
#![allow(confusable_idents)]
#![allow(dead_code)]
#![allow(mixed_script_confusables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![feature(bench_black_box)]
#![feature(bigint_helper_methods)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(int_log)]
#![feature(trait_alias)]
#![feature(type_ascription)]
#![feature(unchecked_math)]
//#![feature(marker_trait_attr)] // #[marker]
//#![feature(never_type)]
//#![feature(optimize_attribute)] // [#optimize(speed)]
//#![feature(specialization)]

macro_rules! time {
  ($val:expr) => {
    let beg = std::time::Instant::now();
    match $val {
      tmp => {
        let end = std::time::Instant::now();
        let time = (end - beg);
        println!(
          "[{}:{}] `{}' took {:?}",
          std::file!(),
          std::line!(),
          std::stringify!($val),
          time
          );
        tmp
      }
    }
  };
  ($($val:expr),+ $(,)?) => {
    ($(time!($val)),+,)
  };
}

fn rel(ex: f64, ap: f64) -> f64 {
  let ε = f64::EPSILON;
  let l10 = 10.0_f64.ln();
  let res = if ex == ap {
    ε.ln()/l10 - 1.0
  } else {
    // TODO
    //((ap - ex).abs() / (ε*ε + ex.abs())).ln() / l10
    ((ap - ex).abs() / ex.abs()).ln() / l10
  };
  if !res.is_finite() {0.0} else {res}
}

fn main() {
  ::simple_logger::SimpleLogger::new().init().unwrap();

  if true {
    let a = f16::f16(16_u16<<10);
    println!("{:016b}  {} {}", a.0, a.to_f32(), a.to_f64());
    println!("{:016b}  {} {}", (-a).0, (-a).to_f32(), (-a).to_f64());
    println!("{:016b}", f16::f16::from_f32(a.to_f32()).0);
    let b = a*a;
    println!("{:016b}  {} {}", b.0, b.to_f32(), b.to_f64());
    let c = b*b;
    println!("{:016b}  {} {}", c.0, c.to_f32(), c.to_f64());
    let a = f16::f16(0x3333);
    println!("{:016b}  {} {}", a.0, a.to_f32(), a.to_f64());
    let b = a*a;
    println!("{:016b}  {} {} {}", b.0, b.to_f32(), b.to_f64(), a.to_f32()*a.to_f32());
    let c = b*b;
    println!("{:016b}  {} {} {}={:016b}", c.0, c.to_f32(), c.to_f64(),
      b.to_f32()*b.to_f32(), f16::f16::from_f32(b.to_f32()*b.to_f32()).0);
  }

  if true {
    let x = f16::f16(0x0000);
    println!("{:?}", x);
    let x = f16::f16(0x5555);
    println!("{:?}", x);
    let x = f16::f16::from_f32(1.5);
    println!("{:?}", x);
    let x = f16::f16::from_f32(2.0);
    println!("{:?}", x);
    println!("{:?}", x*x);
    let x = f16::f16(0x4010);
    println!("{:?}", x);
    println!("{:?}", x*x);
    println!("{:?}", x*x*x);
    println!("{:?}", x*x*x*x);
    println!("{:?}", x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x*x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x*x*x*x*x*x*x*x*x);
    println!("{:?}", x*x*x*x*x*x*x*x*x*x*x*x*x*x*x*x*x);
    let x = f16::f16(0x4010);
    println!("{:?}", x);
    println!("{:?}", x+x);
    println!("{:?}", x+x+x);
    println!("{:?}", x+x+x+x);
    println!("{:?}", x+x+x+x+x);
    println!("{:?}", x+x+x+x+x+x);
    println!("{:?}", x+x+x+x+x+x+x);
    println!("{:?}", x+x+x+x+x+x+x+x);
    println!("{:?}", x+x+x+x+x+x+x+x+x);
    println!("{:?}", x+x+x+x+x+x+x+x+x+x);
    println!("{:?}", x+x+x+x+x+x+x+x+x+x+x);
    let x = f16::f16(0x4010);
    println!("{:?}", x+x*x*x+x+x);
    let x = f16::f16::from_f32(-13.125);
    println!("{:?}", x);
    let x = f16::f16::from_f32(3.14);
    println!("{:?}", x);
    println!("{:?}", x.prev());
    println!("{:?}", x.next());
    let x = f16::f16::from_f32(0.0);
    println!("{:?}", x);
    let x = f16::f16::from_f32(f32::INFINITY);
    println!("{:?}", x);
    let x = f16::f16::from_f32(f32::NAN);
    println!("{:?}", x);
    let x = f16::f16(0x7C01);
    println!("{:?}", x);
    let x = f16::f16::from_f32(0.5);
    let mut y = x;
    for _ in 0 .. 20 {
      y = y * x;
      println!("{:?}", y);
    }
  }

  if true {
    let mut x = f16::f16::from_f32(3.0);
    println!("{:?}", x);
    for _ in 0..20 {
      x = x * f16::f16::from_f32(0.5);
      println!("  {:?}", x);
    }
  }
}
