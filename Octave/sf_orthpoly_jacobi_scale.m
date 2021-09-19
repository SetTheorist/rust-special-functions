## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_jacobi_scale (@var{n}, @var{a}, @var{b})
## Compute the scale-factor to normalize the $n$'th Jacobi polynomial:
## $J^(a,b)_n(z) = a_1 + a_2*x + ... + a_(n+1)*x^n$,
## $a>-1$, $b>-1$, $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_jacobi_scale(n, a, b)
  if (nargin < 3) print_usage; endif
  if (any(!sf_is_nonnegint(n)) || a<-1 || b<-1) print_usage; endif
  res = sf_sqrt( (2*n+a+b+1)./2^(a+b+1) .* sf_gamma(n+1)./sf_gamma(n+a+1) .* sf_gamma(n+a+b+1)./sf_gamma(n+b+1) );
endfunction
