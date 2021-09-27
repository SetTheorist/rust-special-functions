## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_chebyshev_u_weights (@var{n})
## Compute the Gauss quadrature weights for the $n$'th Chebyshev polynomial of the second kind: $U_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_chebyshev_u_weights(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (n==0)
    res = [];
  else
    res = (pi/(n+1)) * sf_sin(pi*(1:n)/(n+1)).^2;
  endif
endfunction
