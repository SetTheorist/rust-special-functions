## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_gegenbauer_value (@var{n}, @var{alpha}, @var{z})
## @deftypefnx {Function File} {@var{res} =} sf_orthpoly_gegenbauer_value (@var{n}, @var{alpha}, @var{z}, [], @var{k})
## Compute the value of the $n$'th Gegenbauer (ultraspherical) polynomial: $C^(\alpha)_n(z)$,
## (or its $k$'th derivative)
## $n=0, 1, 2, ...$, $\alpha>-1/2, \alpha!=0$, typically $z\in[-1,1]$
## @end deftypefn

function res = sf_orthpoly_gegenbauer_value(n, a, z, dum, k)
  if (nargin < 3) print_usage; endif
  if (!sf_is_nonnegint(n) || !(a>-1/2) || a==0) print_usage; endif
  if (nargin>3)
    if (!sf_is_nonnegint(k)) print_usage; endif
    if (k==0)
      res = sf_orthpoly_gegenbauer_value(n, a, z);
    elseif (k>n)
      res = zeros(size(z));
    else
      res = 2*a*sf_orthpoly_gegenbauer_value(n-1, a+1, z, [], k-1);
    endif
    return;
  endif
  switch (n)
  case 0
    res = ones(size(z));
  case 1
    res = 2*a*z;
  otherwise
    rm1 = ones(size(z));
    rm0 = 2*a*z;
    for k=2:n
      rm2 = rm1;
      rm1 = rm0;
      rm0 = (2*z*(k+a-1).*rm1 - (k+2*a-2)*rm2)/k;
    endfor
    res = rm0;
  endswitch
endfunction
