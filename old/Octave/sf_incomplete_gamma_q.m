## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_incomplete_gamma_q (@var{a}, @var{x})
## Compute scaled upper incomplete Gamma function -- $Q(a,x) = 1/Gamma(a) \int_x^\infty ...$
## -- wrapper for now
## @end deftypefn

function res = sf_incomplete_gamma_q(a, x)
  if (nargin<2) print_usage; endif
  res = gammainc(x, a, "upper");
endfunction
