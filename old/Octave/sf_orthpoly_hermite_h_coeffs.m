## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_hermite_h_coeffs (@var{n})
## @deftypefnx {Function File} {@var{res} =} sf_orthpoly_hermite_h_coeffs (@var{n}, [], @var{k})
## Compute the coefficients of the $n$'th Hermite polynomial
## (or its $k$'th derivative):
## $H_n(z) = a_1 + a_2*x + ... + a_(n+1)*x^n$,
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_hermite_h_coeffs(n, dum, k)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (nargin>1)
    if (!isempty(dum) || !sf_is_nonnegint(k)) print_usage; endif
    if (k==0)
      res = sf_orthpoly_hermite_h_coeffs(n);
    elseif (k>n)
      res = [0];
    else
      res = 2*n*sf_orthpoly_hermite_h_coeffs(n-1, [], k-1);
    endif
    return;
  endif
  switch (n)
  case 0
    res = [1];
  case 1
    res = [0,2];
  otherwise
    persistent cache = {};
    if (n<=length(cache) && !isempty(cache{n}))
      res = cache{n};
      return;
    endif
    rm1 = zeros(1,n+1); rm1(1) = 1;
    rm0 = zeros(1,n+1); rm0(2) = 2;
    for k=2:n
      rm2 = rm1;
      rm1 = rm0;
      rm0 = 2*shift(rm1,1) - 2*(k-1)*rm2;
    endfor
    res = rm0;
    if (n<1000) cache{n} = res; endif
  endswitch
endfunction
