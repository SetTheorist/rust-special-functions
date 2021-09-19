## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_chebyshev_t_weights (@var{n})
## Compute the Gauss quadrature weights for the $n$'th Chebyshev polynomial of the first kind: $T_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_chebyshev_t_weights(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (n==0)
    res = [];
  else
    res = (pi/n) * ones(1,n);
  endif
endfunction
