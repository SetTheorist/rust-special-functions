
pub trait FloatType {
  type U;
  const SIGNB : Self::U;
  const EXPB  : Self::U;
  const MANB  : Self::U;
  const IMPB  : Self::U;
  const SHIFT : u32;
  const BIAS : i32;
  const MAXE : i32;
  const MINE : i32;
  const B_INF : Self::U;
  const B_NAN : Self::U;
  const B_ZERO : Self::U;
}

/*
//[1:5:10(+1)]
impl FloatType for f16 {
  type U = u16;
  const SIGNB : Self::U = 0x8000;
  const EXPB  : Self::U = 0x7C00;
  const MANB  : Self::U = 0x03FF;
  const IMPB  : Self::U = 0x0400;
  const SHIFT : u32 = 10;
  const BIAS : i32 = 15;
  const MAXE : i32 = 15;
  const MINE : i32 = -14;
  const B_INF : Self::U = Self::EXPB;
  const B_NAN : Self::U = Self::EXPB|Self::MANB;
  const B_ZERO : Self::U = 0;
}
*/

//[1:8:23(+1)]
impl FloatType for f32 {
  type U = u32;
  const SIGNB : Self::U = 0x8000_0000;
  const EXPB  : Self::U = 0x7F80_0000;
  const MANB  : Self::U = 0x007F_FFFF;
  const IMPB  : Self::U = 0x0080_0000;
  const SHIFT : u32 = 23;
  const BIAS : i32 = 127;
  const MAXE : i32 = 127;
  const MINE : i32 = -126;
  const B_INF : Self::U = Self::EXPB;
  const B_NAN : Self::U = Self::EXPB|Self::MANB;
  const B_ZERO : Self::U = 0;
}

//[1:11:52(+1)]
impl FloatType for f64 {
  type U = u64;
  const SIGNB : Self::U = 0x8000_0000__0000_0000;
  const EXPB  : Self::U = 0x7FF0_0000__0000_0000;
  const MANB  : Self::U = 0x000F_FFFF__FFFF_FFFF;
  const IMPB  : Self::U = 0x0010_0000__0000_0000;
  const SHIFT : u32 = 52;
  const BIAS : i32 = 1023;
  const MAXE : i32 = 1023;
  const MINE : i32 = -1022;
  const B_INF : Self::U = Self::EXPB;
  const B_NAN : Self::U = Self::EXPB|Self::MANB;
  const B_ZERO : Self::U = 0;
}

/*
//[1:15:112(+1)]
impl FloatType for f128 {
  type U = u128;
  const SIGNB : Self::U = 0x8000_0000__0000_0000___0000_0000__0000_0000;
  const EXPB  : Self::U = 0x7FFF_0000__0000_0000___0000_0000__0000_0000;
  const MANB  : Self::U = 0x0000_FFFF__FFFF_FFFF___FFFF_FFFF__FFFF_FFFF;
  const IMPB  : Self::U = 0x0001_0000__0000_0000___0000_0000__0000_0000;
  const SHIFT : u32 = 112;
  const BIAS : i32 = 16383;
  const MAXE : i32 = 16383;
  const MINE : i32 = -16382;
  const B_INF : Self::U = Self::EXPB;
  const B_NAN : Self::U = Self::EXPB|Self::MANB;
  const B_ZERO : Self::U = 0;
}
*/


