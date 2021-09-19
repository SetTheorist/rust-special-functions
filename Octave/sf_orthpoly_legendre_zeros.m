## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_legendre_zeros (@var{n})
## Compute the zeros of the $n$'th Legendre polynomial: $L_n(z)$
## $n=0, 1, 2, ...$
## not good for large
## @end deftypefn

function res = sf_orthpoly_legendre_zeros(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (n==0)
    res = [];
  elseif (n==1)
    res = [0];
  else
    persistent cache = {};
    if (n<=length(cache) && !isempty(cache{n}))
      res = cache{n};
      return;
    endif
    m = zeros(n);
    for k=1:n-1
      m(k,k+1) = m(k+1,k) = k/sqrt(4*k^2-1);
    endfor
    res = sort(eig(m));

    # "polish" the results
    fx = sf_orthpoly_legendre_value(n, res);
    dfx = (-n*res.*fx + n*sf_orthpoly_legendre_value(n-1, res)) ./ (1-res.^2);
    nwt = res - fx./dfx;
    #nrs = sf_orthpoly_legendre_value(n,nwt)
    res = nwt;
    # "polish" the results
    fx = sf_orthpoly_legendre_value(n, res);
    dfx = (-n*res.*fx + n*sf_orthpoly_legendre_value(n-1, res)) ./ (1-res.^2);
    nwt = res - fx./dfx;
    #nrs = sf_orthpoly_legendre_value(n,nwt)
    res = nwt;
    if (n<1000) cache{n} = res; endif
  endif
endfunction
