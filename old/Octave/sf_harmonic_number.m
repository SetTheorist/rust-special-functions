## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_harmonic_number (@var{n})
## Compute the harmonic number $H_n = \sum_(k=1)^n 1/k$
## @end deftypefn

function res = sf_harmonic_number(n)
  if (nargin < 1)
    print_usage;
  endif
  res = ones(size(n));
  for k = 1:prod(size(n))
    res(k) = sf_harmonic_1(n(k));
  endfor
endfunction
function res = sf_harmonic_1(n)
  persistent cache = [1, 1+1/2];
  if (imag(n)!=0 || n!=fix(n) || n<0) res = NaN; return; endif
  if (n==0) res = 0.0; return; endif
  if (n<=length(cache) && cache(n)!=0.0) res = cache(n); return; endif
  if (n<1111)
    res = sum(1.0 ./ (1:n));
    cache(n) = res;
  else
    # asymptotic approximation - gives same result as summation for n>=1111
    res = sf_log(n) + 0.57721566490153286061 + 1/(2*n) - 1.0/(12*n^2) + 1.0/(120*n^4);
  endif
endfunction
