use crate::traits::*;

use sf_hex_float::hexf;
use crate::wide::Wide;
macro_rules! Wide { ($x:tt) => { hexf!(:2:Wide:$x) } }

// Trapezoidal
// Erf / Tanh/Sinh ("stretched trapezoidal")
// Simpson's
// Gaussian &c.
// adaptive? (may not need for purposes of spec.fun. implementations)
//
// extrapolation / sequence acceleration?

// TODO: lots of other possibilities,
// including returning a lot more information...
// allow separate domain & range types?
pub trait Integrator<X> {
  fn domain(&self) -> (X, X);
  fn integrate<F>(&self, f: F) -> X
    where F: Fn(X) -> X;
}

////////////////////////////////////////////////////////////////////////////////

// use trapezoidal rule to integrate on interval [a,b]
// will actually evaluate f at n+1 points
// non-adaptive, simply computes the sum
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Trapezoidal<X> {
  a: X,
  b: X,
  h: X,
  n: isize,
}

impl<X:Value> Trapezoidal<X> {
  pub fn new(a:X, b:X, n:usize) -> Self {
    let n = (n as isize).max(1);
    let h = (b - a) / n;
    Trapezoidal { a, b, h, n }
  }
}

impl<X:Value> Integrator<X> for Trapezoidal<X> {
  fn domain(&self) -> (X, X) { (self.a, self.b) }
  fn integrate<F:Fn(X)->X>(&self, f: F) -> X {
    let mut sum: X = (f(self.a) + f(self.b)) / 2;
    for i in 1..self.n {
      sum += f(self.a + self.h * i);
    }
    sum * self.h
  }
}

////////////////////////////////////////////////////////////////////////////////

use crate::trig::{*};

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct TanhRule<X> {
  a: X,
  b: X,
  h: X,
  n: isize,
}

impl<X:Value> TanhRule<X> {
  pub fn new(a:X, b:X, n:usize) -> Self {
    let n = ((n/2) as isize).max(1);
    let h = X::PI/sf_sqrt(ι(2*n):X);
    TanhRule { a, b, h, n }
  }
}

// TODO: cleanup to allow separate domain & range
impl<X:Value+Trig+std::fmt::LowerExp> Integrator<X> for TanhRule<X> {
  fn domain(&self) -> (X, X) { (self.a, self.b) }
  fn integrate<F:Fn(X)->X>(&self, f: F) -> X {
    let mut sum = X::zero;
    for k in -self.n..(self.n+1) {
      let t = self.h * k;
      let xk = (self.b+self.a)/2 + (self.b-self.a)/2 * sf_tanh(t);
      let wk = (self.b - xk)*(xk - self.a)*2/(self.b - self.a);
      let fx = f(xk);
      sum += self.h * wk * fx;
    }
    sum
  }
}

////////////////////////////////////////////////////////////////////////////////

pub const GAUSS_LAGUERRE_23__MINUS16_XW : [(f64,f64); 23] = [
(0.049002324371591840306,0.22190661574048635538),
(0.29530892594874725178,0.32104976470008496016),
(0.7536138185285887951545,0.278749135218214369837),
(1.426134556419966046761,0.1767533394965254559499),
(2.316024117486417130275,0.085768604825166283443),
(3.42754777110858566473,0.0323649818240780976858),
(4.76620905785530189515,0.0095437317879520212865),
(6.338912250703311815804,0.00219727093568472542502),
(8.15417901904800412076,3.93148051086767951958e-4),
(10.22243803068820728583,5.42468622855624168575e-5),
(12.55641426652768159006,5.71056297226460570316e-6),
(15.1716583032377972468,4.52274883380842870632e-7),
(18.0872778846009670785,2.647475826439460948804e-8),
(21.32697136926332214368,1.11999146278270073486e-9),
(24.92052829583340470665,3.32778316600523246748e-11),
(28.90608392557213166864,6.69403188135392417644e-13),
(33.33365389681871113026,8.68586931031147065516e-15),
(38.27098189613715205865,6.80879193066501873168e-17),
(43.81391104513859513204,2.9392647035826944268e-19),
(50.10658055247607312484,6.08154888669877801016e-22),
(57.38634001743254160404,4.80417922537747243469e-25),
(66.10656201861103391749,9.420868501665319089e-29),
(77.43033332285853341849,1.55075461341823859578e-33),
];

