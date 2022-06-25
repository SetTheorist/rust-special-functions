use crate::real::*;
use crate::traits::*;
use super::*;

impl PCF<r64> for r64 {
  fn u(self, a:r64) -> Self {
    unimplemented!()
  }

  fn v(self, a:r64) -> Self {
    unimplemented!()
  }

  fn uv(self, a:r64) -> (Self,Self) {
    unimplemented!()
  }

  // D_n(z) = U(-n-1/2,z)
  fn d(self, a:r64) -> Self {
    self.u(-a - 0.5)
  }
}