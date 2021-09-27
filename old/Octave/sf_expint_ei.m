## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_expint_ei (@var{z})
## Compute exponential integral $\O{Ei}(z)$
## @end deftypefn

function res = sf_expint_ei(z)
  if (nargin < 1) print_usage; endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    res(kk) = sf_expint_ei__1(z(kk));
  endfor
endfunction

function res = sf_expint_ei__1(z)
  if (z < 0.0) res = nan; return; endif
  if (z == 0.0) res = -inf; return; endif
  if (z < 40.0)
    # series expansion
    term = 1.0;
    res = 0.0;
    n = 1;
    do
      term *= z/n;
      old_res = res;
      res += term/n;
      ++n; if (n>999) break; endif
    until (res == old_res)
    if (z < 0.5)
      res = sf_log(z * exp(0.57721566490153286061 + res));
    else
      res += sf_log(z) + 0.57721566490153286061;
    endif
  else
    # asymptotic expansion
    term = 1.0;
    res = 1.0;
    n = 1;
    do
      old_term = term;
      term *= n / z;
      if (term > old_term) break; endif
      old_res = res;
      res += term;
      ++n; if (n>999) break; endif
    until (res == old_res)
    res *= exp(z)/z;
  endif
endfunction