pub const GAUSS_LAGUERRE_31__MINUS16_XW : [(f64,f64); 31] = [
(0.036522647328503509959,0.17586432444613712748),
(0.220015204475143469789,0.27073761778435636951),
(0.56105879494706826401,0.2639569283919817568983),
(1.060601768819224091469,0.198584895894565763191),
(1.71992834161345682058,0.1210259110847412642397),
(2.54074661704199241945,0.060869396028334219468),
(3.525219508135736550245,0.0254706998524582723923),
(4.675998408487434218708,0.008896965036390985626383),
(5.996264962691047116714,0.002595730985026274276425),
(7.48978319393148230507,6.31783691684456499381e-4),
(9.160964317142089903928,1.2795061355048080865e-4),
(11.01494719689333321795,2.148017742901447252543e-5),
(13.05769836962850538302,2.97453271841657528127e-6),
(15.29613690399742024734,3.37711217487728408601e-7),
(17.73829128807945658912,3.12058215642308271185e-8),
(20.39349828433968086645,2.32647022458181265871e-9),
(23.2726577269908978185,1.384951693641761203694e-10),
(26.38856328166026057054,6.5030071504899334265e-12),
(29.75633847390790131618,2.37349818522089644885e-13),
(33.39402196211773333139,6.61688485881977495e-15),
(37.32336994620830752938,1.37946242241157132683e-16),
(41.57098404046517325882,2.095524502368928855981e-18),
(46.1699442669051471296,2.24586941284345589486e-20),
(51.16225909554449008362,1.62999520688889001886e-22),
(56.60270492076548931907,7.5946764296094457962e-25),
(62.56517947799614396278,2.11406726349682512438e-27),
(69.15397796792722441543,3.1771582432328389273e-30),
(76.52577385703082299522,2.2153508451285478668e-33),
(84.9385629849746199209,5.5912112168394176093e-37),
(94.88569604030576919858,3.19008374061398753179e-41),
(107.63562348298177751,1.25165427667877407484e-46),
];

