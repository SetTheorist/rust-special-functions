## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_laguerre_scale (@var{n}, [@var{a}])
## Compute the scale-factor to normalize the $n$'th (generalized) Laguerre polynomial:
## $L^(\alpha)_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_laguerre_scale(n, a)
  if (nargin < 1) print_usage; endif
  if (any(!sf_is_nonnegint(n))) print_usage; endif
  if (nargin < 2)
    res = ones(size(n));
  else
    res = sf_sqrt(sf_factorial(n) ./ sf_gamma(n+a+1));
  endif
endfunction
