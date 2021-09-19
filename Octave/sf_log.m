## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_log (@var{z})
## Compute natural logarithm $\ln(z)$
## @end deftypefn

function res = sf_log(z)
  res = ones(size(z));
  for n = 1:prod(size(z))
    res(n) = sf_log_1(z(n));
  endfor
endfunction

function res = sf_log_1(z)
  if (isnan(z)) res = z; return; endif
  if (z==0) res = -Inf; return; endif
  if (z==Inf) res = Inf; return; endif
  th = arg(z);
  r = abs(z);
  # extract exponent/mantissa
  # 1/2 <= r < 1
  # we move r to range for slightly faster convergence
  # 2/3 <= r < 4/3
  #
  # Note that alternative reduction approach would subtract
  # off multiples of log(2) ... (need high-precision)
  [r, pow2] = log2(r);
  if (r<2/3) r*=2; --pow2; endif
  res = pow2*0.69314718055994529 + series(r) + I*th;
endfunction

# ln(z) = 2\sum_{n=0}^\infty (z-1 / z+1)^(2n+1) / 2n+1
# much faster convergence than standard series below
function res = series(z)
  res = term = (z-1) / (z+1);
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
  #NN = 24;
  #q = (z-1) / (z+1);
  #r2 = 2*q*sum([q.^(2*(0:NN)) ./ (1+2*(0:NN))], 'extra')
endfunction

# ln(1+z) = z - z^2/2 + z^3/3 ...
function res = series_p1(z)
  res = z;
  n = 2;
  zn = z;
  do
    zn *= -z;
    old_res = res;
    res += zn/n;
    ++n;
    if (n>999) break; endif
  until (res == old_res);
endfunction

