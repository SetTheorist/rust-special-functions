## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_factorial_2 (@var{n})
## Compute double factorial $n!! = 1*3*5*...n$
## @end deftypefn

function res = sf_factorial_2(n)
  if (nargin<1) print_usage; endif
  res = zeros(size(n));
  for k=1:prod(size(n))
    res(k) = sf_factorial_2_1(n(k));
  endfor
endfunction
function res = sf_factorial_2_1(n)
  if (n<0) res = Inf; return; endif
  res = round(sf_gamma(fix(n)+1) / (2^(fix(n/2)) * sf_gamma(fix(n/2)+1)));
endfunction
