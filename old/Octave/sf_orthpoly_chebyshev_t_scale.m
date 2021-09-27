## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_chebyshev_t_scale (@var{n})
## Compute the scale-factor to normalize the $n$'th Chebyshev polynomial of the first kind, $T_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_chebyshev_t_scale(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  persistent qp1 = sf_sqrt(1/pi);
  persistent qp2 = sf_sqrt(2/pi);
  res = qp2*ones(size(n));
  res(n==0) = qp1;
endfunction
