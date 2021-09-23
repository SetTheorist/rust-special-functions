
pub trait ConfluentHypergeometric<N> {
  fn chg_kummer_m(self, a:N, b:N) -> Self;
  fn chg_kummer_m2(self, a:N, b:N) -> Self;
  fn chg_kummer_u(self, a:N, b:N) -> Self;

  fn chg_whittaker_m(self, k:N, m:N) -> Self;
  fn chg_whittaker_w(self, k:N, m:N) -> Self;
}

pub trait Hypergeometric0F1<N> {
  fn hypergeom_0f1(self, b0:N) -> Self;
}
pub fn sf_hypergeom_0f1<N,V>(b0:N, z:V) -> V
where V:Hypergeometric0F1<N> {
  z.hypergeom_0f1(b0)
}

pub trait Hypergeometric1F0<N> {
  fn hypergeom_1f0(self, a0:N) -> Self;
}
pub fn sf_hypergeom_1f0<N,V>(a0:N, z:V) -> V
where V:Hypergeometric1F0<N> {
  z.hypergeom_1f0(a0)
}

pub trait Hypergeometric1F1<N> {
  fn hypergeom_1f1(self, a0:N, b0:N) -> Self;
}
pub fn sf_hypergeom_1f1<N,V>(a0:N, b0:N, z:V) -> V
where V:Hypergeometric1F1<N> {
  z.hypergeom_1f1(a0, b0)
}

pub trait Hypergeometric2F1<N> {
  fn hypergeom_2f1(self, a0:N, a1:N, b0:N) -> Self;
}
pub fn sf_hypergeom_2f1<N,V>(a0:N, a1:N, b0:N, z:V) -> V
where V:Hypergeometric2F1<N> {
  z.hypergeom_2f1(a0, a1, b0)
}
  
pub trait HypergeometricPFQ<N>
where N:Sized
{
  fn hypergeom_pfq(self, ps: &[N], qs: &[N]) -> Self;

  fn hypergeom_pfqx<const P: usize, const Q: usize>(
    self, ps: &[N; P], qs: &[N; Q]) -> Self;
}

/*
pub trait HypergeometricpFq<const P:usize, const usize Q:usize> {
}
*/
