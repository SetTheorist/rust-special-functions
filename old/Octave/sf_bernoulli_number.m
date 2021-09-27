## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bernoulli_number (@var{n})
## Compute Bernoulli number
## @end deftypefn

function res = sf_bernoulli_number(n)
  if (nargin < 1)
    print_usage;
  endif
  res = ones(size(n));
  for k = 1:prod(size(n))
    res(k) = sf_bernoulli_number_1(n(k));
  endfor
endfunction

function res = sf_bernoulli_number_1(n)
  persistent cache = [1.0/6.0];
  if (n<0 || n!=fix(n)) res = nan; return; endif
  if (n==0) res = 1.0; return; endif
  if (n==1) res = -1.0/2.0; return; endif
  if (rem(n,2)!=0) res = 0.0; return; endif
  if (2*length(cache) >= n)
    if (cache(n/2) != 0.0)
      res = cache(n/2);
      return;
    endif
  endif
  res = 2 * (2*pi)^(-n) * factorial(n) * sf_zeta(n) * (-1)^(1 + n/2);
  if (n<=1000)
    cache(n/2) = res;
  endif
endfunction
