## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_chebyshev_u_scale (@var{n})
## Compute the scale-factor to normalize the $n$'th Chebyshev polynomial of the second kind, $U_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_chebyshev_u_scale(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  persistent qp2 = sf_sqrt(2/pi);
  res = qp2*ones(size(n));
endfunction
