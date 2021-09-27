## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_hermite_he_zeros (@var{n})
## Compute the zeros of the $n$'th Hermite polynomial: $He_n(z)$
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_hermite_he_zeros(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  res = sf_sqrt(2)*sf_orthpoly_hermite_h_zeros(n);
endfunction
