use crate::traits::{ι};

pub fn sf_factorial_approx(n:isize) -> f64 {
  //if n==0 { return 1.0; }
  (1..=n).map(|i|i as f64).product()
}

use num::bigint::{BigInt};
pub fn sf_factorial_exact(n:isize) -> BigInt {
  if n==0 { return ι(1); }
  (1..=n).map(|i|ι(i):BigInt).product()
}


/*
use num::bigint::{BigInt};
use num::rational::{BigRational};
use num::{Signed};
use std::ops::{Mul,MulAssign};
use crate::embed::{ι};

use std::collections::HashMap;
//use std::vec::Vec;
use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;


////////////////////////////////////////////////////////////////////////////////

trait NumbersIntegral {
  fn factorial(n:isize) -> Self;
  fn euler(n:isize) -> Self;

  fn binomial(n:isize, k:isize) -> Self;
  fn stirling_first(n:isize, k:isize) -> Self;
  fn stirling_second(n:isize, k:isize) -> Self;

  fn tangent(n:isize) -> Self;
  fn derangement(n:isize) -> Self;
  fn catalan(n:isize) -> Self;
  fn bell(n:isize) -> Self;

  fn fibonacci(n:isize) -> Self;
  fn genocchi(n:isize) -> Self;
}

trait NumbersRational {
  fn bernoulli(n:isize) -> Self;
  fn harmonic(n:isize) -> Self;
}

trait NumbersGeneral {
}

////////////////////////////////////////////////////////////////////////////////
/*
impl NumbersRational for BigRational {
  fn bernoulli(n:isize) -> Self { sf_bernoulli_number_exact(n) }
  fn harmonic(n:isize) -> Self { sf_harmonic_number_exact(n) }
}
impl NumbersRational for f64 {
  fn bernoulli(n:isize) -> Self { 
  fn harmonic(n:isize) -> Self { unimplemented!() }
}
*/

////////////////////////////////////////////////////////////////////////////////

/*
impl Embed<isize> for BigInt {
  fn embed(i:isize) -> Self {
    i.to_bigint().unwrap()
  }
}
impl Embed<isize> for BigRational {
  fn embed(i:isize) -> Self {
    BigRational::from_integer(BigInt::embed(i))
  }
}
*/



#[derive(Clone)]
struct Fib(BigInt,BigInt);
impl Mul<Fib> for Fib {
  type Output = Self;
  #[inline]
  fn mul(self, Fib(c,d):Fib) -> Self {
    let Fib(a,b) = self;
    Fib(&a*&c+5*&b*&d,a*d+b*c)
  }
}
impl MulAssign<Fib> for Fib {
  #[inline]
  fn mul_assign(&mut self, t:Fib) {
    *self = self.clone() * t;
  }
}
fn powi(mut x:Fib,mut n:isize) -> Fib {
  let mut v = Fib(num::One::one(),num::Zero::zero());
  while n != 0 {
    if n%2 == 1 { v *= x.clone(); }
    x *= x.clone();
    n >>= 1;
  }
  v
}

pub fn sf_fibonacci_number_exact(n:isize) -> BigInt { 
  let x = powi(Fib(num::One::one(),num::One::one()), n);
  (x.1*2) >> (n as usize)
}

pub fn sf_catalan_number_exact(n:isize) -> BigInt {
  sf_binomial_exact(2*n, n) / ι(n+1):BigInt
}

pub fn sf_bell_number_exact(_n:isize) -> BigInt { unimplemented!() }
pub fn sf_euler_number_exact(_n:isize) -> BigInt { unimplemented!() }
pub fn sf_binomial_exact(n:isize, k:isize) -> BigInt {
  //sf_factorial_exact(n) / sf_factorial_exact(k) / sf_factorial_exact(n-k)
  let k = k.min(n-k);
  let mut val = num::One::one();
  for i in 0..k {
    val *= ι(n-i):BigInt;
    val /= ι(i+1):BigInt;
  }
  val
}
pub fn sf_binomial_approx(n:isize, k:isize) -> f64 {
  //sf_factorial_exact(n) / sf_factorial_exact(k) / sf_factorial_exact(n-k)
  let k = k.min(n-k);
  let mut val = 1.0;
  for i in 0..k {
    val *= (n-i) as f64;
    val /= (i+1) as f64
  }
  val
}
// \sum_{k=0}^{n}\binom{n+1}{k}B_k = 0 \]
// not the best implementation at the moment, but quick placeholder for now
static BERNOULLI_EXACT_CACHE: Lazy<Mutex<HashMap<isize,BigRational>>> = Lazy::new(||Mutex::new(HashMap::new()));
pub fn sf_bernoulli_number_exact(n:isize) -> BigRational {
  if n == 0 { ι(1) }
  else if n == 1 { BigRational::new(ι(-1),ι(2)) }
  else if n%2 == 1 { ι(0) }
  else {
    {
      let cache = BERNOULLI_EXACT_CACHE.lock().unwrap();
      if let Some(x) = cache.get(&n) {
        return x.clone();
      }
    }
    let mut sum = BigRational::from(0);
    for k in 0..n {
      sum += sf_bernoulli_number_exact(k) * BigRational::from_integer(sf_binomial_exact(n+1,k));
    }
    let val = -sum * BigRational::new(ι(1),ι(n+1));
    BERNOULLI_EXACT_CACHE.lock().unwrap().insert(n,val.clone());
    val
  }
}
static BERNOULLI_FLOAT_CACHE: Lazy<Mutex<HashMap<isize,f64>>> = Lazy::new(||Mutex::new(HashMap::new()));
pub fn sf_bernoulli_number_approx(n:isize) -> f64 {
  if n == 0 { 1.0 }
  else if n == 1 { -0.5 }
  else if n%2 == 1 { 0.0 }
  else {
    {
      let cache = BERNOULLI_FLOAT_CACHE.lock().unwrap();
      if let Some(x) = cache.get(&n) {
        return *x;
      }
    }
    let mut sum = 0.0;
    for k in 0..n {
      sum += sf_bernoulli_number_approx(k) * sf_binomial_approx(n+1,k);
    }
    let val = -sum / ((n+1) as f64);
    BERNOULLI_FLOAT_CACHE.lock().unwrap().insert(n,val);
    val
  }
}

pub fn sf_harmonic_number_exact(n:isize) -> BigRational {
  (1..=n).map(|i|BigRational::new(ι(1),ι(i))).sum()
}
pub fn sf_tangent_number_exact(n:isize) -> BigInt {
  if n==0 { return num::Zero::zero(); }
  let bn = sf_bernoulli_number_exact(2*n).abs();
  let t0 : BigInt = (ι(1):BigInt) << (2*n as usize);
  let t1 : BigInt = t0.clone() - (ι(1):BigInt);
  //(bn * BigRational::new(t0 * t1, ι(2*n))).numer().clone()
  (bn * BigRational::new(t0 * t1, ι(2*n))).to_integer()
}

pub fn sf_genocchi_number_exact(n:isize) -> BigInt {
  if n==0 { return num::Zero::zero(); }
  let bn = sf_bernoulli_number_exact(2*n);
  let t1 : BigInt = (ι(2):BigInt)*((ι(1):BigInt) - ((ι(1):BigInt) << (2*n as usize)));
  //(bn * BigRational::new(t0 * t1, ι(2*n))).numer().clone()
  (bn * BigRational::new(t1, ι(1))).to_integer()
}

*/
