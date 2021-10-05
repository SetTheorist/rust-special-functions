#![allow(clippy::comparison_chain)]
#![allow(clippy::eq_op)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::float_cmp)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]
#![allow(clippy::wrong_self_convention)]
#![allow(confusable_idents)]
#![allow(dead_code)]
#![allow(mixed_script_confusables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![feature(bench_black_box)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_trait_impl)]
#![feature(destructuring_assignment)]
#![feature(trait_alias)]
#![feature(type_ascription)]
//#![feature(marker_trait_attr)] // #[marker]
//#![feature(never_type)]
//#![feature(optimize_attribute)] // [#optimize(speed)]
//#![feature(specialization)]

// ** IDEAS, REMINDERS:
//
// cf. "Inverse Symbolic Calculator"
//
// loop { break returnValue; }  
// no_std??
// c.f. Haskell Numeric.Compensated (E.Kmett) vs. qd/Wide
//
// use proc.macro. for high-precision constants:
//   parse into 1,2,4,8 correctly rounded double sequences (& float?)
//   (or precision on parameter)
//   (or pass in constructor even?)
//   e.g. const Wide::pi = float!(2;Wide(#0,#1);3.1415926535897932384626...)
// note mathematica can generate binary or hexadecimal floating-point:
//   NumberForm[...] and BaseForm[...]
//   e.g. const Wide::pi = float!(2;Wide(#0,#1);3.d4a349a4342...)


/*
          0 	1 	2 	3 	4 	5 	6 	7 	8 	9 	A 	B 	C 	D 	E 	F
U+037x 	Ͱ 	ͱ 	Ͳ 	ͳ 	ʹ 	͵ 	Ͷ 	ͷ 			ͺ 	ͻ 	ͼ 	ͽ 	; 	Ϳ
U+038x 					΄ 	΅ 	Ά 	· 	Έ 	Ή 	Ί 		Ό 		Ύ 	Ώ
U+039x 	ΐ 	Α 	Β 	Γ 	Δ 	Ε 	Ζ 	Η 	Θ 	Ι 	Κ 	Λ 	Μ 	Ν 	Ξ 	Ο
U+03Ax 	Π 	Ρ 		Σ 	Τ 	Υ 	Φ 	Χ 	Ψ 	Ω 	Ϊ 	Ϋ 	ά 	έ 	ή 	ί
U+03Bx 	ΰ 	α 	β 	γ 	δ 	ε 	ζ 	η 	θ 	ι 	κ 	λ 	μ 	ν 	ξ 	ο
U+03Cx 	π 	ρ 	ς 	σ 	τ 	υ 	φ 	χ 	ψ 	ω 	ϊ 	ϋ 	ό 	ύ 	ώ 	Ϗ
U+03Dx 	ϐ 	ϑ 	ϒ 	ϓ 	ϔ 	ϕ 	ϖ 	ϗ 	Ϙ 	ϙ 	Ϛ 	ϛ 	Ϝ 	ϝ 	Ϟ 	ϟ
U+03Ex 	Ϡ 	ϡ 	Ϣ 	ϣ 	Ϥ 	ϥ 	Ϧ 	ϧ 	Ϩ 	ϩ 	Ϫ 	ϫ 	Ϭ 	ϭ 	Ϯ 	ϯ
U+03Fx 	ϰ 	ϱ 	ϲ 	ϳ 	ϴ 	ϵ 	϶ 	Ϸ 	ϸ 	Ϲ 	Ϻ 	ϻ 	ϼ 	Ͻ 	Ͼ 	Ͽ


  	    0 	1 	2 	3 	4 	5 	6 	7 	8 	9 	A 	B 	C 	D 	E 	F
U+040x 	Ѐ 	Ё 	Ђ 	Ѓ 	Є 	Ѕ 	І 	Ї 	Ј 	Љ 	Њ 	Ћ 	Ќ 	Ѝ 	Ў 	Џ
U+041x 	А 	Б 	В 	Г 	Д 	Е 	Ж 	З 	И 	Й 	К 	Л 	М 	Н 	О 	П
U+042x 	Р 	С 	Т 	У 	Ф 	Х 	Ц 	Ч 	Ш 	Щ 	Ъ 	Ы 	Ь 	Э 	Ю 	Я
U+043x 	а 	б 	в 	г 	д 	е 	ж 	з 	и 	й 	к 	л 	м 	н 	о 	п
U+044x 	р 	с 	т 	у 	ф 	х 	ц 	ч 	ш 	щ 	ъ 	ы 	ь 	э 	ю 	я
U+045x 	ѐ 	ё 	ђ 	ѓ 	є 	ѕ 	і 	ї 	ј 	љ 	њ 	ћ 	ќ 	ѝ 	ў 	џ
U+046x 	Ѡ 	ѡ 	Ѣ 	ѣ 	Ѥ 	ѥ 	Ѧ 	ѧ 	Ѩ 	ѩ 	Ѫ 	ѫ 	Ѭ 	ѭ 	Ѯ 	ѯ
U+047x 	Ѱ 	ѱ 	Ѳ 	ѳ 	Ѵ 	ѵ 	Ѷ 	ѷ 	Ѹ 	ѹ 	Ѻ 	ѻ 	Ѽ 	ѽ 	Ѿ 	ѿ
U+048x 	Ҁ 	ҁ 	҂ 	◌҃ 	◌҄ 	◌҅ 	◌҆ 	◌҇ 	◌҈ 	◌҉ 	Ҋ 	ҋ 	Ҍ 	ҍ 	Ҏ 	ҏ
U+049x 	Ґ 	ґ 	Ғ 	ғ 	Ҕ 	ҕ 	Җ 	җ 	Ҙ 	ҙ 	Қ 	қ 	Ҝ 	ҝ 	Ҟ 	ҟ
U+04Ax 	Ҡ 	ҡ 	Ң 	ң 	Ҥ 	ҥ 	Ҧ 	ҧ 	Ҩ 	ҩ 	Ҫ 	ҫ 	Ҭ 	ҭ 	Ү 	ү
U+04Bx 	Ұ 	ұ 	Ҳ 	ҳ 	Ҵ 	ҵ 	Ҷ 	ҷ 	Ҹ 	ҹ 	Һ 	һ 	Ҽ 	ҽ 	Ҿ 	ҿ
U+04Cx 	Ӏ 	Ӂ 	ӂ 	Ӄ 	ӄ 	Ӆ 	ӆ 	Ӈ 	ӈ 	Ӊ 	ӊ 	Ӌ 	ӌ 	Ӎ 	ӎ 	ӏ
U+04Dx 	Ӑ 	ӑ 	Ӓ 	ӓ 	Ӕ 	ӕ 	Ӗ 	ӗ 	Ә 	ә 	Ӛ 	ӛ 	Ӝ 	ӝ 	Ӟ 	ӟ
U+04Ex 	Ӡ 	ӡ 	Ӣ 	ӣ 	Ӥ 	ӥ 	Ӧ 	ӧ 	Ө 	ө 	Ӫ 	ӫ 	Ӭ 	ӭ 	Ӯ 	ӯ
U+04Fx 	Ӱ 	ӱ 	Ӳ 	ӳ 	Ӵ 	ӵ 	Ӷ 	ӷ 	Ӹ 	ӹ 	Ӻ 	ӻ 	Ӽ 	ӽ 	Ӿ 	ӿ

  	    0 	1 	2 	3 	4 	5 	6 	7 	8 	9 	A 	B 	C 	D 	E 	F
U+10Ax 	Ⴀ 	Ⴁ 	Ⴂ 	Ⴃ 	Ⴄ 	Ⴅ 	Ⴆ 	Ⴇ 	Ⴈ 	Ⴉ 	Ⴊ 	Ⴋ 	Ⴌ 	Ⴍ 	Ⴎ 	Ⴏ
U+10Bx 	Ⴐ 	Ⴑ 	Ⴒ 	Ⴓ 	Ⴔ 	Ⴕ 	Ⴖ 	Ⴗ 	Ⴘ 	Ⴙ 	Ⴚ 	Ⴛ 	Ⴜ 	Ⴝ 	Ⴞ 	Ⴟ
U+10Cx 	Ⴠ 	Ⴡ 	Ⴢ 	Ⴣ 	Ⴤ 	Ⴥ 		Ⴧ 						Ⴭ 		
U+10Dx 	ა 	ბ 	გ 	დ 	ე 	ვ 	ზ 	თ 	ი 	კ 	ლ 	მ 	ნ 	ო 	პ 	ჟ
U+10Ex 	რ 	ს 	ტ 	უ 	ფ 	ქ 	ღ 	ყ 	შ 	ჩ 	ც 	ძ 	წ 	ჭ 	ხ 	ჯ
U+10Fx 	ჰ 	ჱ 	ჲ 	ჳ 	ჴ 	ჵ 	ჶ 	ჷ 	ჸ 	ჹ 	ჺ 	჻ 	ჼ 	ჽ 	ჾ 	ჿ

  	    0 	1 	2 	3 	4 	5 	6 	7 	8 	9 	A 	B 	C 	D 	E 	F
U+16Ax 	ᚠ 	ᚡ 	ᚢ 	ᚣ 	ᚤ 	ᚥ 	ᚦ 	ᚧ 	ᚨ 	ᚩ 	ᚪ 	ᚫ 	ᚬ 	ᚭ 	ᚮ 	ᚯ
U+16Bx 	ᚰ 	ᚱ 	ᚲ 	ᚳ 	ᚴ 	ᚵ 	ᚶ 	ᚷ 	ᚸ 	ᚹ 	ᚺ 	ᚻ 	ᚼ 	ᚽ 	ᚾ 	ᚿ
U+16Cx 	ᛀ 	ᛁ 	ᛂ 	ᛃ 	ᛄ 	ᛅ 	ᛆ 	ᛇ 	ᛈ 	ᛉ 	ᛊ 	ᛋ 	ᛌ 	ᛍ 	ᛎ 	ᛏ
U+16Dx 	ᛐ 	ᛑ 	ᛒ 	ᛓ 	ᛔ 	ᛕ 	ᛖ 	ᛗ 	ᛘ 	ᛙ 	ᛚ 	ᛛ 	ᛜ 	ᛝ 	ᛞ 	ᛟ
U+16Ex 	ᛠ 	ᛡ 	ᛢ 	ᛣ 	ᛤ 	ᛥ 	ᛦ 	ᛧ 	ᛨ 	ᛩ 	ᛪ 	᛫ 	᛬ 	᛭ 	ᛮ 	ᛯ
U+16Fx 	ᛰ 	ᛱ 	ᛲ 	ᛳ 	ᛴ 	ᛵ 	ᛶ 	ᛷ 	ᛸ

  	0 	1 	2 	3 	4 	5 	6 	7 	8 	9 	A 	B 	C 	D 	E 	F
U+210x 	℀ 	℁ 	ℂ 	℃ 	℄ 	℅ 	℆ 	ℇ 	℈ 	℉ 	ℊ 	ℋ 	ℌ 	ℍ 	ℎ 	ℏ
U+211x 	ℐ 	ℑ 	ℒ 	ℓ 	℔ 	ℕ 	№ 	℗ 	℘ 	ℙ 	ℚ 	ℛ 	ℜ 	ℝ 	℞ 	℟
U+212x 	℠ 	℡ 	™ 	℣ 	ℤ 	℥ 	Ω 	℧ 	ℨ 	℩ 	K 	Å 	ℬ 	ℭ 	℮ 	ℯ
U+213x 	ℰ 	ℱ 	Ⅎ 	ℳ 	ℴ 	ℵ 	ℶ 	ℷ 	ℸ 	ℹ 	℺ 	℻ 	ℼ 	ℽ 	ℾ 	ℿ
U+214x 	⅀ 	⅁ 	⅂ 	⅃ 	⅄ 	ⅅ 	ⅆ 	ⅇ 	ⅈ 	ⅉ 	⅊ 	⅋ 	⅌ 	⅍ 	ⅎ 	⅏

 	0 	1 	2 	3 	4 	5 	6 	7 	8 	9 	A 	B 	C 	D 	E 	F
U+008x 	XXX 	XXX 	BPH 	NBH 	 IND 	NEL 	SSA 	ESA 	HTS 	HTJ 	VTS 	PLD 	PLU 	 RI   	SS2 	SS3
U+009x 	DCS 	PU1 	PU2 	STS 	CCH 	 MW  	SPA 	EPA 	SOS 	XXX 	SCI  	CSI  	 ST  	OSC 	 PM  	APC
U+00Ax 	NBSP 	¡ 	¢ 	£ 	¤ 	¥ 	¦ 	§ 	¨ 	© 	ª 	« 	¬ 	SHY 	® 	¯
U+00Bx 	° 	± 	² 	³ 	´ 	µ 	¶ 	· 	¸ 	¹ 	º 	» 	¼ 	½ 	¾ 	¿
U+00Cx 	À 	Á 	Â 	Ã 	Ä 	Å 	Æ 	Ç 	È 	É 	Ê 	Ë 	Ì 	Í 	Î 	Ï
U+00Dx 	Ð 	Ñ 	Ò 	Ó 	Ô 	Õ 	Ö 	× 	Ø 	Ù 	Ú 	Û 	Ü 	Ý 	Þ 	ß
U+00Ex 	à 	á 	â 	ã 	ä 	å 	æ 	ç 	è 	é 	ê 	ë 	ì 	í 	î 	ï
U+00Fx 	ð 	ñ 	ò 	ó 	ô 	õ 	ö 	÷ 	ø 	ù 	ú 	û 	ü 	ý 	þ 	ÿ

  	0 	1 	2 	3 	4 	5 	6 	7 	8 	9 	A 	B 	C 	D 	E 	F
U+010x 	Ā 	ā 	Ă 	ă 	Ą 	ą 	Ć 	ć 	Ĉ 	ĉ 	Ċ 	ċ 	Č 	č 	Ď 	ď
U+011x 	Đ 	đ 	Ē 	ē 	Ĕ 	ĕ 	Ė 	ė 	Ę 	ę 	Ě 	ě 	Ĝ 	ĝ 	Ğ 	ğ
U+012x 	Ġ 	ġ 	Ģ 	ģ 	Ĥ 	ĥ 	Ħ 	ħ 	Ĩ 	ĩ 	Ī 	ī 	Ĭ 	ĭ 	Į 	į
U+013x 	İ 	ı 	Ĳ 	ĳ 	Ĵ 	ĵ 	Ķ 	ķ 	ĸ 	Ĺ 	ĺ 	Ļ 	ļ 	Ľ 	ľ 	Ŀ
U+014x 	ŀ 	Ł 	ł 	Ń 	ń 	Ņ 	ņ 	Ň 	ň 	ŉ 	Ŋ 	ŋ 	Ō 	ō 	Ŏ 	ŏ
U+015x 	Ő 	ő 	Œ 	œ 	Ŕ 	ŕ 	Ŗ 	ŗ 	Ř 	ř 	Ś 	ś 	Ŝ 	ŝ 	Ş 	ş
U+016x 	Š 	š 	Ţ 	ţ 	Ť 	ť 	Ŧ 	ŧ 	Ũ 	ũ 	Ū 	ū 	Ŭ 	ŭ 	Ů 	ů
U+017x 	Ű 	ű 	Ų 	ų 	Ŵ 	ŵ 	Ŷ 	ŷ 	Ÿ 	Ź 	ź 	Ż 	ż 	Ž 	ž 	ſ
*/

