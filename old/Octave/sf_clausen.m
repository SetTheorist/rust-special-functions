## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_clausen (@var{theta})
## Compute the Clausen integral $C(\theta)$
## @end deftypefn
function res = sf_clausen(theta, opt)
  if (nargin < 1)
    print_usage;
  endif
  if (nargin < 2) opt = false; endif
  if (opt)
    res = -quad(@(t)(log(2*sin(t/2))), 0, theta, 1e-14);
  else
    if (theta<0 || theta>pi) res = nan; return; endif
    res = theta*(1-sf_log(theta));
    term = -theta;
    th2 = -theta^2;
    n = 1;
    do
      term *= th2;
      old_res = res;
      res += sf_bernoulli_number_scaled(2*n) * term / ((2*n)*(2*n+1));
      ++n;
      if (n>999) break; endif
    until (res == old_res);
  endif
endfunction
