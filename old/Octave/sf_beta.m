## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_beta (@var{a}, @var{b})
## Compute Beta function B(a,b)
## @end deftypefn

function res = sf_beta(a, b)
  if (nargin < 2) print_usage; endif
  res = sf_exp(sf_ln_gamma(a) + sf_ln_gamma(b) - sf_ln_gamma(a+b));
endfunction


%!test assert(sf_beta(3,3), prod(1:2)*prod(1:2)/prod(1:5), -5e-15);
%!test assert(sf_beta(2,7), prod(1:1)*prod(1:6)/prod(1:8), -5e-15);
%!test assert(sf_beta(7,13), sf_beta(13,7));
