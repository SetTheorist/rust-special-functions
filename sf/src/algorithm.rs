use crate::embed::{ι};
use crate::traits::*;

////////////////////////////////////////////////////////////////////////////////

pub fn power_u<T:Multiplication>(mut x:T, mut n:usize) -> T {
  let mut v = T::one;
  while n != 0 {
    if n%2 == 1 { v *= x; }
    x = x.sqr();
    n >>= 1;
  }
  v
}

pub fn power_i<T:Multiplication+Division>(x:T, n:isize) -> T {
  if n<0 {
    power_u(x,-n as usize).recip()
  } else {
    power_u(x,n as usize).recip()
  }
}

////////////////////////////////////////////////////////////////////////////////

// TODO: "wrapped" version (generic over Kahan, e.g.)
#[inline]
pub fn sum_series<T,I>(it:I,eps:f64) -> T
  where
    T:Field+Normed,
    I:IntoIterator<Item=T>
{
  let mut sum = ι(0); // = T::zero;
  let mut n = 1;
  for t in it {
    let old = sum;
    sum += t;
    if fabs(sum - old) <= fabs(sum)*eps || n>1000 { /*eprint!("^{}^",n);*/break; }
    n += 1;
  }
  sum
}

// given the sequence (ai,bi) evaluates the continued fraction
// b0 + a1/(b1 + a2/(b2 + a3/(b3 + ...)))
// (based on modified Lentz)
#[inline]
pub fn contfrac_modlentz<T,I>(b0:T, it:I, eps:f64) -> T
  where
    T:Field+Normed,
    I:IntoIterator<Item=(T,T)>
{
  let zeta = ι(eps*eps);
  let mut fj = b0; if b0==ι(0) {fj=zeta;}
  let mut cj = fj;
  let mut dj = ι(0);
  let mut n = 1;
  for (aj,bj) in it {
    dj = bj + aj*dj; if dj==ι(0) {dj=zeta;}
    cj = bj + aj/cj; if cj==ι(0) {cj=zeta;}
    dj = dj.recip(); // 1/dj
    let deltaj = cj * dj;
    fj *= deltaj;
    if fabs(deltaj - 1) < eps || n>1000 { /*print!("~{}~",n);*/break; }
    n += 1;
  }
  fj
}

