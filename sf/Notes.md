Random jottings:

## optional config settings
- use builtin math-functions (default true)
- use libm where available (default false)
- use num-complex, etc. (default false)

## Implementation levels

Precision:
-----
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

Coverage:
-----

Performance:
-----
(use, say built-in exp() as unit of performance, or maybe fp +/*?)
- ultra ~ 1
- fast ~ <10
- med ~ <100
- slow ~ <1000
- bad ~ 10000 or more


