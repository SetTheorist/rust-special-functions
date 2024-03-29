use crate::traits::*;

////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn lerp<V:Value,T:Value+Ordered>(a:V, b:V, t:T) -> V
  where V:Embeds<T>
{
  if t<ι(0.5) {a+(b-a)*t} else {b-(b-a)*(-t+1.0)}
}

// TODO:
// splines: cubic, B-splines, general
// multi-point linear interpolation
// n-dim. interpolation

////////////////////////////////////////////////////////////////////////////////

// Newton's method:
// x' = x - f(x)/f'(x)
//
// Halley's method:
// x' = x - 2 f(x) f'(x) / (2 f'(x)^2 - f(x)f''(x))

use crate::real::{r64};
pub trait InitialGuess { fn guess(self, n:i32) -> Self; }
impl InitialGuess for r64 {
  #[inline]
  fn guess(self, n:i32) -> r64 {
    // TODO: this only works for finite, normal floats
    let b = self.0.to_bits();
    let e = (((b >> 52) & 0x7FFF) as i32) - 1023;
    let e = (e / n) + 1023;
    let b = (b & 0x8000_FFFF_FFFF_FFFF) | ((e as u64) << 52);
    r64(f64::from_bits(b))
  }
}
// NB: for mantissa part
// 0.45 + 0.62*x*(1.0 - 0.11x)
// reduces to 4 newton steps on [1,2)
// (from 6-7 from n/2 guess)
// simple: 0.5+x/2*(1.0-x/8)  gives 5 Newton steps on [1,2)

use crate::complex::{c64};
impl InitialGuess for c64 {
  #[inline]
  fn guess(self, n:i32) -> c64 {
    let (r, a) = self.to_polar();
    c64::polar(r64(r.0.powf(1.0/(n as f64))), (a/(n as isize)))
  }
}


// TODO: assume normed and get better stopping condition?
pub fn sqrt_newton<V:Value+InitialGuess>(x:V) -> V {
  let mut v = x.guess(2);
  for i in 0..6 {
    let o = v;
    v = (v + x/v)/2;
    if o == v { log::trace!("(x={},v={},i={})",x,v,i); break; }
  }
  v
}

// TODO: assume normed and get better stopping condition?
pub fn sqrt_halley<V:Value+InitialGuess>(x:V) -> V {
  let mut v = x.guess(2);
  for i in 0..11 {
    let o = v;
    v = (v + x*3/v) / (v*3 + x/v) * v;
    if o == v { log::trace!("(x={},v={},i={})",x,v,i); break; }
  }
  v
}

// TODO: assume normed and get better stopping condition?
pub fn cbrt_newton<V:Value+InitialGuess>(x:V) -> V {
  let mut v = x.guess(3);
  for i in 0..100 {
    let o = v;
    v = v * (v*v + x*2/v) / (v*v*2 + x/v);
    if o == v { log::trace!("(x={},v={},i={})",x,v,i); break; }
  }
  v
}

// TODO: assume normed and get better stopping condition?
pub fn nthrt_newton<V:Value+InitialGuess>(x:V, n:isize) -> V {
  let mut v = x.guess(n as i32);
  for i in 0..100 {
    let o = v;
    v = (v*(n-1) + x/v.pow(n-1))/n;
    if o == v { log::trace!("(x={},v={},n={},i={})",x,v,n,i); break; }
  }
  v
}
