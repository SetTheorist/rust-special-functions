## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_hermite_h_scale (@var{n})
## Compute the scale-factor to normalize the $n$'th Hermite polynomial, $H_n(z)$,
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_hermite_h_scale(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  res = 1 ./ sf_sqrt(2.^n .* sf_sqrt(pi) .* sf_factorial(n));
endfunction
