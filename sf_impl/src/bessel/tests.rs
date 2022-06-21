use super::*;
use crate::real::*;

#[test]
fn real_symmetry() {
  assert_eq!(r64(1.0).bessel_j(0), r64(-1.0).bessel_j(0));

  assert_eq!(r64(1.0).bessel_j(1), -r64(1.0).bessel_j(-1));
  assert_eq!(r64(1.0).bessel_j(1), -r64(-1.0).bessel_j(1));
  assert_eq!(r64(1.0).bessel_j(1), r64(-1.0).bessel_j(-1));

  assert_eq!(r64(1.0).bessel_j(2), r64(1.0).bessel_j(-2));
  assert_eq!(r64(1.0).bessel_j(2), r64(-1.0).bessel_j(2));
  assert_eq!(r64(1.0).bessel_j(2), r64(-1.0).bessel_j(-2));
}
