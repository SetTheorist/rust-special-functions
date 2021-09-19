## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_solve_linear (@var{a}, @var{b})
## Solve the equation $a + bx = 0$
## @end deftypefn
function res = sf_solve_linear(a, b)
  if (nargin < 2)
    print_usage;
  endif
  res = -a/b;
endfunction
