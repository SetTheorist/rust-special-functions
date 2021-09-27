## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_laguerre_coeffs (@var{n}, [@var{a}])
## @deftypefnx {Function File} {@var{res} =} sf_orthpoly_laguerre_coeffs (@var{n}, [@var{a}], [], @var{k})
## Compute the coefficients of the $n$'th (generalized) Laguerre polynomial:
## $L^(\alpha)_n(z) = a_1 + a_2*x + ... + a_(n+1)*x^n$,
## (or its $k$'th derivative)
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_laguerre_coeffs(n, a, dum, k)
  if (nargin < 1) print_usage; endif
  if (nargin < 2) a = 0; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (nargin > 2)
    if ((nargin==3 && !isempty(a)) || (nargin==4 && !isempty(dum))) print_usage; endif
    if (nargin==3) a=0; k=dum; endif
    if (k>n)
      res = [0];
    else
      res = (-1)^(rem(k,2)) * sf_orthpoly_laguerre_coeffs(n-k, a+k);
    endif
    return;
  endif
  switch (n)
  case 0
    res = [1];
  case 1
    res = [1+a,-1];
  otherwise
    persistent cache = {};
    if (n<=length(cache) && !isempty(cache{n}))
      res = cache{n};
      return;
    endif
    rm1 = zeros(1,n+1); rm1(1) = 1;
    rm0 = zeros(1,n+1); rm0(1) = 1+a; rm0(2) = -1;
    for k=2:n
      rm2 = rm1;
      rm1 = rm0;
      rm0 = ((2*k+a-1)*rm1 - shift(rm1,1) - (k+a-1)*rm2) / k;
    endfor
    res = rm0;
    if (n<1000) cache{n} = res; endif
  endswitch
endfunction
