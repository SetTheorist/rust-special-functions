## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_ln_bernoulli_number (@var{n})
## Compute logarithm of Bernoulli number
## @end deftypefn

function res = sf_ln_bernoulli_number(n)
  if (nargin < 1)
    print_usage;
  endif
  res = ones(size(n));
  for k = 1:prod(size(n))
    res(k) = sf_ln_bernoulli_number_1(n(k));
  endfor
endfunction

function res = sf_ln_bernoulli_number_1(n)
  persistent cache = [-sf_log(6.0)];
  if (n<0 || n!=fix(n)) res = nan; return; endif
  if (n==0) res = 0.0; return; endif
  if (n==1) res = -sf_log(2.0) + pi*I; return; endif
  if (rem(n,2)!=0) res = -Inf; return; endif
  if (2*length(cache) >= n)
    if (cache(n/2) != 0.0)
      res = cache(n/2);
      return;
    endif
  endif
  res = sf_log(2) - n*sf_log(2*pi) + sf_ln_factorial(n) + sf_log_p1(sf_zeta_m1(n)) + (1-n/2)*pi*I;
  if (n<=1000)
    cache(n/2) = res;
  endif
endfunction
