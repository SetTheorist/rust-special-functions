## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_hermite_h_weights (@var{n})
## Compute the Gaussian quadrature weights of the $n$'th Hermite polynomial: $H_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_hermite_h_weights(n)
  persistent cache = {}
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (n==0)
    res = [];
  elseif (n==1)
    res = [1];
  else
    if (n<=length(cache) && !isempty(cache{n})) res = cache{n}; return endif

    zs = sf_orthpoly_hermite_h_zeros(n);
    # need ortho_normal_ polynomials here
    nrm = sf_orthpoly_hermite_h_scale(0:(n-1));
    res = zeros(n, 1);
    for jj = 0:(n-1)
      res += (sf_orthpoly_hermite_h_value(jj, zs)*nrm(jj+1)).^2;
    endfor
    res = 1./res;
    cache{n} = res;
  endif
endfunction
