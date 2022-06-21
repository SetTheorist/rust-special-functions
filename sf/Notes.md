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