/* Mathematica: (though still requires manual editing, afterwards))

With[{alpha = -1/6, n = 41},
 hexify[x_] := Module[{s}, s = BaseForm[x, 16] // ToString // StringSplit;
   If[Length[s] == 2, s[[1]], s[[2]] <> "p" <> s[[1]]]];
 getx[r_] := x /. r;
 roots = Roots[LaguerreL[n, alpha, x] == 0, x] // N[#, 50] &;
 xs = Map[getx, ({ToRules[roots]} // Flatten)];
 wgt[xi_] := {xi, xi Gamma[n + alpha + 1]/(n! (n + 1)^2 LaguerreL[n + 1, alpha, xi]^2)};
 xws = Map[wgt, xs];
 hexxws = Map[("(Wide!(\"" <> hexify[#[[1]]] <> "\"),Wide!(\"" <>hexify[#[[2]]] <> "\"))") &, xws]
 ]
*/
pub const GAUSS_LAGUERRE_41__MINUS16_XW__WIDE: [(Wide,Wide); 41] = [
(Wide!("0.07179609aaa3342e7687d711c4cbe49eb57564f873"),Wide!("0.2413407b4354bcb7cb214f709cad2b3243015102")),
(Wide!("0.2ab6f7724cdbc34084b23255c71a54399c57b7dd53"),Wide!("0.3a085a70ef6d62bd72d6a62f356db6bad224eacf")),
(Wide!("0.6ce26ab430835a7dc2fc106ab77a5b518b2f7a55f"),Wide!("0.3d67d6abcc4c596ea35a23b23246293f9bf361d2")),
(Wide!("0.cdb6c59cbfae273b8bf794622cb7935d8eb10b591"),Wide!("0.3419f444b783ce0e205754dd8c65c8cff59c2892")),
(Wide!("1.4d58970b91b12036fade5ad55916dea165fa4b658"),Wide!("0.253d2a2023026a1f151fadcd419dfe5ef9bc1db2")),
(Wide!("1.ebf7ea49f0d148288b593897129da9339ed380b6b"),Wide!("0.16dd00cf9c7bc361dd5e31c21996863654d92a3a")),
(Wide!("2.a9d0f7a9185e75bf88ee9f4c2855c83acd7b01a08"),Wide!("0.0c2bdf76a8031ef5963938e104d8581e1cdcaea47")),
(Wide!("3.872cb29f800e80a0b36b0d56701f1eebc85d9998e"),Wide!("0.05a53a7ceb3d18451240297b449432e5e4b303001")),
(Wide!("4.84616c67d0580731865b7034a7af5f3a011a14c5"),Wide!("0.0249786b5e9e1e6a3df149fcb76e8b833cfe9595d")),
(Wide!("5.a1d3963523f4223c6f552713178ce378b164f3cb"),Wide!("0.00cf5ad68eb89cb269cc25bffd308055481358011")),
(Wide!("6.dff6a8e3bb6cc4df094c33f458abb56567fc6c19"),Wide!("0.0040349a17c2eed10b2fc5b2361b1063d30f19008e")),
(Wide!("8.3f4e37e32db9ab7921c72e8064028d6428b7fc3e"),Wide!("0.00115e75774f71f715ab0f5acba52ff428f0d6a739")),
(Wide!("9.c06f360d8523dba3c09b921690f49baa1ca087b8"),Wide!("0.0004198d77ce97e13f4e7a8c7e4849f99c8d753f4b7")),
(Wide!("b.640174992ed137d4c1665a74d85f3c6695ea63c4"),Wide!("0.0000d7c6eddbffd0509d5cd589947b14a446bfccadc")),
(Wide!("d.2ac1664314c5b45a62a2a804d831c467b8fbb9d5"),Wide!("0.0000268d14bff159da1268974499761c53281efa3438")),
(Wide!("f.158233485ad49c9ebbdd027a33b92882f8e70041"),Wide!("5.f81f7a11c349310bce144e0d969c0c6e32d3cd4p-6")),
(Wide!("11.25302df49602da63194126bb31931f5f235e0be4"),Wide!("c.c69840fb386d045be9ebc480a8b6c1d5e4e173dp-7")),
(Wide!("13.5ad3bba5186c0e4c658a629d293c40e5c22d9b5a"),Wide!("1.7891ec8e7c2627c5694acea1b28eae0511fa669p-7")),
(Wide!("15.b794cb7f9092cce233d48ecff91505bdf103ede0"),Wide!("2.529b4c045bf7b190af70b98b756d36754fc9264p-8")),
(Wide!("18.3cbefb2ff286b8ce6df5069f16a368d938dd06e9"),Wide!("3.20e80d0b08f13583eebb18f12e9dca124f78a87p-9")),
(Wide!("1a.ebc6937897294d6410f89b82924f6f0f0f36a4fc"),Wide!("3.934291272d99ecfc107f45e752575557cbd67e5p-10")),
(Wide!("1d.c64e932d42b5d8033c686b0ab5a76234a183ada8"),Wide!("3.71f2f1afbe73a802a41343044cc1764809a1bdcp-11")),
(Wide!("20.ce3010c0743b2783f7b87012a5e37510155763b2"),Wide!("2.c79be181346eeab368963099052f43892d9f745p-12")),
(Wide!("24.058352f9432e02f1eea85d6904c0613bdd9836ad"),Wide!("1.dcf550f72b9af12a7298b1490890819e3fe3f07p-13")),
(Wide!("27.6eab23f5cf5f07232ea3788d44152b6b06494fc2"),Wide!("1.07276768f269d122738cad47deafa43e5778d02p-14")),
(Wide!("2b.0c63135ba5e7059921af640c05626edfd2fbdf05"),Wide!("7.646862985d06c3c8ec490a810c47826c71c911ep-16")),
(Wide!("2e.e1d1a59bf7220fecbbb6698fd57893f569a49110"),Wide!("2.ac9220893ed1c555e316b7b6ef34c8b205694e1p-17")),
(Wide!("32.f29fdbbfaf39434b3f780468b0ffc0cede2434ec"),Wide!("c.48a327db79f1766f2fadb0fc8a685f26da74276p-19")),
(Wide!("37.431827a372e119d88263ee2f071a55a2afa63b87"),Wide!("2.c0d5c805d0a52ff286c13299b8c83fa9d9be2d2p-20")),
(Wide!("3b.d84fead42a22d0b756d8e2998e5724dd7cb342a4"),Wide!("7.8f477f142681919b1d7acfd446abf0b03853579p-22")),
(Wide!("40.b86050aa20fbe6cda6b2de4ad2a650a6d180901a"),Wide!("f.8655f24280272a590ae6239209368e793a4c93p-24")),
(Wide!("45.eab63339d6a7a2dd75da6c463ac43e90d62aa7f"),Wide!("1.72d3bb81f06612913a6d21dbc3aff80c6c48d8ep-25")),
(Wide!("4b.7885cbe80509531697fce770d1485a95b472b1f"),Wide!("1.8481145c6af8f70e0c306a15da219173961c299p-27")),
(Wide!("51.6d785763ab44ad58440fa9b8345d2e718e3ea53"),Wide!("1.112d07b0407a95834c1249c7656d47824ea1842p-29")),
(Wide!("57.d8bc5b5f724665ffc53c4af5b388ddec6d74460"),Wide!("7.9a724a7dbde95f1253ae2d29db99a64cf7575dep-32")),
(Wide!("5e.cec887c2d466096aa4d690337c71aa4b6e0c38f"),Wide!("1.fbc552ab48068052792b5321de6c5f67035dbaep-34")),
(Wide!("66.6c7ca33cf7b129298d3d3689979a43279d0cc39"),Wide!("4.586741d5d1299968c7dcd8b7695cf2421d79610p-37")),
(Wide!("6e.dd3c587ae8025ce3dd7f0f14de48c6cbc8a2878"),Wide!("4.3d2074257f909cea863be83a2dbab0cae9a5a6fp-40")),
(Wide!("78.688ae7ebbcaddd6f8047e67a17bc2e2d26da766"),Wide!("1.67e7236b61d00240f7b89b4503aa532ea5d72b2p-43")),
(Wide!("83.975afc69a97ed29f4b86f80258943c4c3141e3b"),Wide!("1.84fd999971cd1e553c0f0f341b633440060136ep-47")),
(Wide!("91.ca86297cf7149c28ca594277597dd4d1788edb2"),Wide!("1.75472fe584ff0801849e1a64e3e665deb3c478ep-52")),
];
