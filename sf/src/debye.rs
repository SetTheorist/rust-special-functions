
pub trait Debye {
  fn debye(self, n:usize) -> Self;
  fn debye_scaled(self, n:usize) -> Self;
}
