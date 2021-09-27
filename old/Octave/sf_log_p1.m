## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_log_p1 (@var{z})
## Compute logarithm of z+1, $\ln(1+z)$
## @end deftypefn

function res = sf_log_p1(z)
  #the system routine actually seems to have a BUG!
  #when z=I*.1, for example, we get a bogus answer...
  #sys = log1p(z);
  #esy= exp(sys);
  res = ones(size(z));
  for n = 1:prod(size(z))
    res(n) = sf_log_p1_1(z(n));
  endfor
endfunction

function res = sf_log_p1_1(z)
  if (isnan(z)) res = z; return; endif
  if (abs(z) > 0.25) res = sf_log(z + 1); return; endif
  res = seriesx(z);
endfunction

# ln(1+x) = 2\sum_{n=0}^\infty (x / x+2)^(2n+1) / 2n+1
function res = seriesx(z)
  res = term = z / (z+2);
  zr2 = term*term;
  n = 1;
  do
    term *= zr2;
    old_res = res;
    res += term/(2*n+1);
    ++n;
    if (n>999) break; endif
  until (res == old_res)
  res *= 2;
endfunction
