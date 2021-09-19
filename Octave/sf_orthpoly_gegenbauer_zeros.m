## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_gengenbauer_zeros (@var{n}, @var{alpha})
## Compute the zeros of the $n$'th Gegenbauer (ultraspherical) polynomial: $C^(\alpha)_n(z)$
## $n=0, 1, 2, ...$, $\alpha>-1/2, \alpha!=0$
## @end deftypefn

function res = sf_orthpoly_gegenbauer_zeros(n, a)
  if (nargin < 2) print_usage; endif
  if (!sf_is_nonnegint(n) || !(a>-1/2) || a==0) print_usage; endif
  if (n==0)
    res = [];
  elseif (n==1)
    res = [0];
  else
    m = zeros(n);
    m(1,1+1) = m(1+1,1) = sf_sqrt(1/(2+2*a));
    for k=2:n-1
      m(k,k+1) = m(k+1,k) ...
               = 2/(2*k+2*a-1) * sf_sqrt((k*(k+a-1/2)^2*(k+2*a-1))/((2*k+2*a)*(2*k+2*a-2)));
    endfor
    res = sort(eig(m));

    # "polish" the results
    fx = sf_orthpoly_gegenbauer_value(n, a, res);
    dfx = sf_orthpoly_gegenbauer_value(n, a, res, [], 1);
    nwt = res - fx./dfx;
    #nrs = sf_orthpoly_gegenbauer_value(n, a, nwt)
    res = nwt;
  endif
endfunction
