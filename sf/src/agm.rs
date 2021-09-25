

pub trait AGM : Sized {
  fn agm(self,b:Self) -> Self;
  fn agm_vec(self,b:Self,extra:bool) -> (Self,Vec<Self>,Vec<Self>,Vec<Self>,Option<Vec<Self>>);
}

pub mod impls {
use crate::traits::*;

pub fn impl_scalar<V:Value>(a:V, b:V) -> V {
  let mut a = a;
  let mut b = b;
  for n in 1.. {
    let a0 = a;
    let b0 = b;
    a = (a0 + b0) / 2;
    b = sf_sqrt(a0 * b0);
    if a==b {print!("({})",n);break;}
  }
  a
}

use crate::trig::*;
pub fn impl_vec<V:Value+Trig>(a:V, b:V, extra:Option<V>) -> (V,Vec<V>,Vec<V>,Vec<V>,Option<Vec<V>>) {
  // TODO: be smarter with vectors...
  // maybe cleaner return value
  let mut va = Vec::new();
  let mut vb = Vec::new();
  let mut a = a;
  let mut b = b;
  for i in 1.. {
    va.push(a);
    vb.push(b);
    let a0 = a;
    let b0 = b;
    a = (a0 + b0) / 2;
    b = sf_sqrt(a0 * b0);
    if a==b {print!("<{}>",i);break;}
  }
  va.push(a);
  vb.push(b);
  let n = va.len();
  let mut vc = Vec::with_capacity(n);
  for i in 0..n {
    vc.push((va[i] - vb[i]) / 2);
  }
  let vd =
    match extra {
      Some(phi0) => {
        let mut vd = Vec::with_capacity(n);
        vd.push(V::zero);
        vd.push(phi0);
        for i in 2..n {
          let x = vd[i-1] + sf_atan(sf_tan(vd[i-1])*vb[i-1]/va[i-1]);
          vd.push(x);
        }
        Some(vd)
      }
      None => { None }
    };
  (a,va,vb,vc,vd)
}

}
