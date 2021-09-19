use crate::traits::*;

////////////////////////////////////////////////////////////////////////////////

pub fn power_u<T: Multiplication>(mut x: T, mut n: usize) -> T {
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

pub fn power_i<T: Multiplication + Division>(x: T, n: isize) -> T {
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
pub fn cum_prods<T, I>(it: I) -> impl Iterator<Item = T>
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
pub fn cum_prods_1<T, I>(it: I) -> impl Iterator<Item = T>
where
  T: Multiplicative,
  I: Iterator<Item = T>,
{
  std::iter::once(T::one).chain(cum_prods(it))
}

#[inline]
pub fn cum_sums<T, I>(it: I) -> impl Iterator<Item = T>
where
  T: Additive,
  I: Iterator<Item = T>,
{
  it.scan(T::zero, |s, a| {
    *s += a;
    Some(*s)
  })
}

#[inline]
pub fn cum_sums_0<T, I>(it: I) -> impl Iterator<Item = T>
where
  T: Additive,
  I: Iterator<Item = T>,
{
  std::iter::once(T::zero).chain(cum_sums(it))
}

////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn sum_series_<T, I>(it: I, meps: T::NT) -> T
where
  T: Field + Normed,
  I: Iterator<Item = T>,
{
  //cum_sums(it)
  it.scan(T::zero, |s, a| {
    *s += a;
    Some(*s)
  })
  .scan(ι(0.0 / 0.0): T, |s, t| {
    if μ(*s - t) <= μ(*s) * meps {
      None
    } else {
      *s = t;
      Some(t)
    }
  })
  .take(1000)
  .last()
  .unwrap()
}

// TODO: "wrapped" version (generic over Kahan, e.g.)
#[inline]
pub fn sum_series<T, I>(it: I, meps: T::NT) -> T
where
  T: Field + Normed,
  I: Iterator<Item = T>,
{
  let mut sum = ι(0); // = T::zero;
  let mut n = 1;
  for t in it {
    let old = sum;
    sum += t;
    if μ(sum - old) <= μ(sum) * meps {
      /*eprint!("^{}^",n);*/
      break;
    }
    if n > 999 {
      break;
    }
    n += 1;
  }
  sum
}

// given the sequence (ai,bi) evaluates the continued fraction
// b0 + a1/(b1 + a2/(b2 + a3/(b3 + ...)))
// (based on modified Lentz)
#[inline]
pub fn contfrac_modlentz<T, I>(b0: T, it: I, meps: T::NT) -> T
where
  T: Field + Normed,
  I: IntoIterator<Item = (T, T)>,
{
  let zeta = ι(T::epsilon.sqr());
  let mut fj = b0;
  if b0 == 0 {
    fj = zeta;
  }
  let mut cj = fj;
  let mut dj: T = ι(0);
  let mut n = 1;
  for (aj, bj) in it {
    dj = bj + aj * dj;
    if dj == 0 {
      dj = zeta;
    }
    cj = bj + aj / cj;
    if cj == 0 {
      cj = zeta;
    }
    dj = dj.recip();
    let deltaj = cj * dj;
    fj *= deltaj;
    if μ(deltaj - 1) < meps || n > 1000 {
      /*print!("~{}~",n);*/
      break;
    }
    n += 1;
  }
  fj
}
