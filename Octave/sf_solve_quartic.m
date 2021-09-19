## -*- texinfo -*-
## @deftypefn {Function File} {[@var{r1},@var{r2},@var{r3},@var{r4}] =} sf_solve_quartic (@var{a}, @var{b}, @var{c}, @var{d}, @var{e})
## Solve the equation $ex^4 + dx^3 + cx^2 + bx + a = 0$
## approach via factoring into quadratics
## @end deftypefn
function [r1,r2,r3,r4] = sf_solve_quartic(a, b, c, d, e)
  if (nargin < 5)
    print_usage;
  endif
  if (e == 0)
    [r1,r2,r3] = sf_solve_cubic(a, b, c, d);
  endif
  # TODO: make robust!
  # blows up for some cases, e.g. (x+1)^4 = x^4+4x^3+6x^2+4x+1

  # Approach from wikipedia "Quartic function" article

  # normalize
  d /= e;
  c /= e;
  b /= e;
  a /= e;
  e = 1.0;

  # transform to "depressed" quartic
  # x^4 + C x^2 + D x + E
  C = c - 3*d^2/8;
  D = d^3/8 - d*c/2 + b;
  E = a - 3*d^4/256 + d^2*c/16 - b*d/4;

  # solve resolvent cubic
  # P^3 + 2cP^2 (c^2-4e)P - d^2 = 0
  # (P=p^2)
  [P1,P2,P3] = sf_solve_cubic(-D^2, C^2-4*E, 2*C, 1);
  #err1 = P1^3 + (2*C)*P1^2 + (C^2-4*E)*P1 - D^2
  #err2 = P2^3 + (2*C)*P2^2 + (C^2-4*E)*P2 - D^2
  #err3 = P3^3 + (2*C)*P3^2 + (C^2-4*E)*P3 - D^2

  p = sqrt(P3);
  r = -p;
  s = (C + p^2 + D/p)/2;
  q = (C + p^2 - D/p)/2;

  # thus x^4+bx^3+cx^2+dx+e = (x^2+px+q)(x^2+rx+s)
  [r1,r2] = sf_solve_quadratic(q, p, 1);
  [r3,r4] = sf_solve_quadratic(s, r, 1);

  r1 -= d/4;
  r2 -= d/4;
  r3 -= d/4;
  r4 -= d/4;

endfunction
