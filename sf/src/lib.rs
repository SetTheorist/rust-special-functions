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
#![feature(bigint_helper_methods)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(int_log)]
#![feature(trait_alias)]
#![feature(type_ascription)]
#![feature(unchecked_math)]
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

pub mod agm;
pub mod airy;
pub mod algorithm;
pub mod api;
pub mod basic;
pub mod bessel;
pub mod complex;
pub mod data;
pub mod dawson;
pub mod debye;
pub mod dual;
pub mod ellint;
pub mod erf;
pub mod exp;
pub mod expint;
pub mod f128;
pub mod f16;
pub mod farb;
pub mod float;
pub mod gamma;
pub mod hypergeom;
pub mod integration;
pub mod jacobi;
pub mod kahan;
pub mod lambert;
pub mod log;
pub mod numbers;
pub mod orthopoly;
pub mod pcf;
pub mod poly;
pub mod polylog;
pub mod real;
pub mod sievert;
pub mod solve;
pub mod theta;
pub mod traits;
pub mod trig;
pub mod twin;
pub mod util;
pub mod wide;
pub mod zeta;