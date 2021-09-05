use num::bigint::{BigInt,ToBigInt};

pub fn factorial(n:isize) -> BigInt {
  let mut x: BigInt = num::One::one();
  for i in 1..n {
    x *= i.to_bigint().unwrap();
  }
  x
}

pub fn bell_number(n:isize) -> BigInt { unimplemented!() }
pub fn euler_number(n:isize) -> BigInt { unimplemented!() }
pub fn fibonacci_number(n:isize) -> BigInt { unimplemented!() }
pub fn genocchi_number(n:isize) -> BigInt { unimplemented!() }
pub fn binomial(n:isize,k:isize) -> BigInt { unimplemented!() }

//pub fn bernoulli_number(n:isize) -> BigRational { unimplemented!() }
//pub fn harmonic_number(n:isize) -> BigRational { unimplemented!() }
//pub fn tangent_number(n:isize) -> BigRational { unimplemented!() }
