
pub trait Airy {
  fn airy_ai(self) -> Self;
  fn airy_bi(self) -> Self;

  fn airy_aibi(self) -> (Self,Self);
}
