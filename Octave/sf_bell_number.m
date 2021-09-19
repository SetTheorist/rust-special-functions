## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bell_number (@var{n})
## Compute Bell number
## @end deftypefn

function res = sf_bell_number(n)
  if (nargin < 1) print_usage; endif
  res = ones(size(n));
  for k = 1:prod(size(n))
    res(k) = bell_1(n(k));
  endfor
endfunction

function res = bell_1(n)
  persistent cache = [1]
  if (!sf_is_nonnegint(n)) res = nan; return; endif
  if (n==0 || n==1) res = 1; return; endif
  if (length(cache) >= n) && (cache(n)!=0) res = cache(n); return; endif
  if (n>300) res = inf; return; endif

  res = 0.0;
  for k = 0:(n-1)
    res += bincoeff(n-1,k)*bell_1(k);
  endfor

  cache(n) = res;
endfunction

# meh
function res = asympt(n)
  # asymptotic
  lam = sf_exp(sf_lambert_w(n));
  res = lam^(n+1/2)/sf_sqrt(n) * sf_exp(lam - n - 1);
endfunction
