## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_legendre_scale (@var{n})
## Compute the scale-factor to normalize the $n$'th Legendre polynomial:
## $L_n(z) = a_1 + a_2*x + ... + a_(n+1)*x^n$,
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_legendre_scale(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  res = sf_sqrt((2*n+1)/2);
endfunction
