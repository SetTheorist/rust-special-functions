use std::ops::{Add,AddAssign,Sub,SubAssign};
use crate::traits::*;

#[derive(Clone,Copy,Debug,Default,PartialEq)]
pub struct Kahan<T>(pub T, pub T);

impl<T:Default> Kahan<T> {
  #[inline]
  pub fn new(t0:T) -> Self { Kahan(t0,T::default()) }
}

impl<T:Additive> Add<T> for Kahan<T> {
  type Output = Self;
  #[inline]
  fn add(self,t:T) -> Self {
    let y = t - self.1;
    let s = self.0 + y;
    let e = (s - self.0) - y;
    Kahan(s,e)
  }
}

impl<T:Additive> AddAssign<T> for Kahan<T> {
  #[inline]
  fn add_assign(&mut self,t:T) {
    *self = *self + t;
  }
}

impl<T:Additive> Sub<T> for Kahan<T> {
  type Output = Self;
  #[inline]
  fn sub(self,t:T) -> Self {
    self.add(-t)
  }
}
impl<T:Additive> SubAssign<T> for Kahan<T> {
  #[inline]
  fn sub_assign(&mut self,t:T) {
    self.add_assign(-t);
  }
}
impl<T:Additive> Sub<Kahan<T>> for Kahan<T> {
  type Output = Self;
  #[inline]
  fn sub(self,t:Kahan<T>) -> Self {
    self - t.0
  }
}

/*
ksum' :: (Value v) => [v] -> (v -> v -> a) -> a
ksum' terms k = f 0 0 terms
  where
    f !s !e [] = k s e
    f !s !e (t:terms) =
      let !y  = t - e
          !s' = s + y
          !e' = (s' - s) - y
      in if s' == s
         then k s' e'
         else f s' e' terms
\end{code}
\end{titled-frame}
*/

