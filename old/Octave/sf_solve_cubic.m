## -*- texinfo -*-
## @deftypefn {Function File} {[@var{r1},@var{r2},@var{r3}] =} sf_solve_cubic (@var{a}, @var{b}, @var{c}, @var{d})
## Solve the equation $dx^3 + cx^2 + bx + a = 0$
## currently uses the trigonometric approach
## @end deftypefn
function [r1,r2,r3] = sf_solve_cubic(a, b, c, d)
  if (nargin < 4)
    print_usage;
  endif
  if (d == 0)
    [r1,r2] = sf_solve_quadratic(a, b, c);
  endif

  # TODO: make robust!
  #   fails, for example, on x^3 + 3x^2 + 3x + 1 = (x+1)^3
  #   (get division-by-zero and NaN result)

  # get equivalent  "depressed" cubic:
  #   t^3 + pt + q = 0
  p = (3*d*b - c^2) / (3*d^2);
  q = (2*c^3 - 9*d*c*b + 27*d^2*a) / (27*d^3);

  # trigonometric approach
  t = sqrt(-4*p/3);
  alpha = acos(-4*q/(t^3));
  r1 = t*cos(alpha/3);
  r2 = t*cos(alpha/3 + 2*pi/3);
  r3 = t*cos(alpha/3 + 4*pi/3);
  r1 -= c/(3*d);
  r2 -= c/(3*d);
  r3 -= c/(3*d);
endfunction
