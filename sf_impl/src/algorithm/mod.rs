use crate::traits::*;

pub mod integration;

pub fn frexp1(x:f64) -> (f64, isize) {
  if x.is_zero() || x.is_infinite() || x.is_nan() {
    (x, 0)
  } else if x.is_subnormal() {
    // TODO: subnormals
    todo!()
  } else {
    let b = x.to_bits();
    let e = (((b>>52) & 0x7FF) as isize) - 1023;
    let m = (b & !(0x7FF<<52)) | (1023<<52);
    (f64::from_bits(m), e)
  }
}

////////////////////////////////////////////////////////////////////////////////

pub fn power_u<T:Multiplication>(mut x:T, mut n:usize) -> T {
  let mut v = T::one;
  while n != 0 {
    if n % 2 == 1 {
      v *= x;
    }
    x = x.sqr();
    n >>= 1;
  }
  v
}

pub fn power_i<T:Multiplication+Division>(x:T, n:isize) -> T {
  if n < 0 {
    power_u(x, -n as usize).recip()
  } else {
    power_u(x, n as usize)
  }
}

////////////////////////////////////////////////////////////////////////////////

/*
#[inline]
pub fn powers<T>() -> impl Iterator<Item=T>
  where
    T:Multiplicative
{
  it.scan(
}
*/

#[inline]
pub fn cum_prods<T, I>(it:I) -> impl Iterator<Item = T>
where
  T: Multiplicative,
  I: Iterator<Item = T>,
{
  it.scan(T::one, |s, a| {
    *s *= a;
    Some(*s)
  })
}

#[inline]
pub fn cum_prods_1<T, I>(it:I) -> impl Iterator<Item = T>
where
  T: Multiplicative,
  I: Iterator<Item = T>,
{
  std::iter::once(T::one).chain(cum_prods(it))
}

#[inline]
pub fn cum_sums<T, I>(it:I) -> impl Iterator<Item = T>
where
  T: Additive,
  I: Iterator<Item = T>,
{
  it.scan(T::zero, |s, a| { *s += a; Some(*s) })
}

#[inline]
pub fn cum_sums_0<T, I>(it:I) -> impl Iterator<Item = T>
where
  T: Additive,
  I: Iterator<Item = T>,
{
  std::iter::once(T::zero).chain(cum_sums(it))
}

////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn sum_series_<T, I>(it:I, ε:T::NT) -> T
where
  T: Field + Normed,
  I: Iterator<Item = T>,
{
  //cum_sums(it)
  it.scan(T::zero, |s, a| { *s += a; Some(*s) })
    .scan(ι(f64::NAN):T,
      |s, t| { if μ(*s - t) <= μ(*s) * ε { None } else { *s = t; Some(t) } })
    .take(1000)
    .last()
    .unwrap()
}

// TODO: "wrapped" version (generic over Kahan, e.g.)
#[inline]
pub fn sum_series<T, I>(it:I, ε:T::NT) -> T
where
  T: Field + Normed,
  I: Iterator<Item = T>,
{
  let mut sum = ι(0); // = T::zero;
  let mut n = 1;
  for t in it {
    let old = sum;
    sum += t;
    if μ(sum - old) <= μ(sum) * ε { break; }
    if n > 999 { break; }
    n += 1;
  }
  sum
}

// given the sequence (ai,bi) evaluates the continued fraction
// b0 + a1/(b1 + a2/(b2 + a3/(b3 + ...)))
// (based on modified Lentz)
#[inline]
pub fn contfrac_modlentz<T,I>(b0:T, it:I, ε:T::NT) -> T
where
  T: Field + Normed,
  I: IntoIterator<Item=(T,T)>,
{
  let ζ = ι(T::epsilon.sqr());
  let fix = |x:T| if x==0 {ζ} else {x};
  let mut fj = fix(b0);
  let mut cj = fj;
  let mut dj = T::zero;
  let mut n = 1;
  for (aj, bj) in it {
    dj = fix(bj + aj * dj);
    cj = fix(bj + aj / cj);
    dj = dj.recip();
    let deltaj = cj * dj;
    fj *= deltaj;
    if μ(deltaj - 1) < ε || n > 1000 { log::warn!("[{}]",n); break; }
    n += 1;
  }
  fj
}

#[inline]
// Steeds (forward) cont.frac. algorithm
pub fn contfrac_steeds<T,I>(b0:T, it:I, ε:T::NT)  -> T
where
  T: Field + Normed,
  I: IntoIterator<Item=(T,T)>,
{
  let mut it = it.into_iter();
  if let Some((a1,b1)) = it.next() {
    let mut dj = b1.recip();
    let mut d_cj = a1*dj;
    let mut cj = b0 + d_cj;
    let mut n = 1;
    for (aj,bj) in it {
      dj = (dj*aj + bj).recip();
      d_cj = (bj*dj - 1)*d_cj;
      let old_cj = cj;
      cj = cj + d_cj;
      if cj == old_cj || μ(d_cj)<ε || n >= 1000 { log::warn!("<{}>",n); break; }
      n += 1;
    }
    cj
  } else {
    b0
  }
}