macro_rules! time {
  ($val:expr) => {
    let beg = std::time::Instant::now();
    match $val {
      tmp => {
        let end = std::time::Instant::now();
        let time = (end - beg);
        println!(
          "[{}:{}] `{}' took {:?}",
          std::file!(),
          std::line!(),
          std::stringify!($val),
          time
          );
        tmp
      }
    }
  };
  ($($val:expr),+ $(,)?) => {
    ($(time!($val)),+,)
  };
}

// from https://gist.github.com/justanotherdot
/*
If you look, I've also included a pattern to pass the number of times you want 
the benchmark to run. I default to ten runs instead of one-hundred here as code 
under inspection may take a long time to run under one-hundred times. You will 
notice that I'm using nanoseconds for everything, which lets me compute a 
proper mean average without rounding.
*/
/*
macro_rules! bench {
    ($val:expr) => {
        {
            let mut mean = 0;
            let times = 10;
            for _ in 0..times {
                let beg = std::time::Instant::now();
                match $val {
                    _ => {
                        let end = std::time::Instant::now();
                        mean += (end - beg).as_nanos();
                    }
                } 
            }
            mean /= times;
            eprintln!("[{}:{}] `{}' took {} ns after {} runs", std::file!(), std::line!(), std::stringify!($val), mean, times);
            $val
        }
    };
    ($val:expr, $times:expr) => {
        {
            let mut mean = 0;
            for _ in 0..$times {
                let beg = std::time::Instant::now();
                match $val {
                    _ => {
                        let end = std::time::Instant::now();
                        mean += (end - beg).as_nanos();
                    }
                } 
            }
            mean /= $times;
            eprintln!("[{}:{}] `{}' took {} ns after {} runs", std::file!(), std::line!(), std::stringify!($val), mean, $times);
            $val
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(bench!($val)),+,)
    };    
}
*/

mod agm;
mod airy;
mod algorithm;
mod api;
mod basic;
mod bessel;
mod complex;
mod data;
mod dawson;
mod debye;
mod dual;
mod ellint;
mod erf;
mod exp;
mod expint;
mod f16;
mod gamma;
mod hypergeom;
mod integration;
mod jacobi;
mod kahan;
mod lambert;
mod log;
mod numbers;
mod orthopoly;
mod poly;
mod real;
mod sievert;
mod solve;
mod theta;
mod traits;
mod trig;
mod util;
mod wide;
mod zeta;

use std::str::FromStr;
use std::time::Instant;

//use crate::erf::{*};
//use crate::kahan::{*};
//use crate::num::complex::{Complex};
use crate::algorithm::*;
use crate::bessel::*;
use crate::complex::*;
use crate::dawson::*;
use crate::dual::*;
use crate::ellint::*;
use crate::exp::*;
use crate::gamma::*;
use crate::integration::Integrator;
use crate::log::*;
use crate::numbers::*;
use crate::orthopoly::chebyshev_t::*;
use crate::orthopoly::*;
use crate::poly::*;
use crate::real::*;
use crate::theta::*;
use crate::traits::*;
use crate::trig::*;

fn rel(ex: f64, ap: f64) -> f64 {
  if ex == ap {
    return -17.0;
  }
  ((ex - ap).abs() / (1e-20 + ex.abs())).ln() / 10.0_f64.ln()
}

// literate programming?〚 〛

/*
extern crate plotlib;
fn doplots() {
  let lo = -10.0;
  let hi = 10.0;
  let f1 = plotlib::repr::Plot::from_function(
      |x|(rel(x.exp(),exp_cf(r64(x)).0)), lo, hi)
    .line_style(plotlib::style::LineStyle::new().colour("black"));
  let f2 = plotlib::repr::Plot::from_function(
      |x|(rel(x.exp(),sf_exp(x))), lo, hi)
    .line_style(plotlib::style::LineStyle::new().colour("red"));
  let f3 = plotlib::repr::Plot::from_function(
      |x|(rel(x.exp(),exp__powserk(x, 1.0))), lo, hi)
    .line_style(plotlib::style::LineStyle::new().colour("blue"));
  let v = plotlib::view::ContinuousView::new().add(f3).add(f2).add(f1).
    y_range(-18.0, -8.0);
  plotlib::page::Page::single(&v).save("plot1.svg").expect("saving svg");
}
*/
/*
extern crate plotters;
use plotters::prelude::*;
fn make_plot<C:Color>(fs:&[(C, &str, Vec<(f64,f64)>)], filename:&str, caption:&str,
    xrange:(f64,f64), yrange:(f64,f64))
  -> Result<(),Box<dyn std::error::Error>>
{
    //let root = BitMapBackend::new("0.png", (1280, 960)).into_drawing_area();
    //let root = SVGBackend::new(filename, (640, 480)).into_drawing_area();
    let root = SVGBackend::new(filename, (960, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(xrange.0..xrange.1, yrange.0..yrange.1)?;
    chart.configure_mesh().draw()?;
    for (color,label,pts) in fs.iter() {
      chart.draw_series(LineSeries::new((*pts).iter().cloned(), &color,))?
        .label(*label);
        //.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }
    /*
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    */
    Ok(())
}
*/
/*
fn doplots() -> Result<(),Box<dyn std::error::Error>> {
    //let root = BitMapBackend::new("0.png", (1280, 960)).into_drawing_area();
    let root = SVGBackend::new("0.svg", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=e^x", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f64..10f64, -18f64..-8f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
            (-1000..=1000).map(|x| x as f64/100.0).map(|x|
            (x, rel(x.exp(),exp__powserk(x,1.0)))
            ),
            &GREEN,
        ))?
        .label("power series")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    chart.draw_series(LineSeries::new(
            (-1000..=1000).map(|x| x as f64/100.0).map(|x|
            (x, rel(x.exp(),exp_cf(r64(x)).0))
            ),
            &RED,
        ))?
        .label("continued fraction")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart.draw_series(LineSeries::new(
            (-1000..=1000).map(|x| x as f64/100.0).map(|x|
            (x, rel(x.exp(),sf_exp(x)))
            ),
            &BLUE,
        ))?
        .label("range-reduction + power series")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
*/

