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

pub fn main() {
  if true {
    println!("-----");
    let x = 1.0/3.0_f64;
    let y = f128::f128::from(x);
    let z = f64::from(y);
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    println!("{}", -y);
    println!("{}", f64::from(-y));
    println!("{}", y+y);
    println!("{}", f64::from(y+y));
    println!("{}", y+y+y);
    println!("{}", f64::from(y+y+y));
    println!("{}", (y+y)-y);
    println!("{}", f64::from((y+y)-y));
    println!("{}", y-(y+y));
    println!("{}", f64::from(y-(y+y)));

    println!("-----");
    let x = 3.0_f64;
    let x2 = 1.0/3.0_f64;
    let y = f128::f128::from(x);
    let y2 = f128::f128::from(x2);
    println!("{:e}  {}", x, y);
    println!("{:e}  {}", x2, y2);
    let z = y * y2;
    println!("{}  {:e}", z, f64::from(z));
    let mut z = y2;
    let mut yy = y2;
    for _ in 0..40 {
      z = z * yy;
      yy = yy * y2;
      println!("    {}  {:e}", z, f64::from(z));
    }
    println!("{:e}  {}", 1.5, f128::f128::from(1.5));

    let y1 = f128::f128::from(10.0_f64.recip());
    println!("{}", y1);
    let y2 = f128::f128::from(10.0_f64).recip();
    println!("{}", y2);

    let t = f128::f128::from(3.0);
    println!("{}", t);
    println!("{}", y1*t);
    println!("{}", y2*t);
    println!("{}", (t+t+t)/t);
    println!("{}", t/f128::f128::from(7.0));

    println!("---- ----");
    let q = (3.0_f64).sqrt();
    let j = f128::f128::from(q);
    println!("{}", j);
    println!("{}", t.sqrt());
    println!("{}", j*j);
    println!("{}", t.sqrt()*t.sqrt());
    println!("{}", t.sqrt_recip());

    println!("---- ----");
    let a = (2.0_f64).sqrt();
    println!("{}", a);
    let x = f128::f128::from(a);
    let y = f128::f128::from(2.0_f64).sqrt();
    println!("{}", x);
    println!("{}", f64::from(x));
	
    println!("{}", y);
    println!("{}", f64::from(y));
    println!("{}", y*y);
    println!("{}", y*y-f128::f128::from(2.0_f64));
  }
  if true {
    println!("-----");
    let y = f128::f128::from(2.0_f64).sqrt();
    println!("{}", f64::from(y));
    println!("{}", y);
    println!("-----");
    println!("{}", f128::f128::from(2).cbrt());
    println!("{}", f128::f128::from(2).cbrt_recip());
    println!("{}", f128::f128::from(2).nth_root(7));
    println!("{}", f128::f128::from(128_u128));
    println!("{}", f128::f128::from(1_u128<<100));
    println!("- - -");
    println!("  {}", f128::f128::from(2).cbrt().ldexp(-3));
    println!("  {}", f128::f128::from(2).cbrt().ldexp(-2));
    println!("  {}", f128::f128::from(2).cbrt().ldexp(-1));
    println!("* {}", f128::f128::from(2).cbrt().ldexp(0));
    println!("  {}", f128::f128::from(2).cbrt().ldexp(1));
    println!("  {}", f128::f128::from(2).cbrt().ldexp(2));
    println!("  {}", f128::f128::from(2).cbrt().ldexp(3));
    let x = f128::f128::from(3.75_f64);
    println!("{} {}", x.frexp().0, x.frexp().1);
    println!("log(1)={}", f128::f128::from(1).log());
    println!("log(2)={}", f128::f128::from(2).log());
    println!("log(3)={}", f128::f128::from(3).log());
    println!("log(100)={}", f128::f128::from(100).log());
    println!("exp(1)={}", f128::f128::from(1).exp());
    println!("exp(2)={}", f128::f128::from(2).exp());
    println!("exp(-1)={}", f128::f128::from(-1).exp());
    println!("log2(3)={}", f128::f128::from(3).log2());
    println!("log10(3)={}", f128::f128::from(3).log10());

    println!("- - -");
    println!("{}", f128::f128::from(80).recip());
    println!("{}", f128::f128::from(80).recip().exp());
    println!("{}", f128::f128::from(80).recip().exp_m1());
    println!("{}", f128::f128::from(80).recip().exp()-f128::f128::from(1));
    println!("{:032x}", f128::f128::from(80).recip().exp_m1().to_bits());

    println!("-----");
    println!("[0.25] = {}", f128::f128::from(0.25_f64).round());
    println!("[0.50] = {}", f128::f128::from(0.50_f64).round());
    println!("[0.75] = {}", f128::f128::from(0.75_f64).round());
    println!("[1.00] = {}", f128::f128::from(1.00_f64).round());
    println!("[1.25] = {}", f128::f128::from(1.25_f64).round());
    println!("[1.50] = {}", f128::f128::from(1.50_f64).round());
    println!("[1.75] = {}", f128::f128::from(1.75_f64).round());

    println!("-----");
    println!("2^(0.25) = {}", f128::f128::from(0.25_f64).exp2());
    println!("2^(0.5) = {}", f128::f128::from(0.50_f64).exp2());
    println!("2^(0.75) = {}", f128::f128::from(0.75_f64).exp2());
    println!("2^(1.0) = {}", f128::f128::from(1).exp2());
    println!("2^(2.0) = {}", f128::f128::from(2).exp2());
    println!("2^(11.25) = {}", f128::f128::from(11.25_f64).exp2());
  }
}