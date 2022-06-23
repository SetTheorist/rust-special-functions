use super::r64;
use crate::traits::*;

/*
use approx::{*};
impl AbsDiffEq for r64 {
  type Epsilon = <f64 as AbsDiffEq>::Epsilon;
  fn default_epsilon() -> Self::Epsilon {
    f64::default_epsilon()
  }
  fn abs_diff_eq(&self, other:&Self, epsilon:Self::Epsilon) -> bool {
    f64::abs_diff_eq(&self.0, &other.0, epsilon)
  }
}
impl RelativeEq for r64 {
  fn default_max_relative() -> Self::Epsilon {
    f64::default_max_relative()
  }
  fn relative_eq(&self, other:&Self, epsilon:Self::Epsilon, max_relative:Self::Epsilon) -> bool {
    f64::relative_eq(&self.0, &other.0, epsilon, max_relative)
  }
}
impl UlpsEq for r64 {
  fn default_max_ulps() -> u32 {
    f64::default_max_ulps()
  }
  fn ulps_eq(&self, other:&Self, epsilon:Self::Epsilon, max_ulps:u32) -> bool {
    f64::ulps_eq(&self.0, &other.0, epsilon, max_ulps)
  }
}
*/

#[test]
fn basic_ops_0() {
  for ia in -100..=100 {
    for ib in -100..=100 {
      let a: f64 = (ia as f64) / 10.0;
      let b: f64 = (ib as f64) / 10.0;
      let ra: r64 = r64(a);
      let rb: r64 = r64(b);

      assert_eq!((a + b), (ra + rb).0);
      assert_eq!(r64(a + b), (ra + rb));
      assert_eq!((a - b), (ra - rb).0);
      assert_eq!(r64(a - b), (ra - rb));
      assert_eq!((a * b), (ra * rb).0);
      assert_eq!(r64(a * b), (ra * rb));
      if ia != 0 || ib != 0 {
        assert_eq!((a / b), (ra / rb).0);
        assert_eq!(r64(a / b), (ra / rb));
      }
      if ib != 0 {
        assert_eq!((a % b), (ra % rb).0);
        assert_eq!(r64(a % b), (ra % rb));
      }

      assert_eq!(ra.recip(), 1 / ra);
      assert_eq!(rb.recip(), 1 / rb);
    }
  }
}

#[test]
fn basic_ops_1() {
  for ia in -100..=100 {
    for ib in -100..=100 {
      let a: f64 = (ia as f64) / 10.0;
      let b: f64 = (ib as f64) / 10.0;
      let ra: r64 = r64(a);
      let rb: r64 = r64(b);

      assert_eq!(-r64(a), r64(-a));
      assert_eq!(ra.sqr(), ra * ra);
      assert_eq!(ra << 0, ra);
      assert_eq!(ra << 1, ra * 2);
      assert_eq!(ra << 2, ra * 4);
      assert_eq!(ra >> 0, ra);
      assert_eq!(ra >> 1, ra / 2);
      assert_eq!(ra >> 2, ra / 4);
      assert_eq!(ra << -1, ra >> 1);
      assert_eq!(ra << -2, ra >> 2);
      assert_eq!(ra >> -1, ra << 1);
      assert_eq!(ra >> -2, ra << 2);

      assert_eq!(ra, ra);
      assert_ne!(ra, ra + 1);
    }
  }
}

#[test]
fn basic_ops_2() {
  for ia in -100..=100 {
    for ib in -100..=100 {
      let a: f64 = (ia as f64) / 10.0;
      let b: f64 = (ib as f64) / 10.0;
      let ra: r64 = r64(a);
      let rb: r64 = r64(b);

      assert!(ra <= ra);
      assert!(ra <= ra + 1);
      assert!(!(ra + 1 <= ra));
      assert!(!(ra < ra));
      assert!(ra < ra + 1);
      assert!(!(ra + 1 < ra));
      assert!(ra >= ra);
      assert!(!(ra >= ra + 1));
      assert!(ra + 1 >= ra);
      assert!(!(ra > ra));
      assert!(!(ra > ra + 1));
      assert!(ra + 1 > ra);
    }
  }

  assert_eq!(r64(1.0), 1.0);
  assert_eq!(r64(1.0), 1);
  assert_ne!(r64(1.0), 2.0);
  assert_ne!(r64(1.0), 2);
}

