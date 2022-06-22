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

pub fn main() {
  if true {
    println!("-----");
    let mut m = farb::I([0x1234,0x5679]);
    println!("{:?}", m);
    let o = farb::normalize(&mut m, true, 0);
    println!("{:?}  {}", m, o);
    println!("-----");
    let mut m = farb::I([0x1234,0x5679]);
    println!("{:?}", m);
    let o = farb::normalize(&mut m, false, 0);
    println!("{:?}  {}", m, o);
    println!("-----");
    let mut m = farb::I([0x80001234,0x80005679]);
    println!("{:?}", m);
    let o = farb::normalize(&mut m, false, 0x8000_0001);
    println!("{:?}  {}", m, o);
    println!("-----");
    let mut m = farb::I([0xFFFFFFFF,0xFFFFFFFF]);
    println!("{:?}", m);
    let o = farb::normalize(&mut m, false, 0x8000_0000);
    println!("{:?}  {}", m, o);

    println!("-----  -----");
    let x = farb::farb::<2>::from(3.5_f64);
    let y = farb::farb::<2>::from(0.25_f64);
    println!("x = {:?} ({:e})", x, f64::from(x));
    println!("y = {:?} ({:e})", y, f64::from(y));
    println!("x+x = {:?} ({:e})", x+x, f64::from(x+x));
    println!("x+y = {:?} ({:e})", x+y, f64::from(x+y));
    println!("x+y+y = {:?} ({:e})", x+y+y, f64::from(x+y+y));
    println!("y+x = {:?} ({:e})", y+x, f64::from(y+x));
    println!("y+y = {:?} ({:e})", y+y, f64::from(y+y));
    println!("(x+y)-y = {:?} ({:e})", (x+y)-y, f64::from((x+y)-y));
    println!("(x+y)-x = {:?} ({:e})", (x+y)-x, f64::from((x+y)-x));

    println!("-----");
    let mut m = farb::I([0x12345678, 0x98765432, 0x11111111]);
    let mut o = 0;
    println!("++ {:?}  {:08x}", m, o);
    farb::shl(&mut m, &mut o, 12);
    println!("++ {:?}  {:08x}", m, o);
    farb::shl(&mut m, &mut o, 1);
    println!("++ {:?}  {:08x}", m, o);

    println!("-----");
    let mut m = farb::I([0x12345678, 0x98765432, 0x11111111]);
    let mut o = 0xffffffff;
    println!("++ {:?}  {:08x}", m, o);
    farb::shl(&mut m, &mut o, 4);
    println!("++ {:?}  {:08x}", m, o);
    farb::shl(&mut m, &mut o, 36);
    println!("++ {:?}  {:08x}", m, o);

    println!("-----");
    let mut m = farb::I([0x12345688, 0x98765432, 0x11111111]);
    println!("++ {:?}", m);
    let o = farb::shr(&mut m, 4);
    println!("++ {:?}  {:08x}", m, o);
    let o = farb::shr(&mut m, 32);
    println!("++ {:?}  {:08x}", m, o);
    let o = farb::shr(&mut m, 128);
    println!("++ {:?}  {:08x}", m, o);

    println!("-----");
    let mut m = farb::I([0x44444444, 0x88888888, 0x11111111]);
    println!("++ {:?}", m);
    let o = farb::shr(&mut m, 4);
    println!("++ {:?}  {:08x}", m, o);
    let o = farb::shr(&mut m, 36);
    println!("++ {:?}  {:08x}", m, o);

    println!("-----  -----");
    let x = farb::farb::<2>::from(3.5_f64);
    println!("{:?}  {:e}", x*0, f64::from(x*0));
    println!("{:?}  {:e}", x*1, f64::from(x*1));
    println!("{:?}  {:e}", x*2, f64::from(x*2));
    println!("{:?}  {:e}", x*3, f64::from(x*3));
    println!("{:?}  {:e}", x*10, f64::from(x*10));
    println!("{:?}  {:e}", x*100, f64::from(x*100));
    let x = farb::farb::<2>::from(0.1_f64);
    println!("{:?}  {:e}", x*0, f64::from(x*0));
    println!("{:?}  {:e}", x*1, f64::from(x*1));
    println!("{:?}  {:e}", x*2, f64::from(x*2));
    println!("{:?}  {:e}", x*3, f64::from(x*3));
    println!("{:?}  {:e}", x*10, f64::from(x*10));

    let mut res = farb::I([0;4]);
    let x = farb::I([1,2]);
    let y = farb::I([3,3]);
    farb::mul_mm(&mut res, &x, &y);
    println!("{:?}", res);
    let mut res = farb::I([0;2]);
    let x = farb::I([!0]);
    let y = farb::I([!0]);
    farb::mul_mm(&mut res, &x, &y);
    println!("{:?}", res);
    let mut res = farb::I([0;4]);
    let x = farb::I([!0,!0]);
    let y = farb::I([!0,!0]);
    farb::mul_mm(&mut res, &x, &y);
    println!("{:?}", res);

    println!("-----  -----");
    let x = farb::farb::<2>::from(2.0_f64);
    println!("{:?}  {:e}", x*x, f64::from(x*x));
    let x = farb::farb::<2>::from(3.0_f64);
    println!("{:?}  {:e}", x*x, f64::from(x*x));
    let x = farb::farb::<2>::from(7.0_f64);
    println!("{:?}  {:e}", x*x, f64::from(x*x));
    let x = farb::farb::<3>::from(3.25_f64);
    let y = farb::farb::<3>::from(7.5_f64);
    println!("{:?}  {:e}", x*y, f64::from(x*y));
    let x = farb::farb::<3>::from(0.1_f64);
    let y = farb::farb::<3>::from(10.0_f64);
    let o = farb::farb::<3>::from(1.0_f64);
    println!("{:?}  {:e}", x*y-o, f64::from(x*y-o));

    let x = farb::farb::<7>::from(10.0_f64);
    let xi = x.recip();
    println!("{:?}  {:e}", xi, f64::from(xi));

    let x = farb::farb::<8>::from(10.0_f64);
    let xi = x.recip();
    println!("{:?}  {:e}", xi, f64::from(xi));

    let x = farb::farb::<15>::from(10.0_f64);
    let xi = x.recip();
    println!("{:?}  {:e}", xi, f64::from(xi));

    let x = farb::farb::<3>::from(0.1_f64);
    println!("{}", x);
    println!("{}", x*10);
    let x = farb::farb::<3>::from(1.0/3.0_f64);
    println!("{}", x);
    println!("{}", x*3);
    let x = farb::farb::<4>::from(3.0_f64).recip();
    println!("{}", x);
    println!("{}", x*3);
    let x = farb::farb::<10>::from(2.0_f64).sqrt();
    println!("{}", x);
    let x = farb::farb::<50>::from(2.0_f64).sqrt();
    println!("{}", x);
    println!("{:?}", x);
    let x = farb::farb::<100>::from(2.0_f64).sqrt();
    println!("{}", x);
    println!("{:?}", x);
  }
}