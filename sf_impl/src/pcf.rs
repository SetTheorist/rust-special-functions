use crate::traits::*;

// NB Mathematica:
//    ParabolicCylinderD[n,z] = U(-n-1/2, z)
//    ParabolicCylinderD[-a-1/2,z] = U(a, z)
//    for n=0,1,2,...:
//      V(n+1/2,z) = \sqrt(2/π)*exp(z^2/4)*(-i)^n*2^(-n/2)*H_n(iz/\sqrt(2)),
//        H_n(z) = Hermite polynomials
// 
// TODO: maybe remove separate A generic
// (unless there are unique implementation strategies for A integral, for example)
pub trait PCF<A> : Sized {
  fn u(self, a:A) -> Self;
  fn v(self, a:A) -> Self;
  fn uv(self, a:A) -> (Self,Self);

  // D_n(z) = U(-n-1/2,z)
  fn d(self, a:A) -> Self;
}
#[inline] pub fn sf_pcf_u<A,V:PCF<A>>(a:A, z:V) -> V { z.u(a) }
#[inline] pub fn sf_pcf_v<A,V:PCF<A>>(a:A, z:V) -> V { z.v(a) }
#[inline] pub fn sf_pcf_uv<A,V:PCF<A>>(a:A, z:V) -> (V,V) { z.uv(a) }
#[inline] pub fn sf_pcf_d<A,V:PCF<A>>(a:A, z:V) -> V { z.d(a) }

////////////////////////////////////////////////////////////////////////////////

