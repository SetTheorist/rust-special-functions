
pub trait JacobiElliptic : Sized {
  fn cn(self, k:Self) -> Self;
  fn dn(self, k:Self) -> Self;
  fn sn(self, k:Self) -> Self;
  // returns (cn,dn,sn)
  fn jacobi(self, k:Self) -> (Self,Self,Self);
}


pub mod impls {
use crate::traits::*;
use crate::agm::*;
use crate::ellint::{sf_kc};
use crate::trig::*;

macro_rules! agm {
  ($fn:ident; $cn:tt,$dn:tt,$sn:tt; $val:expr; $cnf:expr,$dnf:expr,$snf:expr) => {
    #[inline]
    pub fn $fn<V:Value+AGM+Trig+Float>(x:V, k:V) -> V {
      let ($cn,$dn,$sn) = jacobi_agm_general::<V,$cnf,$dnf,$snf>(x,k);
      $val
    }
  }
}

agm!{jacobi_agm_cn; cn, _, _; cn; true, false ,false}
agm!{jacobi_agm_dn; _, dn, _; dn; false, true ,false}
agm!{jacobi_agm_sn; _, _, sn; sn; false, false ,true}

agm!{jacobi_agm_nc; cn, _, _; cn.recip(); true, false ,false}
agm!{jacobi_agm_nd; _, dn, _; dn.recip(); false, true ,false}
agm!{jacobi_agm_ns; _, _, sn; sn.recip(); false, false ,true}

agm!{jacobi_agm_cd; cn, dn, _; cn/dn; true, true ,false}
agm!{jacobi_agm_dc; cn, dn, _; dn/cn; true, true ,false}
agm!{jacobi_agm_cs; cn, _, sn; cn/sn; true, false ,true}
agm!{jacobi_agm_sc; cn, _, sn; sn/cn; true, false ,true}
agm!{jacobi_agm_ds; _, dn, sn; dn/sn; false, true ,true}
agm!{jacobi_agm_sd; _, dn, sn; sn/dn; false, true ,true}

////////////////////////////////////////////////////////////////////////////////

// TODO: Appears to give the incorrect values at k==1 (e.g. z=2)
// (though nearby values of k (0.999) are still correct)
pub fn jacobi_agm_general
  <V:Value+AGM+Trig+Float, const CN:bool, const DN:bool, const SN:bool>
  (x:V, k:V) -> (V,V,V)
{
  if k.is_zero() {
    return (
      if CN {sf_cos(x)} else {V::nan},
      if DN {V::one   } else {V::nan},
      if SN {sf_sin(x)} else {V::nan},
      );
  } else if k == 1 {
    return (
      if CN {sf_sech(x)} else {V::nan},
      if DN {sf_sech(x)} else {V::nan},
      if SN {sf_tanh(x)} else {V::nan},
      );
  }
  let (va,vb,vc) = sf_agm_vec(V::one, sf_kc(k), k);
  let n = va.len();
  let mut phi_0 = (va[n-1]*x)<<((n-1) as isize);
  //let mut phi_1 = V::zero;
  for i in (0..(n-1)).rev() {
    //phi_1 = phi_0;
    //phi_0 = (phi_1 + sf_asin(sf_sin(phi_1) * vc[i+1] / va[i+1])) / 2;
    phi_0 = (phi_0 + sf_asin(sf_sin(phi_0) * vc[i+1] / va[i+1])) / 2;
  }
  let cn = if CN {sf_cos(phi_0)} else {V::nan};
  let sn = if SN||DN {sf_sin(phi_0)} else {V::nan};
  // dn = cn / sf_cos(phi_1 - phi_0);
  let dn = if DN {sf_sqrt(V::one - (k*sn).sqr())} else {V::nan};
  (cn,dn,sn)
}

}
