use sf_hex_float::hexf;
macro_rules! Wide { ($x:tt) => { hexf!(:2:Wide:$x) } }

pub trait Airy : Sized {
  fn airy_ai(self) -> Self;
  fn airy_bi(self) -> Self;

  fn airy_aibi(self) -> (Self,Self);
}
#[must_use]#[inline] pub fn sf_airy_ai<V:Airy>(z:V) -> V { z.airy_ai() }
#[must_use]#[inline] pub fn sf_airy_bi<V:Airy>(z:V) -> V { z.airy_bi() }
#[must_use]#[inline] pub fn sf_airy_aibi<V:Airy>(z:V) -> (V,V) { z.airy_aibi() }

pub trait AiryConstants {
  const AI_0  : Self;  // =  sf_exp(-sf_log(3)*2/3 - sf_lngamma(2/3))
  const AI_0_DIGITS : &'static str = 
    "0.5ae31e589c56e17a96d7bb04e64f6da97ab1006b26f9eb6421233394220b8457047cb9557c9f3b43d25";
  const DAI_0 : Self;  // = -sf_exp(-sf_log(3)*1/3 - sf_lngamma(1/3))
  const DAI_0_DIGITS : &'static str =
    "-0.4241fd0adc3e2a2eba6fca5021d9ee1e6a31f0935d3a8b0fa6c28c3a9c0b546160e3c53e9e70f05d7ec";
  const BI_0  : Self;  // =  sf_exp(-sf_log(3)*1/6 - sf_lngamma(2/3))
  const BI_0_DIGITS : &'static str =
    "0.9d6bd4da51f54baaecb6804cf7a2ebed9295e0411ef5e1837c243549c5f62ba6b264f543ab24817a527";
  const DBI_0 : Self;  // =  sf_exp( sf_log(3)*1/6 - sf_lngamma(1/3))
  const DBI_0_DIGITS : &'static str =
    "0.72c3069a0322822c217efc48899a487ad1b3abb9b4b8f6e55fe15334ba584999b84b702b5d6bdcaf4bd";

}

pub mod impls;