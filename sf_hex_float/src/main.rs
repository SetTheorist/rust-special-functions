use sf_hex_float::*;

pub fn f(a:f64, b:f64) -> f64 {
  a * b
}

//let pi = hfloats!(:Wide:2:-3.243f6a8885a308d313198a2e03707344a40938223p0);
pub fn main() {
  //const PI : f64 = hfloats!(-3.243f6a8885a308d313198a2e03707344a40938223p0)[0];
  //println!("{:?}", pi);
  println!("{:e}", hfloats!(-3.243f6a8885a308d313198a2e03707344a40938223p0) );
}
