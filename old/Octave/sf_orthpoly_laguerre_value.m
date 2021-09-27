## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_laguerre_value (@var{n}, @var{z})
## @deftypefnx {Function File} {@var{res} =} sf_orthpoly_laguerre_value (@var{n}, @var{a}, @var{z})
## @deftypefnx {Function File} {@var{res} =} sf_orthpoly_laguerre_value (@var{n}, @var{z}, [], @var{k})
## @deftypefnx {Function File} {@var{res} =} sf_orthpoly_laguerre_value (@var{n}, @var{a}, @var{z}, [], @var{k})
## Compute the value of the $n$'th (generalized) Laguerre polynomial: $L^(\alpha)_n(z)$,
## (or its $k$'th derivative),
## $n=0, 1, 2, ...$, typically $z\in[0,\infty]$
## @end deftypefn

function res = sf_orthpoly_laguerre_value(n, a, z, dum, k)
  if (nargin < 2) print_usage; endif
  if (nargin < 3) z=a; a=0; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (nargin>3)
    if ((nargin==4 && !isempty(z)) || (nargin==5 && !isempty(dum))) print_usage; endif
    if (nargin==4) z=a; a=0; k=dum; endif
    if (k>n)
      res = zeros(size(z));
    else
      res = (-1)^(rem(k,2)) * sf_orthpoly_laguerre_value(n-k, a+k, z);
    endif
    return;
  endif
  switch (n)
  case 0
    res = ones(size(z));
  case 1
    res = 1 + a - z;
  otherwise
    rm1 = ones(size(z));
    rm0 = 1 + a - z;
    for k=2:n
      rm2 = rm1;
      rm1 = rm0;
      rm0 = ((2*k+a-1-z).*rm1 - (k+a-1)*rm2)/k;
    endfor
    res = rm0;
  endswitch
endfunction

%!test assert( all(sf_orthpoly_laguerre_value(0,0:20)==sf_polynomial_value(sf_orthpoly_laguerre_coeffs(0),0:20)) )
%!test assert( all(sf_orthpoly_laguerre_value(1,0:20)==sf_polynomial_value(sf_orthpoly_laguerre_coeffs(1),0:20)) )
%!test assert( all(abs(1 - sf_orthpoly_laguerre_value(4,0:20)./sf_polynomial_value(sf_orthpoly_laguerre_coeffs(4),0:20))<1e-12) )
%!test assert( all(abs(1 - sf_orthpoly_laguerre_value(11,0:20)./sf_polynomial_value(sf_orthpoly_laguerre_coeffs(11),0:20))<1e-10) )
%!test assert( all(abs(1 - sf_orthpoly_laguerre_value(12,0:20)./sf_polynomial_value(sf_orthpoly_laguerre_coeffs(12),0:20))<1e-8) )

%!test assert( all(sf_orthpoly_laguerre_value(0,2,0:20)==sf_polynomial_value(sf_orthpoly_laguerre_coeffs(0,2),0:20)) )
%!test assert( all(sf_orthpoly_laguerre_value(1,2,0:20)==sf_polynomial_value(sf_orthpoly_laguerre_coeffs(1,2),0:20)) )
%!test assert( all(abs(1 - sf_orthpoly_laguerre_value(4,2,0:20)./sf_polynomial_value(sf_orthpoly_laguerre_coeffs(4,2),0:20))<1e-12) )
%!test assert( all(abs(1 - sf_orthpoly_laguerre_value(11,2,0:20)./sf_polynomial_value(sf_orthpoly_laguerre_coeffs(11,2),0:20))<1e-8) )
%!test assert( all(abs(1 - sf_orthpoly_laguerre_value(12,2,0:20)./sf_polynomial_value(sf_orthpoly_laguerre_coeffs(12,2),0:20))<1e-8) )
