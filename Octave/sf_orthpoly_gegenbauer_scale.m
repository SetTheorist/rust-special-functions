## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_gegenbauer_value (@var{n}, @var{alpha})
## Compute the scale-factor to normalize the $n$'th Gegenbauer (ultraspherical) polynomial: $C^(\alpha)_n(z)$,
## $n=0, 1, 2, ...$, $\alpha>-1/2, \alpha!=0$
## @end deftypefn

function res = sf_orthpoly_gegenbauer_scale(n, a)
  if (nargin < 2) print_usage; endif
  if (!sf_is_nonnegint(n) || !(a>-1/2) || a==0) print_usage; endif
  res = sf_pochhammer(a+1/2,n).*sf_sqrt(2*(n+a).*sf_factorial(n).*sf_gamma(n+2*a))./sf_pochhammer(2*a,n)./2^a./sf_gamma(n+a+1/2);
endfunction
