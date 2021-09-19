pub trait ConfluentHypergeometric {}

pub trait Hypergeometric0F1 {}

pub trait Hypergeometric1F0 {}

pub trait Hypergeometric1F1 {}

pub trait Hypergeometric2F1 {}

pub trait HypergeometricpFq
where Self: Sized
{
  fn f_pfq(self, p: usize, q: usize, ps: &[Self], qs: &[Self]) -> Self;
  fn f_pfqx<const P: usize, const Q: usize>(self, ps: &[Self; P], qs: &[Self; Q]) -> Self;
}

/*
pub trait HypergeometricpFq<const P:usize, const usize Q:usize> {
}
*/