pub mod impls {
use crate::traits::*;
use super::*;
use crate::gamma::*;
use crate::exp::*;
use crate::trig::*;

////////////////////////////////////////////////////////////////////////////////

pub fn uv_series<V:Value+Power+Gamma+Exp+Trig,const Ub:bool,const Vb:bool>(a:V, z:V) -> (V,V) {
  // TODO: switch to alternate series in bad cases
  let e = uv_even(a,z);
  let o = uv_odd(a,z);
  let u = if !Ub {V::nan} else {u_u0(a) * e + u_du0(a) * o};
  let v = if !Vb {V::nan} else {v_v0(a) * e + v_dv0(a) * o};
  (u, v)
}

pub fn u_u0<V:Value+Power+Gamma>(a:V) -> V {
  V::SQRTPI * (ι(2):V).pow(-a/2-0.25) / sf_gamma(a/2 + 0.75)
}

pub fn u_du0<V:Value+Power+Gamma>(a:V) -> V {
  -V::SQRTPI * (ι(2):V).pow(-a/2+0.25) / sf_gamma(a/2 + 0.25)
}

pub fn v_v0<V:Value+Power+Gamma+Trig>(a:V) -> V {
  (ι(2):V).pow(a/2+0.25) * sf_sin_pix(-a/2+0.75) / sf_gamma(-a/2 + 0.75)
}

pub fn v_dv0<V:Value+Power+Gamma+Trig>(a:V) -> V {
  (ι(2):V).pow(a/2+0.75) * sf_sin_pix(-a/2+0.25) / sf_gamma(-a/2 + 0.25)
}

// fails when 2*a = -(4*n+1) for some n>=0
pub fn uv_even<V:Value+Exp>(a:V, z:V) -> V {
  let z2 = z.sqr();
  let a2 = a*2;
  let mut sum = V::one;
  let mut t = V::one;
  for n in 1..1000 {
    t *= z2*(a2+(n*4-3))/2/((2*n)*(2*n-1));
    let old = sum;
    sum += t;
    if sum == old {print!(":{}:",n);break;}
  }
  sum * sf_exp(-z2/4)
}

// fails when 2*a = (4*n+1) for some n>=0
pub fn uv_even2<V:Value+Exp>(a:V, z:V) -> V {
  let z2 = z.sqr();
  let a2 = a*2;
  let mut sum = V::one;
  let mut t = V::one;
  for n in 1..1000 {
    t *= z2*(a2-(n*4-3))/2/((2*n)*(2*n-1));
    let old = sum;
    sum += t;
    if sum == old {print!(";{};",n);break;}
  }
  sum * sf_exp(z2/4)
}

// fails when 2*a = -(4*n+3) for some n>=0
pub fn uv_odd<V:Value+Exp>(a:V, z:V) -> V {
  let z2 = z.sqr();
  let a2 = a*2;
  let mut sum = z;
  let mut t = z;
  for n in 1..1000 {
    t *= z2*(a2+(n*4-1))/2/((2*n)*(2*n+1));
    let old = sum;
    sum += t;
    if sum == old {print!(":{}:",n);break;}
  }
  sum * sf_exp(-z2/4)
}

// fails when 2*a = (4*n+3) for some n>=0
pub fn uv_odd2<V:Value+Exp>(a:V, z:V) -> V {
  let z2 = z.sqr();
  let a2 = a*2;
  let mut sum = z;
  let mut t = z;
  for n in 1..1000 {
    t *= z2*(a2-(n*4-1))/2/((2*n)*(2*n+1));
    let old = sum;
    sum += t;
    if sum == old {print!(";{};",n);break;}
  }
  sum * sf_exp(z2/4)
}

////////////////////////////////////////////////////////////////////////////////

// quadrature
//  exp(-z^2/4)/Gamma(a+1/2) * \int_0^\infty t^(a-1/2)*exp(-t^2/2-z*t) dt
// 

////////////////////////////////////////////////////////////////////////////////

// reasonable for large a and small z
// assume a>0
pub fn u_recur_up<A:RealValue,V:Value+Power+Gamma+Exp+Trig>(a:A, z:V) -> V
  where V:Embeds<A>
{
  let nn = a.trunc();
  let afrac = a - nn;
  let (mut m2,_) = uv_series::<_,true,false>(ι(afrac), z);
  let (mut m1,_) = uv_series::<_,true,false>(ι(afrac+1), z);
  for n in 2..(nn.rint()+1) {
    let mm = (m2 - z*m1) / (afrac + n - 0.5);
    m2 = m1;
    m1 = mm;
  }
  m1
}

// better for z>1? but still need initial values
pub fn u_recur_dn<A:RealValue,V:Value+Power+Gamma+Exp+Trig>(a:A, z:V) -> V
  where V:Embeds<A>
{
  const EXTRA : isize = 100;
  let nn = a.trunc();
  let afrac = a - nn;
  let (u0,_) = uv_series::<_,true,false>(ι(afrac), z);
  let mut m2 = V::zero;
  let mut m1 = V::one;
  let mut mn = V::zero;
  for n in (0..(nn.rint()+EXTRA)).rev() {
    let mm = z*m1 + m2*(afrac + n+1 + 0.5);
    m2 = m1;
    m1 = mm;
    if nn == n { mn = mm; }
  }
  mn * u0/m1
}

// better for z>1? works a bit better when previous fails (if z>>1)?
pub fn u_recur_dn2<A:RealValue+Gamma,V:Value+Power<A>+Exp+Trig>(a:A, z:V) -> V
  where V:Embeds<A>
{
  // assume a>0
  const EXTRA : isize = 100;
  let nn = a.trunc();
  let afrac = a - nn;
  let z2 = z/2;

  let aa = a + EXTRA + 2;
  let p = sf_sqrt(aa);
  let p2 = p*2;
  let zp = z2/p2;
  let v = -z2.pow(3)/(p*3) - zp.pow(2) - (z2 - z2.pow(5)*2/5)/p2.pow(3) + zp.pow(4)*2;
  let mut m2 = V::SQRTPI/((ι(2):V).pow(aa/2+0.25)*sf_gamma(aa/2+0.75))*sf_exp(-z*p+v);

  let aa = a + EXTRA + 1;
  let p = sf_sqrt(aa);
  let p2 = p*2;
  let zp = z2/p2;
  let v = -z2.pow(3)/(p*3) - zp.pow(2) - (z2 - z2.pow(5)*2/5)/p2.pow(3) + zp.pow(4)*2;
  let mut m1 = V::SQRTPI/((ι(2):V).pow(aa/2+0.25)*sf_gamma(aa/2+0.75))*sf_exp(-z*p+v);
  
  for n in (0..(nn.rint()+EXTRA+1)).rev() {
    let mm = m1*z + m2*(afrac + n+1 + 0.5);
    m2 = m1;
    m1 = mm;
    if nn == n {break;}
  }
  m1
}

////////////////////////////////////////////////////////////////////////////////

pub fn u_asymp_z<V:Value+Exp+Power>(a:V, z:V) -> V {
  let z2 = z.sqr();
  let a2 = a*2;
  let mut sum = V::zero;
  let mut t = V::one;
  for s in 1..1000 {
    let old_sum = sum;
    sum += t;
    if sum == old_sum {print!(",{},",s);break;}
    let old_t = t;
    t *= -(a2+s*2-1)/2*(a2+s*2)/2/(z2*2*s);
    if μ(t) > μ(old_t) {print!("'{}'",s);break;}
  }
  sum * sf_exp(-z2/4) * z.pow(-a-0.5)
}

}
