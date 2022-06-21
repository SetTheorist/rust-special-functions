

pub trait AGM : Sized {
  fn agm(self, b:Self) -> Self;
  fn agm_vec(self, b:Self, c0:Self) -> (Vec<Self>,Vec<Self>,Vec<Self>);
  fn agm_vec_extra(self, b:Self, c0:Self, extra:Self) -> (Vec<Self>,Vec<Self>,Vec<Self>,Vec<Self>);
}

pub fn sf_agm<V:AGM>(a:V, b:V) -> V {
  a.agm(b)
}
pub fn sf_agm_vec<V:AGM>(a:V, b:V, c0:V) -> (Vec<V>,Vec<V>,Vec<V>) {
  a.agm_vec(b, c0)
}
pub fn sf_agm_vec_extra<V:AGM>(a:V, b:V, c0:V, extra:V) -> (Vec<V>,Vec<V>,Vec<V>,Vec<V>) {
  a.agm_vec_extra(b, c0, extra)
}

pub mod impls {
use crate::agm::*;
use crate::complex::*;
use crate::real::*;
use crate::traits::*;

impl AGM for r64 {
  fn agm(self,b:Self) -> Self {
    impl_scalar(self, b)
  }
  fn agm_vec(self, b:Self, c0:Self) -> (Vec<Self>,Vec<Self>,Vec<Self>) {
    let (va,vb,vc,vd) = impl_vec(self, b, c0, None);
    (va,vb,vc)
  }
  fn agm_vec_extra(self, b:Self, c0:Self, extra:Self) -> (Vec<Self>,Vec<Self>,Vec<Self>,Vec<Self>) {
    let (va,vb,vc,vd) = impl_vec(self, b, c0, Some(extra));
    (va,vb,vc,vd.unwrap())
  }
}
impl AGM for c64 {
  fn agm(self, b:Self) -> Self {
    impl_scalar(self, b)
  }
  fn agm_vec(self, b:Self, c0:Self) -> (Vec<Self>,Vec<Self>,Vec<Self>) {
    let (va,vb,vc,vd) = impl_vec(self, b, c0, None);
    (va,vb,vc)
  }
  fn agm_vec_extra(self, b:Self, c0:Self, extra:Self) -> (Vec<Self>,Vec<Self>,Vec<Self>,Vec<Self>) {
    let (va,vb,vc,vd) = impl_vec(self, b, c0, Some(extra));
    (va,vb,vc,vd.unwrap())
  }
}

pub fn impl_scalar<V:Value>(a:V, b:V) -> V {
  if a.is_zero() || b.is_zero() {return V::zero;}
  let (a_, b_) = (a, b);
  let (mut a, mut b) = (a, b);
  for n in 1..100 {
    let a0 = a;
    let b0 = b;
    a = (a0 + b0) / 2;
    b = sf_sqrt(a0 * b0);
    if a==b || (a==a0 && b==b0) {
      ::log::debug!("impl_scalar::<{}>({},{}) converged in {} iterations", std::any::type_name::<V>(), a_, b_, n);
      break;
    }
  }
  a
}

use crate::trig::*;
pub fn impl_vec<V:Value+Trig>(a:V, b:V, c0:V, extra:Option<V>) -> (Vec<V>,Vec<V>,Vec<V>,Option<Vec<V>>) {
  if a.is_zero() || b.is_zero() { todo!(); }
  let (a_,b_) = (a,b);
  // TODO: be smarter with vectors...
  // maybe cleaner return value
  let mut va = Vec::new();
  let mut vb = Vec::new();
  let (mut a, mut b) = (a, b);
  for i in 1..1000 {
    va.push(a);
    vb.push(b);
    let a0 = a;
    let b0 = b;
    a = (a0 + b0) / 2;
    b = sf_sqrt(a0 * b0);
    if a==b || (a==a0 && b==b0) {
      ::log::debug!("impl_vec::<{}>({},{},..) converged in {} iterations", std::any::type_name::<V>(), a_, b_, i);
      break;
    }
  }
  va.push(a);
  vb.push(b);
  let n = va.len();
  let mut vc = Vec::with_capacity(n);
  vc.push(c0);
  for i in 1..n {
    vc.push((va[i-1] - vb[i-1]) / 2);
  }
  let vd =
    match extra {
      Some(phi0) => {
        let mut vd = Vec::with_capacity(n);
        vd.push(phi0);
        for i in 1..n {
          let x = vd[i-1] + sf_atan(sf_tan(vd[i-1])*vb[i-1]/va[i-1]);
          vd.push(x);
        }
        Some(vd)
      }
      None => { None }
    };
  (va,vb,vc,vd)
}

}
