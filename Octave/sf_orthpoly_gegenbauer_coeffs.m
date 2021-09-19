## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_gegenbauer_coeffs (@var{n}, @var{alpha})
## @deftypefnx {Function File} {@var{res} =} sf_orthpoly_gegenbauer_coeffs (@var{n}, @var{alpha}, [], @var{k})
## Compute the coefficients of the $n$'th Gegenbauer (ultraspherical) polynomial
## (or its $k$'th derivative):
## $C^(\alpha)_n(z) = a_1 + a_2*x + ... + a_(n+1)*x^n$,
## $n=0, 1, 2, ...$, $\alpha>-1/2, \alpha!=0$
## @end deftypefn

function res = sf_orthpoly_gegenbauer_coeffs(n, a, dum, k)
  if (nargin < 2) print_usage; endif
  if (!sf_is_nonnegint(n) || !(a>-1/2) || a==0) print_usage; endif
  if (nargin>3)
    if (!sf_is_nonnegint(k)) print_usage; endif
    if (k==0)
      res = sf_orthpoly_gegenbauer_coeffs(n, a);
    elseif (k>n)
      res = [0];
    else
      res = 2*a*sf_orthpoly_gegenbauer_coeffs(n-1, a+1, [], k-1);
    endif
    return;
  endif
  switch (n)
  case 0
    res = [1];
  case 1
    res = [0,2*a];
  otherwise
    persistent cache = {};
    if (n<=length(cache) && !isempty(cache{n}))
      res = cache{n};
      return;
    endif
    rm1 = zeros(1,n+1); rm1(1) = 1;
    rm0 = zeros(1,n+1); rm0(2) = 2*a;
    for k=2:n
      rm2 = rm1;
      rm1 = rm0;
      rm0 = (2*(k+a-1)*shift(rm1,1) - (k+2*a-2)*rm2)/k;
    endfor
    res = rm0;
    if (n<1000) cache{n} = res; endif
  endswitch
endfunction
