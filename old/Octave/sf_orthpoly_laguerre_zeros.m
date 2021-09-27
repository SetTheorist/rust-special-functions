## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_laguerre_zeros (@var{n}, [@var{a}])
## @deftypefnx {Function File} {@var{res} =} sf_orthpoly_laguerre_zeros (@var{n}, [@var{a}], [], @var{k})
## Compute the zeros of the $n$'th (generalized) Laguerre polynomial: $L^(\alpha)_n(z)$
## (or its $k$'th derivative)
## $n=0, 1, 2, ...$
## not good for large
## @end deftypefn

function res = sf_orthpoly_laguerre_zeros(n, a, dum, k)
  if (nargin < 1) print_usage; endif
  if (nargin < 2) a = 0; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (nargin > 2)
    if ((nargin==3 && !isempty(a)) || (nargin==4 && !isempty(dum))) print_usage; endif
    if (nargin==3) a=0; k=dum; endif
    if (k>n)
      res = []; # degenerate case... technically every point is a zero...
    else
      res = sf_orthpoly_laguerre_zeros(n-k, a+k);
    endif
    return;
  endif
  if (n==0)
    res = [];
  elseif (n==1)
    res = [1+a];
  else
    m = zeros(n);
    for k=1:n-1
      m(k,k+1) = -k;
      m(k,k) = 2*k-1 + a;
      m(k+1,k) = -k - a;
    endfor
    m(n,n) = 2*n-1 + a;
    res = sort(eig(m));

    # "polish" the results
    #fx = sf_orthpoly_laguerre_value(n, a, res)
    #dfx = sf_orthpoly_laguerre_value(n, a, res, [], 1);
    #nwt = res - fx./dfx;
    #nrs = sf_orthpoly_laguerre_value(n,nwt)
    #res = nwt;
  endif
endfunction
