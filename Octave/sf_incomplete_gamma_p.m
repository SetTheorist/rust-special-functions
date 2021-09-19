## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_incomplete_gamma_p (@var{a}, @var{x})
## Compute scaled lower incomplete gamma function -- $P(a,x) = 1/Gamma(a) \int_0^x ...$
## -- wrapper for now
## @end deftypefn

function res = sf_incomplete_gamma_p(a, x)
  if (nargin<2) print_usage; endif
  res = gammainc(x, a, "lower");
endfunction
