#![allow(dead_code)]
use proc_macro::TokenStream;
use proc_macro::TokenTree::*;

// TODO: rounding-modes?

#[proc_macro]
// hexf!(:3:f:"3.243f6")
// hexf!(3.243f6)
// hexf!("3.243f6")
pub fn hexf(item:TokenStream) -> TokenStream {
  let mut item = item.into_iter().peekable();
  let constructor_form;
  if let Some(Punct(x)) = item.peek() {
    if x.to_string() == ":" {
      item.next(); // consume ":"
      let n : usize = item.next().unwrap().to_string().parse().unwrap();
      item.next(); // consume ":"
      let id = { 
        match item.next() {
          Some(Ident(x)) => { x.to_string() }
          z => { panic!("Expected id not {:?}", z); }
        }
      };
      constructor_form = Some((n,id));
    } else {
      constructor_form = None;
    }
  } else {
    constructor_form = None;
  }
  let mut s = String::new();
  for it in item {
    match it {
      Group(x) => {panic!("Unexpected group token {}", x.to_string());}
      Ident(x) => {s+=&x.to_string();}
      Punct(x) => {s+=&x.to_string();}
      Literal(x) => {s+=&x.to_string();}
    }
  }
  s.retain(|c|"0123456789abcdefABCDEF+-.p".contains(c));
  let v = parse_hex_f64s(&s);
  let res =
    match constructor_form {
      // TODO: if n<v.size() ...
      Some((n,id)) => {
        format!("{}({})",
          id,
          &(v.iter().take(n).map(|x|format!("{:e}",x)).collect::<Vec<String>>().join(","))
        )
      }
      None => { format!("{:?}",v) }
    };
  res.parse().unwrap()
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum RoundingMode {
  ToNearest, // TiesToEven
  ToZero,
  ToPosInfinity,
  ToNegInfinity,
  //ToNearestTiesToZero
}

////////////////////////////////////////////////////////////////////////////////

#[inline]
// msbit of byte 0 = bit #0, etc.
fn bit(bytes:&[u8], n:usize) -> u64 {
  let b = n/8;
  if b<bytes.len() {((bytes[b]>>(7-(n%8)))&1) as u64} else {0}
}

#[inline]
fn hex(c:char) -> u8 {
  match c {
    '0' => 0, '1' => 1,
    '2' => 2, '3' => 3,
    '4' => 4, '5' => 5,
    '6' => 6, '7' => 7,
    '8' => 8, '9' => 9,
    'a'|'A' => 10,
    'b'|'B' => 11,
    'c'|'C' => 12,
    'd'|'D' => 13,
    'e'|'E' => 14,
    'f'|'F' => 15,
    _ => 0,
  }
}

// TODO: optional initial sign!
fn parse_hex_f64s(s:&str) -> Vec<f64> {
  let digits;
  let exp : isize;
  if let Some((ds,es)) = s.split_once('p') {
    exp = 4 * es.parse::<isize>().unwrap();
    digits = ds;
  } else {
    exp = 0;
    digits = s;
  }
  let (bs,o,negative) = string_to_bytes(digits);
  let mut exp = exp + (o as isize) - 1;
  let mut start = 0;
  let mut fs = Vec::with_capacity(1 + o/6);
  while start < bs.len()*8 {
    let (f,s) = parse_bytes_f64(&bs, exp, start);
    if f == 0.0 {break;} // TODO
    fs.push(if negative {-f} else {f});
    start = s;
    exp -= s as isize;
  }
  fs
}

// starts reading bits from start bit, returns next bit after last significant used
// always returns positive float
fn parse_bytes_f64(bytes:&[u8], exp:isize, start:usize) -> (f64,usize) {
  let maxn = bytes.len()*8;
  let mut exp = exp;
  let mut n = start;
  // find first 1 bit
  while n<maxn && bit(bytes,n)==0 { n+=1; exp-=1; }
  if n==maxn { return (0.0,n); }
  // skip implicit 1
  n += 1;
  let mut mantissa = 0;
  for j in 0..52 {
    mantissa = (mantissa<<1) | bit(bytes, n+j);
  }
  if exp < -1022 {return (0.0, maxn);} // TODO
  let exp = ((exp+1023) as u64) << 52;
  let f = f64::from_bits(exp|mantissa);
  (f, (n+52).min(maxn))
}

// isize = shift in bits from beginning of string to decimal point = starting binary exponent...
// expects a string of the form h*(.h*)?
fn string_to_bytes(s:&str) -> (Vec<u8>,usize,bool) {
  let negative = s.contains('-'); // TODO: pretty hacky, negative sign can be anywhere...
  let dp = if let Some(x)=s.find('.') {x} else {s.len()};
  let (predigs,postdigs) = s.split_at(dp);
  let predigs : Vec<u8> = predigs.chars().map(|c|hex(c)).collect();
  let postdigs = if postdigs.len()>0 {&postdigs[1..]} else {postdigs};
  let postdigs : Vec<u8> = postdigs.chars().map(|c|hex(c)).collect();
  let pren = (predigs.len()+1)/2;
  let postn = (postdigs.len()+1)/2;
  let mut arr = vec![0; pren + postn];
  if predigs.len() % 2 == 0 {
    for i in (0..predigs.len()).step_by(2) {
      arr[i/2] = (predigs[i]<<4)|predigs[i+1];
    }
  } else {
    arr[0] = predigs[0];
    for i in (1..predigs.len()).step_by(2) {
      arr[(i+1)/2] = (predigs[i]<<4)|predigs[i+1];
    }
  }
  if postdigs.len() > 0 {
    for i in (0..(postdigs.len()-1)).step_by(2) {
      arr[pren + (i/2)] = (postdigs[i]<<4)|postdigs[i+1];
    }
    if postdigs.len() % 2 == 1 {
      arr[pren + postn - 1] = postdigs[postdigs.len()-1] << 4;
    }
  }
  (arr,pren*8,negative)
}

