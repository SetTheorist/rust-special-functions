Random jottings:

cargo rustdoc --open -- --html-in-header katex.html

## optional config settings
- use builtin math-functions (default true)
- use libm where available (default false)
- use num-complex, etc. (default false)

## Implementation levels

**Precision:**
- unimplemented ~0 digits
- low ~3 digits
- single rough ~6 digits
- single full ~single-precision
- single correct ~single-precision with correct rounding
- double rough ~14 digits
- double full ~double-precision
- double correct ~double-precision with correct rounding
- high ~30 digits (e.g. double-double or quadruple-precision)
- perfect ~full double-double or quadruple-precision with correct rounding
- arbitrary ~arbitrary precision (requires high-precision floats)

**Coverage:**

**Performance:**
(use, say built-in exp() as unit of performance, or maybe fp +/*?)
- ultra ~ 1
- fast ~ <10
- med ~ <100
- slow ~ <1000
- bad ~ 10000 or more

ldexp: ...  Guard/Round/Sticky ...


## Outline
- Basic, Exp, Log, Numbers, Classify, Constants
- Trig
- Gamma, digamma, polygamma, Beta
- Dilog, Polylog
- Erf, Fresnel (C,S), Faddeeva, Dawson
- Zeta, Hurwitz, Lerch Phi
- Incomplete Gamma
- Bessel, Airy, Kelvin, Struve, Anger, Weber
- OrthoPoly
- ExpInt, LogInt, CosInt, SinInt
- Probability distributions (etc.)
- Theta; Elliptic, beta, eta, lambda; Jacobian elliptic
- PCF
- Spheroidal wafe
- Hypergeometrics, confluent, G
- Lambert
- Misc.
  - Debye
  - Sievert
  - Spence
  - Lommel
  - Abramowitz 1 & 2
  - Bose-Einstein
  - Coulomb Wafe
  - Legendre P,Q
  - Scorer gi, hi
  - Voight u,v
  -
  - Mathieu
  - Lame
  - Heun
  - Painleve'
  - Coulomb
- Math finance: option pricing, etc.

**Integration**
- trapezoidal
- Erf, tanh/sinh, etc.
- Simpson's &c.
- Gaussian &c.
- ?Adaptive? - probably unnecessary for spec.func. impls?


## Misc. notes, jottings, meanderings, todos, ideas
- make literate source - generate .rs files with make (outside of cargo system)
- make Complex a generic struct over RealValue, eliminate ComplexType, ComplexValue entirely?!
- make a fully generic Wide type (V -> Wide<V> or Complex<V> -> Complex<Wide<V>>)
- add Wide associated type to Value (for easy expansion to extra precision)
- cf. micromath crate; provide ultra-fast, low-precision implementations? (e.g. f16 impls or "fast f64" - fast approx but low prec)
  - e.g. 2/2-minimax pade on (0,1) is >6x faster than exp()
  - cos(x) on [-pi/4,pi/4] with >4.5 digs precision, given by (1-0.04130114755x^2)/(1+0.08747300145x^2) with 2mul,1div,2 adds
- utility to give 2^n float (bit twiddling)
- clean-up complex implementations & document (esp. inverse trigonometric functions at branch-cuts)
- generic high-quality range-reduction
- cleanup and complete f16 basic implementations (+,-,*,/,%,sqrt,sqr,ldexp,input/output,exp,log,expm1,log1p,gamma,lngamma,sin,cos,tan,asin,acos,atan,zeta,...)
- cleanup bernoulli number implementations
- cf fungrim.org - online function repository (though doesn't seem to be maintained)
- compare FpCategory rust enum for float traits, etc.
- make custom svg plotting library (replace plotlib - too many dependencies) - maybe use svg crate or maybe just do my own;
    2d-charts (function plot, error plots, performance plots); 2d area charts/accuracy/region charts
- sample test-case, e.g. regularized beta function I_x(a,b) for x=4999/10000, a=b=10^5 (should be ~0.464365...)
- blog by Frederik Johansson on special functions & implementations
- check bessel function implementations, e.g. J_0 at ~15 with complex arguments (discontinuity in imaginary part?)
- fix exp() for complex numbers
- check Debye functions
- separate performance testing (criterion crate) into completely separated crate (to eliminate a lot of dependencies)
- separate plotting / graphing into a separate crate also
- nalgebra/SymmetricEigen

**Structure**
- theory
  - definitions
  - equations/representations/relationships
  - special values
  - graphics
- code
  - methods
  - implementations
    - real
    - error
    - complex
- tests
  - basic (sanity testing values)
  - relations
  - specific numbers
- error
  - systematic error
  - different domains
  - charts / stats
- performance
  - systematic
  - different domains
  - charts / stats
- (use same ranges & values for error & performance)
- automate the generation of these