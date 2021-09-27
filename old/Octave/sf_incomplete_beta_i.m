## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_incomplete_beta_i (@var{x}, @var{a}, @var{b})
## Compute normalized incomplete Beta function $B_x(a,b) / B(a,b)$
##
## @end deftypefn

function res = sf_incomplete_beta_i(x, a, b)
  if (nargin < 3) print_usage; endif
  res = sf_incomplete_beta(x,a,b) / sf_beta(a,b);
endfunction
