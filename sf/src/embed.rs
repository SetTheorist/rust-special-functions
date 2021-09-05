use num::complex::{Complex};

// T embedded into Self
pub trait Embed<T> {
  fn embed(t:T) -> Self;
}
impl<T> Embed<T> for T {
  #[inline]
  fn embed(t:T) -> Self { t }
}

#[inline]
pub fn embed<A,B:Embed<A>>(a:A) -> B { B::embed(a) }
#[inline]
pub fn ι<A,B:Embed<A>>(a:A) -> B { B::embed(a) }

impl Embed<isize> for f64 {
  #[inline]
  fn embed(t:isize) -> Self { t as f64 }
}
impl Embed<isize> for Complex<f64> {
  #[inline]
  fn embed(t:isize) -> Self { Complex::new(ι(t),0.0) }
}
impl Embed<f64> for Complex<f64> {
  #[inline]
  fn embed(t:f64) -> Self { Complex::new(t,0.0) }
}

//impl<T:Default+Num> Embed<T> for Complex<T> { fn embed(t:T) -> Self { Complex::new(t,T::default) } }

