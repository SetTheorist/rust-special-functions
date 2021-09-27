## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_chebyshev_t_zeros (@var{n})
## Compute the zeros of the $n$'th Chebyshev polynomial of the first kind: $T_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_chebyshev_t_zeros(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  if (n==0)
    res = [];
  else
    res = sf_cos((pi/2) * (2*(n:(-1):1) - 1)/n);
    if (sf_is_oddint(n)) res((n+1)/2) = 0; endif
  endif
endfunction