#[test]
fn rounding_ops() {
  assert_eq!(r64(0.5).floor(), r64(0.0));
  assert_eq!(r64(0.5).ceil(), r64(1.0));
  assert_eq!(r64(0.5).round(), r64(1.0));
  assert_eq!(r64(0.5).trunc(), r64(0.0));
  assert_eq!(r64(0.5).rint(), 1);

  assert_eq!(r64(1.0).floor(), r64(1.0));
  assert_eq!(r64(1.0).ceil(), r64(1.0));
  assert_eq!(r64(1.0).round(), r64(1.0));
  assert_eq!(r64(1.0).trunc(), r64(1.0));
  assert_eq!(r64(1.0).rint(), 1);

  assert_eq!(r64(1.1).floor(), r64(1.0));
  assert_eq!(r64(1.1).ceil(), r64(2.0));
  assert_eq!(r64(1.1).round(), r64(1.0));
  assert_eq!(r64(1.1).trunc(), r64(1.0));
  assert_eq!(r64(1.1).rint(), 1);

  assert_eq!(r64(1.5).floor(), r64(1.0));
  assert_eq!(r64(1.5).ceil(), r64(2.0));
  assert_eq!(r64(1.5).round(), r64(2.0));
  assert_eq!(r64(1.5).trunc(), r64(1.0));
  assert_eq!(r64(1.5).rint(), 2);

  assert_eq!(r64(1.9).floor(), r64(1.0));
  assert_eq!(r64(1.9).ceil(), r64(2.0));
  assert_eq!(r64(1.9).round(), r64(2.0));
  assert_eq!(r64(1.9).trunc(), r64(1.0));
  assert_eq!(r64(1.9).rint(), 2);

  assert_eq!(r64(-0.5).floor(), r64(-1.0));
  assert_eq!(r64(-0.5).ceil(), r64(0.0));
  assert_eq!(r64(-0.5).round(), r64(-1.0));
  assert_eq!(r64(-0.5).trunc(), r64(0.0));
  assert_eq!(r64(-0.5).rint(), -1);

  assert_eq!(r64(-1.0).floor(), r64(-1.0));
  assert_eq!(r64(-1.0).ceil(), r64(-1.0));
  assert_eq!(r64(-1.0).round(), r64(-1.0));
  assert_eq!(r64(-1.0).trunc(), r64(-1.0));
  assert_eq!(r64(-1.0).rint(), -1);

  assert_eq!(r64(-1.1).floor(), r64(-2.0));
  assert_eq!(r64(-1.1).ceil(), r64(-1.0));
  assert_eq!(r64(-1.1).round(), r64(-1.0));
  assert_eq!(r64(-1.1).trunc(), r64(-1.0));
  assert_eq!(r64(-1.1).rint(), -1);

  assert_eq!(r64(-1.5).floor(), r64(-2.0));
  assert_eq!(r64(-1.5).ceil(), r64(-1.0));
  assert_eq!(r64(-1.5).round(), r64(-2.0));
  assert_eq!(r64(-1.5).trunc(), r64(-1.0));
  assert_eq!(r64(-1.5).rint(), -2);

  assert_eq!(r64(-1.9).floor(), r64(-2.0));
  assert_eq!(r64(-1.9).ceil(), r64(-1.0));
  assert_eq!(r64(-1.9).round(), r64(-2.0));
  assert_eq!(r64(-1.9).trunc(), r64(-1.0));
  assert_eq!(r64(-1.9).rint(), -2);
}

#[test]
fn power_ops() {
  assert_eq!(r64(10.0).recip(), 0.1);
  assert_eq!(r64(10.0).sqr(), 100);
  assert_eq!(r64(100.0).sqrt(), 10);
  assert_eq!(r64(1000.0).cbrt(), 10);

  for ia in 1..=100 {
    let a: f64 = (ia as f64) / 10.0;
    let ra: r64 = r64(a);

    assert_eq!(ra.sqr().sqrt(), ra);
    assert!((ra.sqrt().sqr() - ra).abs() < ra.abs() * r64::epsilon * 2);

    assert!((ra.pow(3_usize).cbrt() - ra).abs() < ra.abs() * r64::epsilon * 8);
    assert!((ra.cbrt().pow(3_usize) - ra).abs() < ra.abs() * r64::epsilon * 8);

    assert_eq!(ra.pow(0_usize), r64::one);
    assert_eq!(ra.pow(0_isize), r64::one);
    assert_eq!(ra.pow(r64(0.0)), r64::one);

    assert_eq!(ra.pow(1_usize), ra);
    assert_eq!(ra.pow(1_isize), ra);
    assert_eq!(ra.pow(r64(1.0)), ra);

    assert_eq!(ra.pow(3_usize), (ra * ra) * (ra));
    assert_eq!(ra.pow(3_isize), (ra * ra) * (ra));
    assert_eq!(ra.pow(-3_isize), ((ra * ra) * (ra)).recip());
    assert!((ra.pow(r64(3.0)) - (ra * ra) * (ra)).abs() < r64::epsilon * 2 * (ra.pow(3_usize)));

    assert_eq!(ra.pow(4_usize), (ra * ra) * (ra * ra));
    assert_eq!(ra.pow(4_isize), (ra * ra) * (ra * ra));
    assert_eq!(ra.pow(-4_isize), ((ra * ra) * (ra * ra)).recip());
    assert!((ra.pow(r64(4.0)) - (ra * ra) * (ra * ra)).abs() < r64::epsilon * 2 * (ra.pow(4_usize)));
  }
}
