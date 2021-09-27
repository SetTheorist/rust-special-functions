## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_polynomial_value (@var{a}, @var{z})
## Compute the value of the polynomial $a_1 + a_2*z + ... + a_n*z^(n-1)$
## @end deftypefn
function res = sf_polynomial_value(a, z)
  if (nargin < 2) print_usage; endif
  n = length(a);
  pows = 0:(n-1);
  res = ones(size(z));
  for k = 1:prod(size(z))
    res(k) = sum(a .* (z(k) .^ pows));
  endfor
endfunction
