## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_euler_number_scaled (@var{n})
## Compute scaled Euler number $E_n/n!$
## @end deftypefn

function res = sf_euler_number_scaled(n)
  if (nargin< 1) print_usage; endif
  res = ones(size(n));
  for k = 1:prod(size(n))
    res(k) = sf_euler_number_1(n(k));
  endfor
endfunction

function res = sf_euler_number_1(n)
  persistent cache = [-0.5];
  if (!sf_is_nonnegint(n)) res = NaN; return; endif
  if (n==0) res = 1.0; return; endif
  if (rem(n,2)!=0) res = 0.0; return; endif
  if (2*length(cache) >= n) && (cache(n/2) != 0.0) res = cache(n/2); return; endif
  res = (-1)^(n/2) * 2 * (2/pi)^(n+1) * sf_dirichlet_beta(n+1);
  if (n<=1000) cache(n/2) = res; endif
endfunction
