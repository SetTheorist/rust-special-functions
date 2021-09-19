## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_incomplete_gamma (@var{a}, @var{x})
## Compute lower incomplete Gamma function -- $\gamma(a,x) = \int_0^x ...$
## -- wrapper for now
## @end deftypefn

function res = sf_incomplete_gamma(a, x)
  if (nargin<2) print_usage; endif
  res = gammainc(x, a, "lower")*sf_gamma(a);
endfunction