fn main() {
  ::simple_logger::SimpleLogger::new().init().unwrap();

  if false {
    let ch: ChebyshevT<r64> = orthopoly::chebyshev_t::ChebyshevT::<r64>::new();
    for i in 0..10 {
      println!("{:?}", ch.coeffs(i).iter().map(|x| x.0).collect::<Vec<_>>());
    }
    for i in 0..10 {
      println!("{}", ch.poly(i));
    }
    println!("{} {}", ch.poly(3).value(ι(0.3)), ch.value(3, ι(0.3)));
    println!("{} {}", ch.poly(37).value(ι(0.1)), ch.value(37, ι(0.1)));
  }
  /*
    //if true { doplots(); }
    let ch : ChebyshevT<r64> = orthopoly::chebyshev_t::ChebyshevT::<r64>::new();
    if true {
      let fxrange = (-1.0,1.0);
      let num_points = 200; // 5000;
      let dx = (fxrange.1 - fxrange.0) / ((num_points - 1) as f64);
      let xrange = (-1.01,1.01);
      let yrange = (-1.05,1.05);
      make_plot(
        &(0..11)
          .map(|n|
            (BLACK, "T",
              (0..num_points).map(|ix| (ix as f64)*dx + fxrange.0)
                .map(|x|(x, ch.value(n,r64(x)).0)).collect::<Vec<_>>()))
          .collect::<Vec<_>>(),
        "chebyshev_t.svg", "Chebyshev T", xrange, yrange).unwrap();
      /*
      make_plot(&[
        (GREEN, "T1",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(1,r64(x)).0)).collect::<Vec<_>>()),
        (BLUE, "T2",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(2,r64(x)).0)).collect::<Vec<_>>()),
        (RED, "T3",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(3,r64(x)).0)).collect::<Vec<_>>()),
        (CYAN, "T4",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(4,r64(x)).0)).collect::<Vec<_>>()),
        (MAGENTA, "T5",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(5,r64(x)).0)).collect::<Vec<_>>()),
        (BLACK, "T11",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(11,r64(x)).0)).collect::<Vec<_>>()),
        ], "chebyshev_t.svg", "Chebyshev T", xrange, yrange).unwrap();
        */
    }
  */

  if false {
    let cc = c64 { re: ι(1), im: ι(1) };
    println!("cc={}", cc);
    println!("cc^2={}", cc * cc);
    println!("cc/2={}", cc / 2);
    println!("{}", cc / 2.0);
    println!("{}", cc / r64(2.0));
    println!("{}  {}", cc.arg(), 3.1415926535897932384626 / 4.0);
    println!("{}  {}", (cc.sqr()).arg(), 3.1415926535897932384626 / 2.0);
    println!("μcc={}", μ(cc));
    println!("{}", μ(cc.sqr()));
    println!("|cc|={}", abs(cc));
    println!("|cc^2|={}", abs(cc.sqr()));
    println!("(cc*cc).sqrt()={}", (cc * cc).sqrt());
    println!("{}", (cc * cc * cc).cbrt());
    println!("{}", exp::impls::exp_power_series(cc, 0));
    println!("{} {}", (1.0_f64.exp() * 1.0_f64.cos()), (1.0_f64.exp() * 1.0_f64.sin()));
    println!("{}", erf::impls::erf_series(cc));
  }

  if false {
    let mut p = Poly(vec![ι(1), ι(0), ι(3), ι(-4), ι(6), ι(0): r64]);
    println!("{}", p);
    println!("{:?}", p);
    p.reduce();
    println!("{}", p);
    println!("{:?}", p);
    let mut p = Poly(vec![ι(0), ι(0): r64]);
    println!("{}", p);
    println!("{:?}", p);
    p.reduce();
    println!("{}", p);
    println!("{:?}", p);
    let p = Poly(vec![ι(1), ι(1): r64]);
    println!("p={}", p);
    println!("p*p={}", &p * &p);
    println!("p*p*p={}", &(&p * &p) * &p);

    println!("-----");
    let mut p = Poly(vec![ι(1), ι(0), ι(3), ι(-4), ι(6), ι(0): r64]);
    println!("{}", p);
    for _ in 0..6 {
      p = p.diff();
      println!("{}", p);
    }
    println!("-----");
    let p = Poly(vec![ι(1), ι(0), ι(3): r64]);
    println!("p={}", p);
    println!("p(0)={}", p.value(ι(0)));
    println!("p(1)={}", p.value(ι(1)));
    println!("p(2)={}", p.value(ι(2)));
  }

  if false {
    println!("-----");
    println!("Exp:");
    let x = r64(1.0);
    println!("exact: {}", r64(x.0.exp()));
    println!("e:ps:  {}", exp::impls::exp_power_series(x, 0));
    println!("e:cf:  {}", exp::impls::exp_continued_fraction(x));
    println!("e:RR:  {:?}", exp::impls::range_reduce_ln2(x * 2));

    println!("---");
    let terms = (1..).scan(ι(1): r64, |s, n| {
      let o = *s;
      *s *= x / n;
      Some(o)
    });
    let terms = cum_sums(terms);
    let terms = terms.scan(ι(0): r64, |s, t| {
      if *s == t {
        None
      } else {
        *s = t;
        Some(t)
      }
    });
    println!("{:.16e}", terms.last().unwrap().0);
    //for t in terms.take(100) { println!("{:.16e}", t.0); }

    println!("Log1p:");
    let x = r64(0.10);
    println!("l1p:na {:.16e}", (x.0 + 1.0).ln());
    println!("l1p:ps {:.16e}", log::impls::ln1p_power_series(x).0);
    println!("l1p:xx {:.16e}", log::sf_ln_1p_real(x.0));
    println!("l1p:cf {:.16e}", log::impls::ln1p_contfrac(x).0);
    println!("l1p:mp {:.16e}", log::impls::sf_ln_1p_macroseries(x.0));
    //println!("ksum: {:.16e}", [1.0_f64,1e-12,-1.0,1e-22].iter().ksum():f64);

    println!("---");
    for x in cum_prods((1..).map(|n| r64(n as f64))).take(10) {
      print!("{:?}", x);
    }
    println!();
    for x in cum_sums((0..).map(|n| r64(n as f64))).take(10) {
      print!("{:?}", x);
    }
    println!();
  }

  if false {
    println!("-----");
    println!("Debye:");
    println!("{}  {}", debye::impls::debye_series_1(1, r64(0.1)), debye::impls::debye_scaled_series_1(1, r64(0.1)));
    println!("{}  {}", debye::impls::debye_series_1(1, r64(1.0)), debye::impls::debye_scaled_series_1(1, r64(1.0)));
    println!("{}  {}", debye::impls::debye_series_1(2, r64(1.0)), debye::impls::debye_scaled_series_1(2, r64(1.0)));
    println!("{}  {}", debye::impls::debye_series_1(1, r64(10.0)), debye::impls::debye_scaled_series_1(1, r64(10.0)));
    println!("{}  {}", debye::impls::debye_series_1(2, r64(2.0)), debye::impls::debye_scaled_series_1(2, r64(2.0)));
  }

  if false {
    println!("-----");
    println!("Bessel:");
    for n in 0..=5 {
      let x = r64(1.0);
      println!(
        "J_{}({}) = {:.16e}  {:.16e}  {:.16e}",
        n,
        x,
        bessel::impls::bessel_j_series(ι(n), x).0,
        bessel::impls::bessel_j_asymp_z(ι(n), x).0,
        bessel::impls::bessel_j_recur_back(31, n as isize, x).0
      );
    }
    for n in 0..=5 {
      let x = r64(10.0);
      println!(
        "J_{}({}) = {:.16e}  {:.16e}  {:.16e}",
        n,
        x,
        bessel::impls::bessel_j_series(ι(n), x).0,
        bessel::impls::bessel_j_asymp_z(ι(n), x).0,
        bessel::impls::bessel_j_recur_back(51, n as isize, x).0
      );
    }
    for n in 0..=5 {
      let x = r64(100.0);
      println!(
        "J_{}({}) = {:.16e}  {:.16e}  {:.16e}",
        n,
        x,
        bessel::impls::bessel_j_series(ι(n), x).0,
        bessel::impls::bessel_j_asymp_z(ι(n), x).0,
        bessel::impls::bessel_j_recur_back(151, n as isize, x).0
      );
    }
    for n in 0..=5 {
      let x = r64(250.0);
      println!(
        "J_{}({}) = {:.16e}  {:.16e}  {:.16e}",
        n,
        x,
        bessel::impls::bessel_j_series(ι(n), x).0,
        bessel::impls::bessel_j_asymp_z(ι(n), x).0,
        bessel::impls::bessel_j_recur_back(301, n as isize, x).0
      );
    }
    for n in 0..=5 {
      let x = c64 { re: ι(2), im: ι(1) };
      println!(
        "J_{}({}) = {:.16}  {:.16}  {:.16}",
        n,
        x,
        bessel::impls::bessel_j_series(ι(n), x),
        bessel::impls::bessel_j_asymp_z(ι(n), x),
        bessel::impls::bessel_j_recur_back(31, n as isize, x)
      );
    }

    {
      let x = c64 { re: ι(13), im: ι(0) };
      for n in 0..=5 {
        println!("-");
        for m in (3..=25).step_by(3) {
          println!("J_{}({})[{:2}] = {:.16}", n, x, m + n, bessel::impls::bessel_j_recur_back(m + n, n as isize, x));
        }
      }
    }
  }

  if false {
    let j3n: [(f64, f64); 21] = [
      (0.0, 0.0),
      (1.0000000000000000000, 0.019563353982668405919),
      (2.0000000000000000000, 0.12894324947440205110),
      (3.0000000000000000000, 0.30906272225525164362),
      (4.0000000000000000000, 0.43017147387562194036),
      (5.0000000000000000000, 0.36483123061366699446),
      (6.0000000000000000000, 0.11476838482077529636),
      (7.0000000000000000000, -0.16755558799533423603),
      (8.0000000000000000000, -0.29113220706595224938),
      (9.0000000000000000000, -0.18093519033665684004),
      (10.000000000000000000, 0.058379379305186812343),
      (11.000000000000000000, 0.22734803305806741747),
      (12.000000000000000000, 0.19513693953109267731),
      (13.000000000000000000, 0.0033198169704070507954),
      (14.000000000000000000, -0.17680940686509600251),
      (15.000000000000000000, -0.19401825782012263456),
      (16.000000000000000000, -0.043847495425981134212),
      (17.000000000000000000, 0.13493057304919323175),
      (18.000000000000000000, 0.18632099329078039410),
      (19.000000000000000000, 0.072489661438052575226),
      (20.000000000000000000, -0.098901394560449675613),
    ];
    println!("-----");
    println!("Bessel:");
    for &(x, j3x) in &j3n {
      let n = 3;
      let x = r64(x);
      let myj = sf_bessel_j(n, x);
      println!("J_{}({}) = {:.16e}  {:.16e}  err={}", n, x, myj.0, j3x, util::relerr(ι(j3x), myj));
    }
  }

  if false {
    println!("-----");
    println!("Dawson:");
    println!("{:.16e}", dawson::impls::dawson_contfrac(r64(1.0)).0);
    println!("{:.16e}", dawson::impls::dawson_contfrac2(r64(1.0)).0);
    println!("{:.16e}", dawson::impls::dawson_series(r64(1.0)).0);
    println!("{:.16e}", dawson::impls::dawson_rybicki(r64(1.0)).0);
  }

  if false {
    println!("-----");
    println!("Erf:");
    println!("{:.16e}  {:.16e}", erf::impls::erf_series(r64(1.0)).0, 1.0 - erf::impls::erf_series(r64(1.0)).0);
    println!("{:.16e}  {:.16e}", 1.0 - erf::impls::erfc_contfrac(r64(1.0)).0, erf::impls::erfc_contfrac(r64(1.0)).0);
    println!("{:.16e}  {:.16e}", 1.0 - erf::impls::erfc_contfrac2(r64(1.0)).0, erf::impls::erfc_contfrac2(r64(1.0)).0);
  }

  if false {
    let scale = 0.25;
    println!("-----");
    {
      let mut t = (0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = ((n % 1000) as f64 / 1000.0) * scale;
        t += log::impls::sf_ln_1p_macroseries(x);
      }
      let en = Instant::now();
      println!("{}\t{}", en.duration_since(st).as_micros(), t);
    }
    {
      let mut t = (0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = ((n % 1000) as f64 / 1000.0) * scale;
        t += log::sf_ln_1p_real(x);
      }
      let en = Instant::now();
      println!("{}\t{}", en.duration_since(st).as_micros(), t);
    }
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = r64((n % 1000) as f64 / 1000.0) * scale;
        t += log::impls::ln1p_power_series(x);
      }
      let en = Instant::now();
      println!("{}\t{}", en.duration_since(st).as_micros(), t.0);
    }
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = r64((n % 1000) as f64 / 1000.0) * scale;
        t += log::impls::ln1p_contfrac(x);
      }
      let en = Instant::now();
      println!("{}\t{}", en.duration_since(st).as_micros(), t.0);
    }
  }
  if false {
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = r64((n % 1000) as f64 / 1000.0);
        t += exp::impls::exp_power_series(x, 0);
      }
      let en = Instant::now();
      println!("{}\t{}", en.duration_since(st).as_micros(), t.0);
    }
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = r64((n % 1000) as f64 / 1000.0);
        t += exp::impls::exp_power_series_(x, 0);
      }
      let en = Instant::now();
      println!("{}\t{}", en.duration_since(st).as_micros(), t.0);
    }
  }

  if false {
    println!("=====");
    println!("{}", sf_factorial_approx(4));
    //println!("{} {:.16e} {}", 3.0, gamma_asympt(3.0), sf_factorial_exact(2));
    //println!("{} {:.16e} {}", 13.0, gamma_asympt(13.0), sf_factorial_exact(12));
    //println!("{} {:.16e} {}", 20.0, gamma_asympt(20.0), sf_factorial_exact(19));
    //println!("{} {:.16e} {}", 21.0, gamma_asympt(21.0), sf_factorial_exact(20));
    //println!("{} {:.16e} {}", 51.0, gamma_asympt(51.0), sf_factorial_exact(50));
    //println!("-----");
    println!("{} {:.16e} {}", 0.5, gamma::impls::gamma_spouge(11, r64(0.5)).0, 3.1415926535897932384626433_f64.sqrt());
    println!("{} {:.16e} {}", 3.0, gamma::impls::gamma_spouge(11, r64(3.0)).0, sf_factorial_approx(2));
    println!("{} {:.16e} {}", 13.0, gamma::impls::gamma_spouge(11, r64(13.0)).0, sf_factorial_approx(12));
    println!("{} {:.16e} {}", 40.0, gamma::impls::gamma_spouge(11, r64(40.0)).0, sf_factorial_approx(39));
    //println!("{} {:.16e} {}", 100.0, gamma::impls::gamma_spouge(11,r64(100.0)).0, sf_factorial_exact(99));
    println!("-----");
    println!(
      "{} {:.16e} {}",
      0.5,
      gamma::impls::lngamma_lanczos_7(r64(0.5)).exp().0,
      3.1415926535897932384626433_f64.sqrt()
    );
    println!("{} {:.16e} {}", 3.0, gamma::impls::lngamma_lanczos_7(r64(3.0)).exp().0, sf_factorial_approx(2));
    println!("{} {:.16e} {}", 13.0, gamma::impls::lngamma_lanczos_7(r64(13.0)).exp().0, sf_factorial_approx(12));
    println!("{} {:.16e} {}", 40.0, gamma::impls::lngamma_lanczos_7(r64(40.0)).exp().0, sf_factorial_approx(39));
    //println!("{} {:.16e} {}", 100.0, gamma::impls::lngamma_lanczos_7(r64(100.0)).exp().0, sf_factorial_exact(99));
    let z = c64::rect(ι(0), ι(1));
    println!("z = {}  1/z={}", z, ι(1): c64 / z);
    let z = c64::rect(ι(2), ι(0));
    println!("z = {}  1/z={}", z, ι(1): c64 / z);
    let z = c64::rect(ι(1), ι(1));
    println!("z = {}  1/z={}", z, ι(1): c64 / z);
    let z = c64::rect(ι(3), ι(1));
    println!("z = {}", z);
    println!("1/z = {}", ι(1): c64 / z);
    println!("log(z) = {:.16e}", sf_log(z));
    println!("exp(z) = {:e}", sf_exp(z));
    println!("exp(log(z)) = {}", sf_exp(sf_log(z)));
    println!("log(exp(z)) = {}", sf_log(sf_exp(z)));
    println!("lngamma(z) = {}", gamma::impls::lngamma_lanczos_7(z));
    println!("gamma(z) = {}", gamma::impls::lngamma_lanczos_7(z).exp());
    println!("gamma(z) = {}", gamma::impls::lngamma_lanczos_15(z).exp());
    let z = c64::rect(ι(1), ι(1));
    println!("z = {}", z);
    println!("gamma(z) = {}", gamma::impls::lngamma_lanczos_15(z).exp());
    println!("-----");
    println!(
      "{} {:.16e} {}",
      0.5,
      gamma::impls::lngamma_lanczos_15(r64(0.5)).exp().0,
      3.1415926535897932384626433_f64.sqrt()
    );
    println!("{} {:.16e} {}", 3.0, gamma::impls::lngamma_lanczos_15(r64(3.0)).exp().0, sf_factorial_approx(2));
    println!("{} {:.16e} {}", 13.0, gamma::impls::lngamma_lanczos_15(r64(13.0)).exp().0, sf_factorial_approx(12));
    println!("{} {:.16e} {}", 40.0, gamma::impls::lngamma_lanczos_15(r64(40.0)).exp().0, sf_factorial_approx(39));
    //println!("{} {:.16e} {}", 100.0, gamma::impls::lngamma_lanczos_15(r64(100.0)).exp().0, sf_factorial_exact(99));
  }

  if false {
    let pz = 0.0_f64;
    let mz = -0.0_f64;
    println!("{} {}", pz, mz);
    println!("{} {}", pz + mz, mz + pz);
    println!("{} {}", 1.0 * 0.0, -1.0 * 0.0);
    println!("{} {}", pz.ln(), mz.ln());
    let x = r64(3.15);
    println!("{}", x);
    println!("{:.16}", x);
  }

  // wide
  let qq = wide::Wide::new(11.0, 0.0) / 10.0;
  println!("11/10:{}", qq);
  if false {
    let q_pi = wide::Wide::from_str("3.14159265358979323846264338327950288419716939937510");
    println!("{:?}", q_pi);
    println!("{}", q_pi.unwrap());
    let q_eulergamma = wide::Wide::from_str("0.57721566490153286060651209008240243104215933593992");
    println!("{} {:?}", q_eulergamma.unwrap(), q_eulergamma);
    let q_ln2 = wide::Wide::from_str("0.69314718055994530941723212145817656807").unwrap();
    {
      let mut dsum = 1.0;
      let mut dt = 1.0;
      let dln2 = q_ln2.hi();
      let mut qsum = wide::Wide::new(1.0, 0.0);
      let mut t = wide::Wide::new(1.0, 0.0);
      for i in 1..28 {
        dt = dt * dln2 / (i as f64);
        dsum += dt;
        t = t * q_ln2 / (i as f64);
        qsum += t;
        println!("{:4}  {}  {}", i, qsum, dsum);
      }
    }

    println!("{:?}", q_ln2);
    println!("-----");
    let x = wide::Wide::new(1.0, 0.0);
    let y = wide::Wide::new(0.0, 0.1);
    println!("{}", x);
    println!("{}", y);
    println!("{}", wide::Wide::new(1.0, 0.1));
    println!("{}", x + y);
    println!("11/10:{}", wide::Wide::new(11.0, 0.0) / 10.0);
    println!("{:?}", (x + y) * (x + y));
    println!("{}", (x + y) * (x + y));
    println!("{:?}", (x * y) + (x * y));
    println!("{}", (x * y) + (x * y));
    println!("{:?}", (x + y) * 10.0);
    println!("{:?}", wide::Wide::new(1.0, 0.0) / 10.0);
    println!("{:?}", (wide::Wide::new(1.0, 0.0) / 10.0) * 10.0);
    println!("{:?}", wide::Wide::new(1.0, 0.1).scale2(3));
    println!("-----");
    let mut z = wide::Wide::new(1.0, 0.0);
    z /= 10.0;
    println!("{}  {:?}", z, z);
    println!("{}", wide::Wide::new(0.1, 0.0));
  }
  /*

  if false {
    println!("-----");
    println!("{}", sf_exp(0.25));
    println!("{}", (0.25_f64).exp());
    println!("{}", sf_exp(1.00));
    println!("{}", (1.00_f64).exp());
    println!("{}", sf_exp(5.00));
    println!("{}", (5.00_f64).exp());
    println!("-----");
    println!("{:e}", sf_exp(Complex::new(0.00,3.1415926535897932384626/2.0)));
    println!("{}", sf_exp(Complex::new(0.25,0.25)));
    println!("{}", (Complex::new(0.25,0.25)).exp());
    println!("{}", sf_exp(Complex::new(-2.5,2.5)));
    println!("{}", (Complex::new(-2.5,2.5)).exp());
    println!("{}", sf_exp(Complex::new(-22.5,12.5)));
    println!("{}", (Complex::new(-22.5,12.5)).exp());
    println!("-----");
    println!("{}", sf_exp(1.0/256.0)-1.0);
    println!("{}", sf_exp_m1(1.0/256.0));
    println!("{}", (1.0/256.0_f64).exp_m1());
    println!("-----");
    println!("{}", sf_ln(1.0 + 1.0/16.0));
    println!("{}", sf_ln_1p(1.0/16.0));
    println!("{}", (1.0_f64 + 1.0/16.0).ln());
    println!("-----");
    for n in 0..5 {
      println!("  {}", power_i(3.0,n));
    }
  }

  if false {
    println!("-----");
    let x = 2.0_f64;
    println!("{:e}", f64::EPSILON);
    println!("{:.16e}", x.exp());
    println!("{:.16e}  {}", sf_exp(x), rel(x.exp(),sf_exp(x)));
    println!("{:.16e}  {}", exp__powser(x,1.0), rel(x.exp(),exp__powser(x,1.0)));
    println!("{:.16e}  {}", exp__powser2(x,1.0), rel(x.exp(),exp__powser2(x,1.0)));
    println!("{:.16e}  {}", exp__powserk(x,1.0), rel(x.exp(),exp__powserk(x,1.0)));

    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += (x + ((n%13) as f64)/26.0).exp(); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += sf_exp((x + ((n%13) as f64)/26.0)); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += exp__powser((x + ((n%13) as f64)/26.0),1.0); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += exp__powser2((x + ((n%13) as f64)/26.0),1.0); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
    let mut t = 0.0;
    let st = Instant::now();
    for n in 0..1000000 { t += exp__powserk((x + ((n%13) as f64)/26.0),1.0); }
    let en = Instant::now();
    println!("{}\t{}", en.duration_since(st).as_micros(), t);
  }
  if false {
    for i in 0..=10 {
      println!("{} {}", 3*i, sf_factorial_exact(3*i));
    }
    for i in 0..=10 {
      println!("{} {}", 10*i, sf_fibonacci_number_exact(10*i));
    }
    println!("{} {}", 1000, sf_fibonacci_number_exact(1000));

    for n in 0..=5 {
      print!("{} : ", n);
      for k in 0..=n { print!("{}  ", sf_binomial_exact(n,k)); }
      println!();
    }

    println!("=====");
    for i in 0..=30 { print!("{}  ", sf_bernoulli_number_exact(i)); }
    println!();
    println!("=====");
    for i in 0..=10 { print!("{}  ", sf_harmonic_number_exact(i)); }
    println!();
    for i in 0..=10 { print!("{}  ", sf_tangent_number_exact(i)); }
    println!();
    for i in 0..=10 { print!("{}  ", sf_genocchi_number_exact(i)); }
    println!();
  }

  if false {
    println!("=====");
    for i in 0..=10 { print!("{}  ", sf_bernoulli_number_exact(i)); }
    println!();
    for i in 0..=10 { print!("{}  ", sf_bernoulli_number_approx(i)); }
    println!();
  }

  if false {
    for x in &vec![-9.0, -1.0, 0.0, 1.0, 5.0, 13.0] {
      let x = *x;
      println!("{}  {:.16e}  {:.16e}  {:.16e}  {:.16e}",
        x, dawson_contfrac(x), dawson_contfrac2(x), dawson_seres(x), dawson_rybicki(x));
    }
    for z in &vec![Complex::new(0.0,1.0), Complex::new(1.0,1.0), Complex::new(5.0,5.0)] {
      let z = *z;
      println!("{}  {:.16e}  {:.16e}  {:.16e}  {:.16e}",
        z, dawson_contfrac(z), dawson_contfrac2(z), dawson_seres(z), dawson_rybicki(z));
    }
  }

  if false {
    for x in &vec![-2.0, -1.0, 0.0, 0.5, 1.0, 3.0] {
      let x = *x;
      println!("{}  {:.16e}", x, erf_series(x));
    }
    for z in &vec![Complex::new(0.0,1.0), Complex::new(1.0,1.0), Complex::new(5.0,5.0)] {
      let z = *z;
      println!("{}  {:.16e}", z, erf_series(z));
    }
  }

  if false {
    let terms = (1..10).scan(1.0_f64,|s,n|{*s*=2.0/(ι(n):f64);Some(*s)});
    for t in terms { print!("  {}", t); } println!();
    println!("sum:  {:.16e}",
      ((1..10).scan(1.0_f64,|s,n|{*s*=2.0/(ι(n):f64);Some(*s)})).sum():f64);
    println!("ksum: {:.16e}",
      ((1..10).scan(1.0_f64,|s,n|{*s*=2.0/(ι(n):f64);Some(*s)})).ksum():f64);
    println!("sum:  {:.16e}", [1.0_f64,1e-12,-1.0,1e-22].iter().sum():f64);
    //println!("ksum: {:.16e}", [1.0_f64,1e-12,-1.0,1e-22].iter().ksum():f64);
    println!("ksum: {:.16e}", [1.0_f64,1e-12,-1.0,1e-22].iter().cloned().ksum():f64);
    let mut k = Kahan::default();
    for &x in &[1.0_f64,1e-12,-1.0,1e-22] {
      k += x;
    }
    println!("{:.16e}", k.0);
  }

  if false {
    //let x = 2 + 3*r(1.0) + 0.5 + 3 + (3.5 % 1.0);
    let x = 2 - 2*(r64(1.0) + 0.5)*3;
    println!("{:?}", x);
    println!("{:?}", eps(r64(1.0)));
    println!("{:?}", eps2(r64(1.0)));
    println!("{:?}", dss(r64(1.0)));
  }
  //println!("{:e}", {let x:f64 = 2.5_f64 + ι(3);x});
  if false {
    println!("{:?}", eps2(r64(1.0)));
    println!("{:?}", erf_series(1.0));
    println!("{:?}", erf_ss(r64(1.0)));
  }
  if false {
    println!("-----");
    for &x in &[30.0, 8.0, 0.5, 0.01, 1e-8] {
      println!("{:.16e} {:.16e}", (1.0_f64+x).ln(), ln_1p_cf(r64(x)).0);
    }
    println!("--");
    for &x in &[-1.0, 0.1, 1.0, 9.0_f64] {
      println!("{} {:.16e} {:.16e} {:.16e}",
        x, (x).exp(), exp_cf(r64(x)).0, eps2(r64(x)).0);
    }
    println!("--");
    println!("{:.16e} {:.16e}",
      (1.0+5.0_f64.sqrt())*0.5,
      contfrac(r64(1.0), (1..).map(|_|(r64(1.0),r64(1.0))),1e-12).0);
  }

  */

  if (false) {
    println!("-----");
    println!("Zeta:");
    for ix in 2..=20 {
      let x = ι(ix): r64 / 2 + 1;
      println!(
        "{}  {}  {}",
        x,
        zeta::impls::zeta_series_em9(x, r64::epsilon),
        zeta::impls::zeta_m1_series_em9(x, r64::epsilon)
      );
    }

    let z: c64 = c64::rect(r64(3.0), r64(4.0));
    println!("{}  {}", z, zeta::impls::zeta_series_em9(z, r64::epsilon.sqr()));
    let z: c64 = c64::rect(r64(3.0), r64(-4.0));
    println!("{}  {}", z, zeta::impls::zeta_series_em9(z, r64::epsilon.sqr()));
    let z: c64 = c64::rect(r64(4.5), r64(0.5));
    println!("{}  {}", z, zeta::impls::zeta_series_em9(z, r64::epsilon.sqr()));
    let z: c64 = c64::rect(r64(4.5), r64(-0.5));
    println!("{}  {}", z, zeta::impls::zeta_series_em9(z, r64::epsilon.sqr()));

    /*
    for n in 2..=10 {
      println!("{}\n\t{:.16e}\n\t{:.16e}\n\t{:.16e}\n\t{:.16e}\n\t{:.16e}", n,
        zeta_m1_directseries(r64(n as f64)).0,
        zeta_directseries(r64(n as f64)).0,
        zeta_directseries2(r64(n as f64)).0,
        zeta_directseries_em1(r64(n as f64)).0,
        zeta_directseries_em2(r64(n as f64)).0
      );
    }
    println!("{:.16e} {:.16e}",
      zeta_directseries_em1(r64(2.0)).0,
      (zeta_directseries_em1(r64(4.0)).0*2.5).sqrt()
    );
    for n in 2..=100 {
      print!("  {:.16e}", zeta_directseries_em2(r64(n as f64)).0);
      if zeta_directseries_em2(r64(n as f64)).0 == 1.0 {
        println!("  {}",n);break; }
    }
    for n in 5..=100 {
      print!("  {:.16e}", zeta_m1_directseries(r64(n as f64)).0);
    }
    println!();
    */
  }

  if false {
    let p = Poly(vec![1.0, 0.0, 1.0_f64]);
    let q = Poly(vec![1.0, 1.0_f64]);
    println!("p={} q={}", p, q);
    println!("p(q)={}", p.substitute(q));

    let p = Poly(vec![1, 0, 1_isize]);
    let q = Poly(vec![1, 1_isize]);
    println!("p={} q={}", p, q);
    println!("p(q)={}", p.substitute(q));
  }

  if false {
    println!("{}", sf_gamma(r64(4.5)));
    println!("{}", sf_gamma(r64(-4.5)));
    println!("{}", sf_gamma(c64 { re: r64(4.0), im: r64(1.0) }));
    println!("{}", sf_gamma(c64 { re: r64(-4.0), im: r64(1.0) }));

    for i in (5..=100).step_by(5) {
      let x: r64 = ι(i);
      println!("Γ({}) = {:.16e}  {:.16e}", x, gamma::impls::gamma_asympt(x).0, sf_factorial_approx((i - 1) as usize));
    }
  }

  if false {
    for i in 0..=20 {
      println!("{:2} {:+.16e}  {:+.16e}", i, sf_bernoulli_number_approx(i), sf_bern2(i));
    }
  }

  if false {
    for i in 1..16 {
      println!(
        "{:.16e}  {:.16e}  | {:.16e}",
        integration::Trapezoidal::new(r64(0.0), r64(1.0), 1 << i).integrate(trig::sf_cos).0,
        integration::TanhRule::new(r64(0.0), r64(1.0), 1 << i).integrate(trig::sf_cos).0,
        trig::sf_sin(r64(1.0)).0
      );
    }
  }

  if false {
    println!("-----\nSievert:");
    let th = r64::PI/4;
    println!("{:.16e}  {:.16e}",
      sievert::impls::sievert_integrate(th, r64(1.0)).0,
      sievert::impls::sievert_asympt(th, r64(1.0)).0);
    println!("{:.16e}  {:.16e}",
      sievert::impls::sievert_integrate(th, r64(5.0)).0,
      sievert::impls::sievert_asympt(th, r64(5.0)).0);
    println!("{:.16e}  {:.16e}",
      sievert::impls::sievert_integrate(th, r64(25.0)).0,
      sievert::impls::sievert_asympt(th, r64(25.0)).0);
  }

  if false {
    println!("----------");
    for n in [0, 57, -10] {
      dbg!(n);
      dbg!(n.is_nan());
      dbg!(n.is_infinite());
      dbg!(n.is_finite());
      dbg!(n.is_zero());
      dbg!(n.is_negzero());
      dbg!(n.is_real());
      dbg!(n.is_imag());
      dbg!(n.is_negreal());
      dbg!(n.is_posreal());
      dbg!(n.is_nonnegreal());
      dbg!(n.is_nonposreal());
      dbg!(n.is_int());
      dbg!(n.is_posint());
      dbg!(n.is_negint());
      dbg!(n.is_nonposint());
      dbg!(n.is_nonnegint());
      dbg!(n.is_evenint());
      dbg!(n.is_oddint());
    }
    println!("----------");
    for n in [0.0, 57.0, -10.0, -0.0, 0.5, 1e20-1.0] {
      dbg!(n);
      dbg!(n.is_nan());
      dbg!(n.is_infinite());
      dbg!(n.is_finite());
      dbg!(n.is_zero());
      dbg!(n.is_negzero());
      dbg!(n.is_real());
      dbg!(n.is_imag());
      dbg!(n.is_negreal());
      dbg!(n.is_posreal());
      dbg!(n.is_nonnegreal());
      dbg!(n.is_nonposreal());
      dbg!(n.is_int());
      dbg!(n.is_posint());
      dbg!(n.is_negint());
      dbg!(n.is_nonposint());
      dbg!(n.is_nonnegint());
      dbg!(n.is_evenint());
      dbg!(n.is_oddint());
    }
  }
  println!("{:.4e}  {:.4E}  {:.4}", r64::PI, r64::PI, r64::PI);
  println!("{:b}  {:x}  {:X}", r64::PI, r64::PI, r64::PI);

  if false {
    println!("-----");
    println!("Airy:");
    println!("{:?}", airy::impls::airy_series(r64(0.5)));
    println!("{:?}", airy::impls::airy_series(r64(1.0)));
    println!("{:e} {:e}",
      airy::impls::airy_series(c64{re:r64(1.0),im:r64(1.0)}).0,
      airy::impls::airy_series(c64{re:r64(1.0),im:r64(1.0)}).1);
  }

  if false {
    println!("-----");
    let x = r64(1.0e-300);
    println!("{:.16e}  {:.16e}  {:.16e}", x.sqrt(), basic::sqrt_newton(x), basic::sqrt_halley(x));
    let x = r64(2.0);
    println!("{:.16e}  {:.16e}  {:.16e}", x.sqrt(), basic::sqrt_newton(x), basic::sqrt_halley(x));
    let x = r64(65.0);
    println!("{:.16e}  {:.16e}  {:.16e}", x.sqrt(), basic::sqrt_newton(x), basic::sqrt_halley(x));
    let x = r64(1.23451e20);
    println!("{:.16e}  {:.16e}  {:.16e}", x.sqrt(), basic::sqrt_newton(x), basic::sqrt_halley(x));
    let x = r64(1.2e300);
    println!("{:.16e}  {:.16e}  {:.16e}", x.sqrt(), basic::sqrt_newton(x), basic::sqrt_halley(x));
    let x = c64{re:r64(2.0),im:r64(2.0)};
    println!("{:.16e}  {:.16e}  {:.16e}", x.sqrt(), basic::sqrt_newton(x), basic::sqrt_halley(x));
    println!("-");
    let x = r64(2.0);
    println!("{:.16e}  {:.16e}", x.cbrt(), basic::cbrt_newton(x));
    let x = r64(64.0);
    println!("{:.16e}  {:.16e}", x.cbrt(), basic::cbrt_newton(x));
    let x = r64(1e20);
    println!("{:.16e}  {:.16e}", x.cbrt(), basic::cbrt_newton(x));
    let x = c64{re:r64(2.0),im:r64(2.0)};
    println!("{:.16e}  {:.16e}", x.cbrt(), basic::cbrt_newton(x));
    println!("-");
    let x = r64(2.0);
    println!("{:.16e}  {:.16e}", x.pow(r64(1.0/7.0)), basic::nthrt_newton(x, 7));
    let x = r64(64.0);
    println!("{:.16e}  {:.16e}", x.pow(r64(1.0/7.0)), basic::nthrt_newton(x, 7));
    let x = r64(1e20);
    println!("{:.16e}  {:.16e}", x.pow(r64(1.0/7.0)), basic::nthrt_newton(x, 7));
    let x = c64{re:r64(2.0),im:r64(2.0)};
    println!("{:.16e}  {:.16e}", x.pow(r64(1.0/7.0)), basic::nthrt_newton(x, 7));

  }

  if false {
    println!("{} {} {}",
      basic::lerp(r64(1.0), r64(3.0), r64(0.00)),
      basic::lerp(r64(1.0), r64(3.0), r64(0.25)),
      basic::lerp(r64(1.0), r64(3.0), r64(1.00)));
    println!("{} {} {}",
      basic::lerp(r64(1.0), r64(1.0+1e-12), r64(0.00)),
      basic::lerp(r64(1.0), r64(1.0+1e-12), r64(0.25)),
      basic::lerp(r64(1.0), r64(1.0+1e-12), r64(1.00)));
  }

  if false {
    let x = c64::rect(r64::PI, ι(2.5));
    let a = c64::rect(ι(1.0), ι(1.0));
    let b = c64::rect(ι(1.0), ι(-1.0));
    println!("x={}  a={}  b={}", x, a, b);
    println!("x%a={}", x % a);
    println!("x%b={}", x % b);
    println!("x%(1,i)={}", x % (c64{re:ι(1),im:ι(0)},c64{re:ι(0),im:ι(1)}));
    println!("x%(a,b)={}", x % (a,b));
    println!("x%(b,a)={}", x % (b,a));
    println!("x%(-a,-b)={}", x % (-a,-b));
    println!("x%(-b,-a)={}", x % (-b,-a));
  }

  if false {
    println!("-----");
    println!("Exp:");
    let x = r64(0.5);
    println!("{}", sf_exp(x));
    println!("{}", exp::impls::exp_power_series(x, 0));
    println!("{}", exp::impls::exp_minimax(x));
    println!("{}", exp::impls::fastexp(x));
    println!("{}", exp::impls::exp_minimax2(x));
    println!("{}", exp::impls::fastexp2(x));

    let scale = 0.5;
    let base = 0.5;
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = base + ((n % 1000) as f64 / 1000.0) * scale;
        t += sf_exp(r64(x));
      }
      let en = Instant::now();
      println!("sf_exp(x): {:6}\t{}", en.duration_since(st).as_micros(), t);
    }
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = base + ((n % 1000) as f64 / 1000.0) * scale;
        t += exp::impls::exp_minimax(r64(x));
      }
      let en = Instant::now();
      println!("minmax(x): {:6}\t{}", en.duration_since(st).as_micros(), t);
    }
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = base + ((n % 1000) as f64 / 1000.0) * scale;
        t += exp::impls::fastexp(r64(x));
      }
      let en = Instant::now();
      println!("fastex(x): {:6}\t{}", en.duration_since(st).as_micros(), t);
    }
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = base + ((n % 1000) as f64 / 1000.0) * scale;
        t += exp::impls::exp_power_series(r64(x), 0);
      }
      let en = Instant::now();
      println!("powser(x): {:6}\t{}", en.duration_since(st).as_micros(), t);
    }
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = base + ((n % 1000) as f64 / 1000.0) * scale;
        t += exp::impls::fastexp2(r64(x));
      }
      let en = Instant::now();
      println!("faste2(x): {:6}\t{}", en.duration_since(st).as_micros(), t);
    }
    {
      let mut t = r64(0.0);
      let st = Instant::now();
      for n in 0..1000000 {
        let x = base + ((n % 1000) as f64 / 1000.0) * scale;
        t += exp::impls::exp_minimax2(r64(x));
      }
      let en = Instant::now();
      println!("minma2(x): {:6}\t{}", en.duration_since(st).as_micros(), t);
    }
  }

  if false {
    for n in 1..7 {
      for x in util::Grid::new(r64(1.0), r64(2.0), n) { print!("({})", x); }
      println!();
    }
  }

  if false {
    println!("{:?}", 7.5_f64.frexp());
    println!("{}", 7.5_f64.ilogb());
    println!("{}", 7.5_f64.next_up());
    println!("{}", 7.5_f64.next_dn());
    println!("{}", 7.5_f64.copysign(f64::neg_zero));
    println!("{}", f64::neg_zero);
    println!("{}", 7.5_f64.ldexp(1));
    println!("{}", 7.5_f64.ldexp(-1));
    println!("{}", -0.0);
  }

  if false {
    let a = f16::f16(16_u16<<10);
    println!("{:016b}  {} {}", a.0, a.to_f32(), a.to_f64());
    println!("{:016b}  {} {}", (-a).0, (-a).to_f32(), (-a).to_f64());
    println!("{:016b}", f16::f16::from_f32(a.to_f32()).0);
    let b = a*a;
    println!("{:016b}  {} {}", b.0, b.to_f32(), b.to_f64());
    let c = b*b;
    println!("{:016b}  {} {}", c.0, c.to_f32(), c.to_f64());
    let a = f16::f16(0x3333);
    println!("{:016b}  {} {}", a.0, a.to_f32(), a.to_f64());
    let b = a*a;
    println!("{:016b}  {} {} {}", b.0, b.to_f32(), b.to_f64(), a.to_f32()*a.to_f32());
    let c = b*b;
    println!("{:016b}  {} {} {}={:016b}", c.0, c.to_f32(), c.to_f64(),
      b.to_f32()*b.to_f32(), f16::f16::from_f32(b.to_f32()*b.to_f32()).0);
  }

  if false {
    println!("-----");
    println!("Lambert");
    let x = r64(-0.1);
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos(x));
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos_2(x));
    let x = r64(-0.2);
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos(x));
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos_2(x));
    let x = r64(-0.3);
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos(x));
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos_2(x));
    let x = r64(1.0);
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos(x));
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos_2(x));
    let x = r64(3.0);
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos(x));
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos_2(x));
    let x = r64(99.0);
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos(x));
    println!("W0({}) = {:.16e}", x, lambert::impls::real_branch_pos_2(x));
    println!("-");
    let x = r64(-0.3);
    println!("W-1({}) = {:.16e}", x, lambert::impls::real_branch_neg(x));
    println!("W-1({}) = {:.16e}", x, lambert::impls::real_branch_neg_2(x));
    let x = r64(-0.2);
    println!("W-1({}) = {:.16e}", x, lambert::impls::real_branch_neg(x));
    println!("W-1({}) = {:.16e}", x, lambert::impls::real_branch_neg_2(x));
    let x = r64(-0.1);
    println!("W-1({}) = {:.16e}", x, lambert::impls::real_branch_neg(x));
    println!("W-1({}) = {:.16e}", x, lambert::impls::real_branch_neg_2(x));
    let x = r64(-0.01);
    println!("W-1({}) = {:.16e}", x, lambert::impls::real_branch_neg(x));
    println!("W-1({}) = {:.16e}", x, lambert::impls::real_branch_neg_2(x));
  }
  if false {
    for i in 1..10 {
      println!("{:5}  {:.16e}  | {:.16e}", (1<<i)+1,
        integration::TanhRule::new(r64(0.0), r64(1.0), (1<<i)+1).integrate(trig::sf_cos).0,
        trig::sf_sin(r64(1.0)).0);
    }
  }
  if true {
    use crate::agm::*;
    println!("-----");
    println!("AGM:");
    println!("{:e}", agm::sf_agm(r64(24.0), r64(6.0)));
    println!("{:e}", agm::impls::impl_scalar(r64(1.0), sf_sqrt(ι(2))).recip());
    println!("{:e}", agm::impls::impl_scalar(ι(1), ι(2):r64).recip());
    println!("{:e}", agm::impls::impl_scalar(ι(1), c64::I));
    println!("{:e}", agm::sf_agm(r64(1.0), sf_sqrt(ι(2))));
    //println!("{:?}", agm::impls::impl_vec(r64(1.0), sf_sqrt(ι(2)), r64(1.0), None));
    //println!("{:?}", agm::impls::impl_vec(r64(1.0), sf_sqrt(ι(2)), r64(1.0), Some(r64(1.0))));
  }
  if true {
    println!("-----");
    println!("Elliptic:");
    println!("k(1.5) = {:e}", sf_ellint_k(r64(1.5)));
    println!("k(0.5) = {:e}", sf_ellint_k(r64(0.5)));
    println!("k(0.0) = {:e}", sf_ellint_k(r64(0.0)));
    println!("f = {:e}", ellint::impls::f_agm_method(r64::PI/2, r64(0.5)));
    println!("f = {:e}", ellint::impls::f_ascending_landen(r64::PI/2, r64(0.5)));
    println!("f = {:e}", ellint::impls::ell_f(r64::PI/2, r64(0.5)));
    println!("f = {:e}", ellint::impls::ell_f(r64::PI/4, r64(0.5)));
    //println!("k = {:e}", sf_ellint_k(c64{re:r64(0.5),im:r64(0.1)}));
  }
  if true {
    println!("-----");
    println!("Theta:");
    println!("th_1 = {:e}", sf_theta_1(r64(2.0), r64(1.0)/3));
    println!("th_2 = {:e}", sf_theta_2(r64(2.0), r64(1.0)/3));
    println!("th_3 = {:e}", sf_theta_3(r64(2.0), r64(1.0)/3));
    println!("th_3 = {:e}", theta::impls::theta_3_series_xform(r64(2.0), r64(1.0)/3));
    println!("th_4 = {:e}", sf_theta_4(r64(2.0), r64(1.0)/3));
    println!("th_1 = {:e}", sf_theta_1(r64(9.0), r64(1.0)/3));
    println!("th_2 = {:e}", sf_theta_2(r64(9.0), r64(1.0)/3));
    println!("th_3 = {:e}", sf_theta_3(r64(9.0), r64(1.0)/3));
    println!("th_3 = {:e}", theta::impls::theta_3_series_xform(r64(9.0), r64(1.0)/3));
    println!("th_4 = {:e}", sf_theta_4(r64(9.0), r64(1.0)/3));
    println!("th_1 = {:e}", sf_theta_1(r64(2.0), r64(1.0)/3));
    println!("th_1 = {:e}", sf_theta_1(r64(2.0)+r64::PI*2, r64(1.0)/3));
  }
  if false {
    println!("-----");
    println!("Dual:");
    let a = Dual::from(r64(3.0));
    let b = Dual{x:r64(1.0),dx:r64(1.0)};
    println!("{}", a);
    println!("{}", b);
    println!("{}", a*b);
    println!("{}", a*a);
    println!("{}", b*b);
    println!("{}", b*b*b);
    println!("{}", b*b*b*b);
    println!("{}", b/b);
    let bb = Dual{x:b,dx:b};
    println!("{}", bb);
    println!("{}", bb*bb);
    println!("{}", bb*bb*bb*bb);
  }
  if true {
    println!("-----");
    println!("Jacobi:");
    let z = r64(2.0);
    let k = r64(1.0)/3;
    println!(" = {:?}", jacobi::impls::jacobi_agm_general::<_,true,true,true>(z, k));
    let zeta = r64::PI*z/sf_ellint_k(k)/2;
    let q = sf_exp(-r64::PI*sf_ellint_kc(k)/sf_ellint_k(k));
    let t10 = sf_theta_1(ι(0),q);
    let t1z = sf_theta_1(zeta,q);
    let t20 = sf_theta_2(ι(0),q);
    let t2z = sf_theta_2(zeta,q);
    let t30 = sf_theta_3(ι(0),q);
    let t3z = sf_theta_3(zeta,q);
    let t40 = sf_theta_4(ι(0),q);
    let t4z = sf_theta_4(zeta,q);
    let cn = (t40*t2z)/(t20*t4z);
    let dn = (t40*t3z)/(t30*t4z);
    let sn = (t30*t1z)/(t20*t4z);
    println!("cn = {:e}", cn);
    println!("dn = {:e}", dn);
    println!("sn = {:e}", sn);
    println!("cn(0) {:e} {:e}", jacobi::impls::jacobi_agm_cn(z, r64(0.0)), sf_cos(z));
    println!("cn(1) {:e} {:e}", jacobi::impls::jacobi_agm_cn(z, r64(1.0)), sf_sech(z));
    println!("dn(0) {:e} {:e}", jacobi::impls::jacobi_agm_dn(z, r64(0.0)), r64(1.0));
    println!("dn(1) {:e} {:e}", jacobi::impls::jacobi_agm_dn(z, r64(1.0)), sf_sech(z));
    println!("sn(0) {:e} {:e}", jacobi::impls::jacobi_agm_sn(z, r64(0.0)), sf_sin(z));
    println!("sn(1) {:e} {:e}", jacobi::impls::jacobi_agm_sn(z, r64(1.0)), sf_tanh(z));

    println!("cn {:e}", jacobi::impls::jacobi_agm_cn(c64::I, ι(0.5)));
    println!("cn {:e}", jacobi::impls::jacobi_agm_cn(c64::I+1, ι(0.5)));
    /*
    let z = r64(2.0);
    for i in 0..=64 {
      let k = i/r64(64.0);
      println!("cn({:.5})={:.5}", k, jacobi::impls::jacobi_agm_cn(z, k));
    }
    for i in 0..=64 {
      let k = i/r64(64.0);
      println!("dn({:.5})={:.5}", k, jacobi::impls::jacobi_agm_dn(z, k));
    }
    for i in 0..=64 {
      let k = i/r64(64.0);
      println!("sn({:.5})={:.5}", k, jacobi::impls::jacobi_agm_sn(z, k));
    }
    */
  }
  //println!("{:e} {:e}", sf_tan(r64(1.0)), trig::impls::tan_contfrac(r64(1.0)));
  if true {
    println!("-----");
    println!("Hurwitz Zeta:");
    println!("{:e}", zeta::impls_hurwitz::hurwitz_series_em(r64(11.0),r64(2.25)));
    println!("{:e}", zeta::impls_hurwitz::hurwitz_series_em(r64(4.5),r64(0.25)));
    println!("{:e}", zeta::impls_hurwitz::hurwitz_series_em(ι(4.5),c64::I+r64(0.25)));
    println!("{:e}", zeta::impls_hurwitz::hurwitz_series_em(c64::I+r64(4.5),c64::I+r64(0.25)));
    println!("{:e}", zeta::impls_hurwitz::hurwitz_series_em(c64::I+r64(4.5),c64::I*0+r64(0.25)));
    println!("{:e}", zeta::impls_hurwitz::hurwitz_series_em(c64::I*3+r64(4.5),c64::I*3+r64(0.25)));
    println!("{:e}  {:e}", zeta::impls_hurwitz::hurwitz_series_em(r64(4.5),r64(1.00)), zeta::impls::zeta_series_em9(r64(4.5),r64::epsilon));
  }
  if true {
    println!("-----");
    println!("Elliptic:");
    println!("e(..) = {:e}", sf_ellint_e(r64(1.5)));
    println!("e(..) = {:e}", sf_ellint_e(r64(0.5)));
    println!("e(..) = {:e}", sf_ellint_e(r64(0.0)));
    println!("e'(..) = {:e}", sf_ellint_e_inc(r64(0.5), r64::PI/2));
    println!("e'(..) = {:e}", sf_ellint_e_inc(r64(0.5), r64::PI/4));
    println!("pi(..) = {:e}", sf_ellint_pi(r64(0.5), r64(0.25)));
    println!("pi'(..) = {:e}", sf_ellint_pi_inc(r64::PI/2, r64(0.5), r64(0.25)));
    println!("pi'(..) = {:e}", sf_ellint_pi_inc(r64::PI/4, r64(0.5), r64(0.25)));
    println!("rc(0.5,0.1) = {:e}", sf_ellint_rc(r64(0.5), r64(0.1)));
    println!("rc(0.1,0.5) = {:e}", sf_ellint_rc(r64(0.1), r64(0.5)));
    println!("rd(0.1,0.5,0.9) = {:e}", sf_ellint_rd(r64(0.1), r64(0.5), r64(0.9)));
    println!("rd(0.9,0.5,0.1) = {:e}", sf_ellint_rd(r64(0.9), r64(0.5), r64(0.1)));
    println!("rd(3,4,5) = {:e}", sf_ellint_rd(r64(3.0), r64(4.0), r64(5.0)));
    println!("rd(5,4,3) = {:e}", sf_ellint_rd(r64(5.0), r64(4.0), r64(3.0)));
    println!("rd(5,3,4) = {:e}", sf_ellint_rd(r64(5.0), r64(3.0), r64(4.0)));
    println!("rd(4,5,3) = {:e}", sf_ellint_rd(r64(4.0), r64(5.0), r64(3.0)));
    println!("rf(0.1,0.5,0.9) = {:e}", sf_ellint_rf(r64(0.1), r64(0.5), r64(0.9)));
    println!("rf(0.9,0.5,0.1) = {:e}", sf_ellint_rf(r64(0.9), r64(0.5), r64(0.1)));
    println!("rg(3,4,5) = {:e}", sf_ellint_rg(r64(3.0), r64(4.0), r64(5.0)));
    println!("rg(3,5,4) = {:e}", sf_ellint_rg(r64(3.0), r64(5.0), r64(4.0)));
    println!("rg(5,4,3) = {:e}", sf_ellint_rg(r64(5.0), r64(4.0), r64(3.0)));
    println!("rg(5,3,4) = {:e}", sf_ellint_rg(r64(5.0), r64(3.0), r64(4.0)));
    println!("rg(4,3,5) = {:e}", sf_ellint_rg(r64(4.0), r64(3.0), r64(5.0)));
    println!("rg(4,5,3) = {:e}", sf_ellint_rg(r64(4.0), r64(5.0), r64(3.0)));
    println!("rj(1,4,5,3) = {:e}", sf_ellint_rj(r64(1.0), r64(4.0), r64(5.0), r64(3.0)));
    println!("rj(4,1,5,3) = {:e}", sf_ellint_rj(r64(4.0), r64(1.0), r64(5.0), r64(3.0)));
    println!("rj(4,1,3,5) = {:e}", sf_ellint_rj(r64(4.0), r64(1.0), r64(3.0), r64(5.0)));
    println!("cel(1,1,1,1) = {:e}", sf_ellint_cel(r64(1.0),r64(1.0),r64(1.0),r64(1.0)));
    println!("el1(1,1) = {:e}", sf_ellint_el1(r64(1.0),r64(1.0)));
    println!("el2(1,1,1,1) = {:e}", sf_ellint_el2(r64(1.0),r64(1.0),r64(1.0),r64(1.0)));
    println!("el3(1,1,1) = {:e}", sf_ellint_el3(r64(1.0),r64(1.0),r64(1.0)));
    time!(std::hint::black_box(sf_ellint_el3(r64(1.0),r64(1.0),r64(1.0))));
    /*
    let mut x = r64::default();
    time!({for i in 1..25000000{x+=std::hint::black_box(sf_exp(r64(1.0+(i as f64/1e8))));};()});
    println!("{:e}", x);
    let mut x = f64::default();
    time!({for i in 1..25000000{x+=std::hint::black_box((1.0_f64+(i as f64/1e8)).exp());};()});
    println!("{:e}", x);
    let mut x = r64::default();
    time!({for i in 1..25000000{x+=std::hint::black_box(exp::impls::fastexp(r64(1.0+(i as f64/1e8))));};()});
    println!("{:e}", x);
    let mut x = r64::default();
    time!({for i in 1..25000000{x+=std::hint::black_box(exp::impls::fastexp2(r64(1.0+(i as f64/1e8))));};()});
    println!("{:e}", x);
    */
  }
  if true {
    //println!("i10={:e}", sf_bessel_spher_i1(0, r64(1e1)));
    dbg!(sf_bessel_spher_i1(0, r64(1e1)).0);
    dbg!(sf_bessel_spher_i1(1, r64(1e1)).0);
    dbg!(sf_bessel_spher_i1(2, r64(1e1)).0);
    dbg!(sf_bessel_spher_i1(3, r64(1e1)).0);
    dbg!(sf_bessel_spher_i1(7, r64(1e1)).0);
    dbg!(sf_bessel_spher_i1(8, r64(1e1)).0);
    dbg!(sf_bessel_spher_i1(0, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i1(1, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i1(2, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i1(3, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i1(7, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i1(8, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i2(0, r64(1e1)).0);
    dbg!(sf_bessel_spher_i2(1, r64(1e1)).0);
    dbg!(sf_bessel_spher_i2(2, r64(1e1)).0);
    dbg!(sf_bessel_spher_i2(3, r64(1e1)).0);
    dbg!(sf_bessel_spher_i2(7, r64(1e1)).0);
    dbg!(sf_bessel_spher_i2(8, r64(1e1)).0);
    dbg!(sf_bessel_spher_i2(0, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i2(1, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i2(2, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i2(3, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i2(7, r64(1e-1)).0);
    dbg!(sf_bessel_spher_i2(8, r64(1e-1)).0);
    println!("j0={:e}", sf_bessel_spher_j(0, r64(1e-0)));
    println!("j1={:e}", sf_bessel_spher_j(1, r64(1e-0)));
    println!("j2={:e}", sf_bessel_spher_j(2, r64(1e-0)));
    println!("j3={:e}", sf_bessel_spher_j(3, r64(1e-0)));
    println!("j7={:e}", sf_bessel_spher_j(7, r64(1e-0)));
    println!("j8={:e}", sf_bessel_spher_j(8, r64(1e-0)));
    println!("k0={:e}", sf_bessel_spher_k(0, r64(1.0)));
    println!("k1={:e}", sf_bessel_spher_k(1, r64(1.0)));
    println!("k2={:e}", sf_bessel_spher_k(2, r64(1.0)));
    println!("k3={:e}", sf_bessel_spher_k(3, r64(1.0)));
    println!("k7={:e}", sf_bessel_spher_k(7, r64(1.0)));
    println!("k8={:e}", sf_bessel_spher_k(8, r64(1.0)));
    println!("y0={:e}", sf_bessel_spher_y(0, r64(1.0)));
    println!("y1={:e}", sf_bessel_spher_y(1, r64(1.0)));
    println!("y2={:e}", sf_bessel_spher_y(2, r64(1.0)));
    println!("y3={:e}", sf_bessel_spher_y(3, r64(1.0)));
    println!("y7={:e}", sf_bessel_spher_y(7, r64(1.0)));
    println!("y8={:e}", sf_bessel_spher_y(8, r64(1.0)));
  }
  if true {
    println!("{:?}", sf_sqrt(c64::rect(r64(0.0),r64(1.0))));
    println!("{:?}", solve::solve_linear(r64(3.0), r64(2.0)));
    println!("{:?}", solve::solve_quadratic(r64(-1.0), r64(0.0), r64(1.0)));
    println!("{:?}", solve::solve_quadratic(c64::one, c64::zero, c64::one));
    println!("{:?}", solve::solve_cubic(r64(-6.0), r64(11.0), r64(-6.0), r64(1.0)));
    println!("{:?}", solve::solve_cubic(-c64::one, c64::one, c64::one, c64::one));
    println!("{:?}", solve::solve_quartic(r64(24.0), r64(-50.0), r64(35.0), r64(-10.0), r64(1.0)));
    println!("{:?}", solve::solve_quartic(c64::one, c64::one, c64::one, c64::one, c64::one));
  }
  if true {
    println!("ψ(-10.1)={:e}", sf_digamma(r64(-10.1)));
    println!("ψ(0.1)={:e}", sf_digamma(r64(0.1)));
    println!("ψ(2)={:e}", sf_digamma(r64(2.0)));
    println!("ψ(10.1)={:e}", sf_digamma(r64(10.1)));
    println!("ei(0.1)={:e}", expint::sf_expint_ei(r64(0.1)));
    println!("ei(1.0)={:e}", expint::sf_expint_ei(r64(1.0)));
    println!("ei(21.0)={:e}", expint::sf_expint_ei(r64(21.0)));
    println!("ei(51.0)={:e}", expint::sf_expint_ei(r64(51.0)));
    println!("en(0,0.1)={:e}", expint::sf_expint_en(0,r64(0.1)));
    println!("en(0,1.0)={:e}", expint::sf_expint_en(0,r64(1.0)));
    println!("en(0,3.0)={:e}", expint::sf_expint_en(0,r64(3.0)));
    println!("en(1,0.1)={:e}", expint::sf_expint_en(1,r64(0.1)));
    println!("en(1,1.0)={:e}", expint::sf_expint_en(1,r64(1.0)));
    println!("en(1,3.0)={:e}", expint::sf_expint_en(1,r64(3.0)));
    println!("en(2,0.1)={:e}", expint::sf_expint_en(2,r64(0.1)));
    println!("en(2,1.0)={:e}", expint::sf_expint_en(2,r64(1.0)));
    println!("en(2,3.0)={:e}", expint::sf_expint_en(2,r64(3.0)));
    println!("en(5,0.1)={:e}", expint::sf_expint_en(5,r64(0.1)));
    println!("en(5,1.0)={:e}", expint::sf_expint_en(5,r64(1.0)));
    println!("en(5,3.0)={:e}", expint::sf_expint_en(5,r64(3.0)));
    println!("ci(0.5)={:e}", expint::sf_cosint(r64(0.5)));
    println!("ci(2.5)={:e}", expint::sf_cosint(r64(2.5)));
    println!("ci(9.5)={:e}", expint::sf_cosint(r64(9.5)));
    println!("ci(19.5)={:e}", expint::sf_cosint(r64(19.5)));
    println!("ci(1+I)={:e}", expint::sf_cosint(c64{re:r64::one,im:r64::one}));
    println!("ci(-1+0I)={:e}", expint::sf_cosint(c64{re:-r64::one,im:r64::zero}));
    println!("si(-1)={:e}", expint::sf_sinint(-r64(1.0)));
    println!("si(0.5)={:e}", expint::sf_sinint(r64(0.5)));
    println!("si(2.5)={:e}", expint::sf_sinint(r64(2.5)));
    println!("si(9.5)={:e}", expint::sf_sinint(r64(9.5)));
    println!("si(19.5)={:e}", expint::sf_sinint(r64(19.5)));
    println!("si(1+I)={:e}", expint::sf_sinint(c64{re:r64::one,im:r64::one}));
    println!("si(-1+0I)={:e}", expint::sf_sinint(c64{re:-r64::one,im:r64::zero}));
    println!("si(9.5+I)={:e}", expint::sf_sinint(c64::I+r64(9.5)));
  }
  if true {
    let z = c64{re:r64(1.0), im:r64(1.0)};
    println!("j0({:e})={:e}", z, sf_bessel_spher_j(0, z));
    println!("j1({:e})={:e}", z, sf_bessel_spher_j(1, z));
    println!("j2({:e})={:e}", z, sf_bessel_spher_j(2, z));
    println!("j3({:e})={:e}", z, sf_bessel_spher_j(3, z));
    let z = -z;
    println!("j0({:e})={:e}", z, sf_bessel_spher_j(0, z));
    println!("j1({:e})={:e}", z, sf_bessel_spher_j(1, z));
    println!("j2({:e})={:e}", z, sf_bessel_spher_j(2, z));
    println!("j3({:e})={:e}", z, sf_bessel_spher_j(3, z));
    let z = c64::I;
    println!("j0({:e})={:e}", z, sf_bessel_spher_j(0, z));
    println!("j1({:e})={:e}", z, sf_bessel_spher_j(1, z));
    println!("j2({:e})={:e}", z, sf_bessel_spher_j(2, z));
    println!("j3({:e})={:e}", z, sf_bessel_spher_j(3, z));
    let z = c64::zero;
    println!("j0({:e})={:e}", z, sf_bessel_spher_j(0, z));
    println!("j1({:e})={:e}", z, sf_bessel_spher_j(1, z));
    println!("j2({:e})={:e}", z, sf_bessel_spher_j(2, z));
    println!("j3({:e})={:e}", z, sf_bessel_spher_j(3, z));
    let z = r64::zero;
    println!("j0({:e})={:e}", z, sf_bessel_spher_j(0, z));
    println!("j1({:e})={:e}", z, sf_bessel_spher_j(1, z));
    println!("j2({:e})={:e}", z, sf_bessel_spher_j(2, z));
    println!("j3({:e})={:e}", z, sf_bessel_spher_j(3, z));
  }
  if true {
    println!("{:e}", sf_exp_men(3, r64(1.0)));
    println!("{:e}", sf_exp_men(7, r64(1.0)));
    println!("{:e}", sf_exp_men(12, r64(1.0)));
  }
  if true {
    print!(":"); for x in 0..(-1) {print!("<{}>", x);} println!(":");
    print!(":"); for x in (0..(-1)).step_by(2) {print!("<{}>", x);} println!(":");
    println!("{:016X}", 1.0_f64.to_bits());
    let bytes = [
      0b11001001,0b00001111,0b11011010,0b10100010,
      0b00100001,0b01101000,0b11000010,0b00110100,
      0b11000100,0b11000110,0b01100010,0b10001011,
      0b10000000,0b11011100,0b00011100,0b11010001,
      0b00101001,0b00000010,0b01001110,0b00001000,];
    let (f1,n) = data::parse_bytes_f64(&bytes, 1, 0);
    let (f2,n) = data::parse_bytes_f64(&bytes, 1-(n as isize), n);
    println!("{:.30e} {:.30e}", f1, f2);
    println!("{}", wide::Wide::new(f1,f2));

    let spi = "3.243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be";
    let stb = data::string_to_bytes(&spi);
    println!("{:?}\n{:?}", spi, stb);
    for x in stb.0.iter() { print!("{:02X}", x); } println!();
    let (f1,n1) = data::parse_bytes_f64(&stb.0, stb.1 as isize - 1, 0);
    let (f2,n2) = data::parse_bytes_f64(&stb.0, stb.1 as isize - 1 - (n1 as isize), n1);
    println!("{:e} {}", f1, n1);
    println!("{:e} {}", f2, n2);

    println!("{:?}", data::parse_hex_f64s(spi));
    for x in data::parse_hex_f64s(spi) {print!(" {:e}", x);} println!();

    let sx = "3p4";
    println!("{:?}", data::parse_hex_f64s(sx));
    let sx = "3p-4";
    println!("{:?}", data::parse_hex_f64s(sx));
  }
  if true {
    println!("I31(1/2)={:e}", bessel::impls::bessel_i_series_int(31, r64(0.5)));
    println!("I31(1/2)={:e}", bessel::impls::bessel_i_series(ι(31), r64(0.5)));
    println!("I31(1/2)={:e}", bessel::impls::bessel_i_order_recur(31, r64(0.5)));
    println!("I9(1/2)={:e}", bessel::impls::bessel_i_series_int(9, r64(0.5)));
    println!("I9(1/2)={:e}", bessel::impls::bessel_i_series(ι(9), r64(0.5)));
    println!("I9(1/2)={:e}", bessel::impls::bessel_i_order_recur(9, r64(0.5)));
    println!("I1(1/2)={:e}", bessel::impls::bessel_i_series_int(1, r64(0.5)));
    println!("I1(1/2)={:e}", bessel::impls::bessel_i_series(ι(1), r64(0.5)));
    println!("I1(1/2)={:e}", bessel::impls::bessel_i_order_recur(1, r64(0.5)));
    println!("I1(31/2)={:e}", bessel::impls::bessel_i_series_int(1, r64(15.5)));
    println!("I1(31/2)={:e}", bessel::impls::bessel_i_series(ι(1), r64(15.5)));
    println!("I1(31/2)={:e}", bessel::impls::bessel_i_order_recur(1, r64(15.5)));
    println!("I1(401/2)={:e}", bessel::impls::bessel_i_series_int(1, r64(200.5)));
    println!("I1(401/2)={:e}", bessel::impls::bessel_i_series(ι(1), r64(200.5)));
    println!("I1(401/2)={:e}", bessel::impls::bessel_i_order_recur(1, r64(200.5)));
    println!("I1(-1/2)={:e}", bessel::impls::bessel_i_series_int(1, r64(-0.5)));
    println!("I1(-1/2)={:e}", bessel::impls::bessel_i_series(ι(1), r64(-0.5)));
    println!("I1(-1/2)={:e}", bessel::impls::bessel_i_order_recur(1, r64(-0.5)));
    println!("I1(-31/2)={:e}", bessel::impls::bessel_i_series_int(1, r64(-15.5)));
    println!("I1(-31/2)={:e}", bessel::impls::bessel_i_series(ι(1), r64(-15.5)));
    println!("I1(-31/2)={:e}", bessel::impls::bessel_i_order_recur(1, r64(-15.5)));
    println!("I1(401/2)={:e}", bessel::impls::bessel_i_series_int(1, r64(-200.5)));
    println!("I1(401/2)={:e}", bessel::impls::bessel_i_series(ι(1), r64(-200.5)));
    println!("I1(401/2)={:e}", bessel::impls::bessel_i_order_recur(1, r64(-200.5)));
    println!("Y1(1/2)={:e}", bessel::impls::bessel_y_series_int(1, r64(0.5)));
    println!("Y1(7.25)={:e}", bessel::impls::bessel_y_series_int(1, r64(7.25)));
    println!("Y4(1/2)={:e}", bessel::impls::bessel_y_series_int(4, r64(0.5)));
    println!("Y1(7.25)={:e}", bessel::impls::bessel_y_asymp_z(ι(1), r64(7.25)));
  }

  if true {
    println!("Γ(1,2)={:e}", sf_gamma_inc_co(ι(1), r64(2.0)));
    println!("Γ(7,2)={:e}", sf_gamma_inc_co(ι(7), r64(2.0)));
    println!("Γ(1,7)={:e}", sf_gamma_inc_co(ι(1), r64(7.0)));
    println!("γ(1,2)={:e}", sf_gamma_inc(ι(1), r64(2.0)));
    println!("γ(1,7)={:e}", sf_gamma_inc(ι(1), r64(7.0)));
    println!("γ(7,2)={:e}", sf_gamma_inc(ι(7), r64(2.0)));
    println!("P(2,3)={:e}", sf_gamma_inc_p(ι(2), r64(3.0)));
    println!("Q(2,3)={:e}", sf_gamma_inc_q(ι(2), r64(3.0)));
  }
}

