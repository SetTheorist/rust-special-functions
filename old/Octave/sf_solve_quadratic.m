## -*- texinfo -*-
## @deftypefn {Function File} {[@var{r1},@var{r2}] =} sf_solve_quadratic (@var{a}, @var{b}, @var{c})
## Solve the equation $a + bx + cx^2 = 0$
## @end deftypefn
function [r1,r2] = sf_solve_quadratic(a, b, c)
  if (nargin < 3)
    print_usage;
  endif
  if (c==0)
    r1 = sf_solve_linear(a, b);
    r2 = NaN;
    return;
  endif

  # TODO: make this robust!
  r1 = (-b + sqrt(b^2 - 4*a*c)) / (2*c);
  if (nargout>1)
    r2 = (-b - sqrt(b^2 - 4*a*c)) / (2*c);
  endif
endfunction
