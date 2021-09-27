## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_laguerre_weights (@var{n}, [@var{a}])
## Compute the Gaussian quadrature weights of the $n$'th (generalized) Laguerre polynomial: $L^(a)_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_laguerre_weights(n, a)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (nargin < 2) a=0; endif
  if (n==0)
    res = [];
  elseif (n==1)
    res = [1];
  else
    zs = sf_orthpoly_laguerre_zeros(n, a);
    # need ortho_normal_ polynomials here
    nrm = sf_orthpoly_laguerre_scale(0:(n-1), a);
    res = zeros(n, 1);
    for jj = 0:(n-1)
      res += (sf_orthpoly_laguerre_value(jj, a, zs)*nrm(jj+1)).^2;
    endfor
    res = 1./res;
  endif
endfunction
